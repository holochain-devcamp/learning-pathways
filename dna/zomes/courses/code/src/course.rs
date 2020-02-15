/************************ Import Required Libraries */
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};

use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_core_types::{entry::Entry, validation::EntryValidationData};
use holochain_wasm_utils::api_serialization::{
    get_entry::{GetEntryOptions, GetEntryResult},
    get_links::GetLinksOptions,
};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::AddressableContent;
use hdk::prelude::LinkMatch;
use hdk::ValidationData;
use std::convert::TryFrom;
/******************************************* */

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Course {
    pub title: String,
    pub modules: Vec<Address>,
    pub timestamp: u64,
    pub teacher_address: Address,
}

impl Course {
    // Constrcuctor
    pub fn new(title: String, owner: Address, timestamp: u64) -> Self {
        Course {
            title: title,
            teacher_address: owner,
            modules: Vec::default(),
            timestamp: timestamp,
        }
    }
    pub fn from(title: String, owner: Address, timestamp: u64, modules: Vec<Address>) -> Self {
        Course {
            title: title,
            teacher_address: owner,
            modules: modules,
            timestamp: timestamp,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("course".into(), self.into())
    }
}

////////////////////Course Entry Definition
pub fn course_entry_def() -> ValidatingEntryType {
    entry!(
        name: "course",
        description: "this is the definition of course",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Course>| {
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                    if !validation_data.sources().contains(&entry.teacher_address) {
                        return Err(String::from("Only the teacher can create their courses"));
                    }

                    validate_course_title(&entry.title)
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    if new_entry.teacher_address != old_entry.teacher_address {
                        return Err(String::from("Cannot change the teacher of the course"));
                    }

                    if !validation_data.sources().contains(&old_entry.teacher_address) {
                        return Err(String::from("Only the teacher can modify their courses"));
                    }

                    validate_course_title(&new_entry.title)?;

                    Ok(())
                },
                EntryValidationData::Delete {old_entry, validation_data, .. } => {
                    if !validation_data.sources().contains(&old_entry.teacher_address) {
                        return Err(String::from("Only the teacher can delete their courses"));
                    }

                    Ok(())
                }
            }
        },
        links: [
          from!( // to query all the courses of a user(all courses that a user is the teacher or owner of)
              "%agent_id",
              link_type: "teacher->courses",
              validation_package: || {
                  hdk::ValidationPackageDefinition::Entry
              }              ,
              validation: | _validation_data: hdk::LinkValidationData | {
                // TODO: Homework. Implement validation rules if required.
                Ok(())
              }
          ),
          from!( // to query all courses that one user enrolled
            "%agent_id",
            link_type: "student->courses",
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            }              ,
            validation: | _validation_data: hdk::LinkValidationData | {
                // TODO: Homework. Implement validation rules if required.
               Ok(())
            }
        ),
        to!( // to query all enrolled user for a course
            "%agent_id",
            link_type: "course->students",
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::LinkValidationData | {
                // TODO: Homework. Implement validation rules if required.
                Ok(())
            }
        )
      ]
    )
}

//// Anchor Definition : This Anchor will be used to query all courses
pub fn anchor_entry_def() -> ValidatingEntryType {
    entry!(
        name: "anchor",
        description:"Anchor to all Courses",
        sharing: Sharing::Public,
        validation_package:||{
            hdk::ValidationPackageDefinition::Entry
        },
        validation:|_validation_data: hdk::EntryValidationData<String>|{
            Ok(())
        },
        links:[
            to!(
                "course",
                link_type: "course_list",
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                    Ok(())
                }
            )
        ]
    )
}

pub fn anchor_entry() -> Entry {
    Entry::App("anchor".into(), "course".into())
}

pub fn anchor_address() -> ZomeApiResult<Address> {
    hdk::entry_address(&anchor_entry())
}

/*********************** Course Validations */
fn validate_course_title(title: &str) -> Result<(), String> {
    if title.len() > 50 {
        Err("Course title is too long".into())
    } else {
        Ok(())
    }
}

/********************************************** */
/// Course Helper Functions: CRUD

pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    let anchor_entry = anchor_entry();
    let anchor_address = hdk::commit_entry(&anchor_entry)?; // if Anchor exist, it returns the commited one.

    let new_course = Course::new(title, AGENT_ADDRESS.to_string().into(), timestamp);
    let new_course_entry = new_course.entry();
    let new_course_address = hdk::commit_entry(&new_course_entry)?;

    hdk::link_entries(&AGENT_ADDRESS, &new_course_address, "teacher->courses", "")?;

    hdk::link_entries(&anchor_address, &new_course_address, "course_list", "")?;

    Ok(new_course_address)
}

pub fn update(
    title: String,
    modules_addresses: Vec<Address>,
    course_address: &Address,
) -> ZomeApiResult<Address> {
    let course: Course = hdk::utils::get_as_type(course_address.clone())?;

    let new_version_course = Course::from(
        title,
        course.teacher_address,
        course.timestamp,
        modules_addresses,
    );
    let new_version_course_entry = new_version_course.entry();

    hdk::update_entry(new_version_course_entry, course_address)
}

pub fn delete(address: Address) -> ZomeApiResult<Address> {
    hdk::remove_link(&anchor_address()?, &address, "course_list", "")?;

    let students = get_students(address.clone())?;
    let course: Course = hdk::utils::get_as_type(address.clone())?;

    for student in students {
        hdk::remove_link(&student, &address, "student->course", "")?;
    }
    hdk::remove_link(&course.teacher_address, &address, "teacher->courses", "")?;

    hdk::remove_entry(&address)
}

pub fn list() -> ZomeApiResult<Vec<Address>> {
    let addresses = hdk::get_links(
        &anchor_address()?,
        LinkMatch::Exactly("course_list"),
        LinkMatch::Any,
    )?
    .addresses();

    Ok(addresses)
}

pub fn get_my_courses() -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &AGENT_ADDRESS,
        LinkMatch::Exactly("teacher->courses"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

pub fn get_my_enrolled_courses() -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &AGENT_ADDRESS,
        LinkMatch::Exactly("student->courses"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

pub fn add_module_to_course(
    course_address: &Address,
    module_address: &Address,
) -> ZomeApiResult<Address> {
    let current_course = hdk::get_entry(course_address).unwrap().unwrap();
    if let Entry::App(_, current_course) = current_course {
        let mut course_entry = Course::try_from(current_course.clone())
            .expect("Entry at this address is not Course. You sent a wrong address");
        course_entry.modules.push(module_address.clone());
        hdk::update_entry(
            Entry::App("course".into(), course_entry.into()),
            course_address,
        )
    } else {
        panic!("This address is not a valid address")
    }
}

pub fn enrol_in_course(course_address: Address) -> ZomeApiResult<Address> {
    hdk::link_entries(&AGENT_ADDRESS, &course_address, "student->courses", "")?;
    hdk::link_entries(&course_address, &AGENT_ADDRESS, "course->students", "")
}

pub fn get_students(course_address: Address) -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &course_address,
        LinkMatch::Exactly("course->students"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}
