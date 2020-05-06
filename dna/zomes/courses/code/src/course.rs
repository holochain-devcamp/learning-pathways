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
pub struct CourseData {
    // This struct's instances would be updated if we need to change anything about the course
    pub title: String,
    pub modules: Vec<Address>,
    pub timestamp: u64,
    pub teacher_address: Address,
    // Store reference to the corresponding CourseAnchor to save resources on lookup
    pub course_anchor: Address,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct CourseAnchor {
    // This struct's instances would be constant and responsible for holding links for the course from other entries
    pub timestamp: u64,
    pub teacher_address: Address,
}

impl CourseData {
    // Constructor that creates an emptry CourseData without any modules
    pub fn new(title: String, owner: Address, course_anchor: Address, timestamp: u64) -> Self {
        CourseData {
            title: title,
            modules: Vec::default(),
            timestamp: timestamp,
            teacher_address: owner,
            course_anchor: course_anchor,
        }
    }

    // Constructor that creates a filled-in CourseData instance with modules
    pub fn from(
        title: String,
        owner: Address,
        course_anchor: Address,
        timestamp: u64,
        modules: Vec<Address>,
    ) -> Self {
        CourseData {
            title: title,
            modules: modules,
            timestamp: timestamp,
            teacher_address: owner,
            course_anchor: course_anchor,
        }
    }

    // Copy constructor that saves us some typing effort during the update operations
    pub fn from_instance(another_course_data: CourseData) -> Self {
        CourseData {
            title: another_course_data.title,
            modules: another_course_data.modules,
            timestamp: another_course_data.timestamp,
            teacher_address: another_course_data.teacher_address,
            course_anchor: another_course_data.course_anchor,
        }
    }

    // Return this instance as a Holochain entry. Useful for all HDK actions
    pub fn entry(&self) -> Entry {
        Entry::App("course_data".into(), self.into())
    }
}

impl CourseAnchor {
    // Constructor
    pub fn new(owner: Address, timestamp: u64) -> Self {
        CourseAnchor {
            timestamp: timestamp,
            teacher_address: owner,
        }
    }

    // Return this instance as a Holochain entry. Useful for all HDK actions
    pub fn entry(&self) -> Entry {
        Entry::App("course_anchor".into(), self.into())
    }
}

/* *************** CourseAnchor HDK entry definition */
pub fn course_anchor_def() -> ValidatingEntryType {
    entry!(
        name: "course_anchor",
        description: "Anchor entry that can be safely used to address a particular course",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<CourseAnchor>| {
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                    validate_only_teacher_can_do(validation_data.sources(), &entry.teacher_address, "create")
                },
                EntryValidationData::Delete {old_entry, validation_data, .. } => {
                    validate_only_teacher_can_do(validation_data.sources(), &old_entry.teacher_address, "delete")
                },
                // course_anchor can't be modified so there's no need to validate this action
                EntryValidationData::Modify { .. } => Ok(())
            }
        },
        links: [
            to!( // to query this course's latest data
                "course_anchor",
                link_type: "course_anchor->course_data",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    // TODO:  should validate that we're only linking against CourseData that has
                    // this entry's address in course_anchor field
                    Ok(())
                }
            ),
            from!( // to query all the courses of a user(all courses that a user is the teacher or owner of)
                "%agent_id",
                link_type: "teacher->courses",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
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
                },
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

/* *************** CourseData HDK entry definition */
pub fn course_data_def() -> ValidatingEntryType {
    entry!(
        name: "course_data",
        description: "Entry type that holds all course data that could be modified",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<CourseData>| {
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                    validate_only_teacher_can_do(validation_data.sources(), &entry.teacher_address, "create")?;
                    validate_course_title(&entry.title)
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    validate_only_teacher_can_do(validation_data.sources(), &old_entry.teacher_address, "modify")?;
                    validate_course_title(&new_entry.title)
                },
                EntryValidationData::Delete { old_entry, validation_data, .. } => {
                    validate_only_teacher_can_do(validation_data.sources(), &old_entry.teacher_address, "delete")
                }
            }
        },
        links: []
    )
}

/* *************** Course Validations */
fn validate_course_title(title: &str) -> Result<(), String> {
    if title.len() > 50 {
        Err("Course title is too long".into())
    } else {
        Ok(())
    }
}

fn validate_only_teacher_can_do(
    validation_data_sources: Vec<Address>,
    teacher_addr: &Address,
    action_name: &str,
) -> Result<(), String> {
    if !validation_data_sources.contains(teacher_addr) {
        return Err(format!(
            "Only the teacher can {} their courses",
            action_name
        ));
    }
    Ok(())
}

fn validate_no_teacher_change(
    old_teacher_addr: &Address,
    new_teacher_addr: &Address,
) -> Result<(), String> {
    if old_teacher_addr != new_teacher_addr {
        return Err(String::from("Cannot change the teacher of the course"));
    }
    Ok(())
}

/* ********************************************* */

/* *************** Anchor for all courses HDK entry definition */
// This Anchor will be used to query all available courses
pub fn anchor_all_courses_entry_def() -> ValidatingEntryType {
    entry!(
        name: "anchor-all-courses",
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
                "course_anchor",
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

/* *************** Anchor for all courses helper functions for HDK entry */
pub fn anchor_all_courses_entry() -> Entry {
    Entry::App("anchor-all-courses".into(), "course_anchor".into())
}

pub fn anchor_all_courses_address() -> ZomeApiResult<Address> {
    hdk::entry_address(&anchor_all_courses_entry())
}
/* ********************************************* */

/* *************** Course Helper Functions: CRUD */
pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    // create anchor for all courses
    // it's an idempotent action that won't change anything if this anchor already exists in DHT
    let anchor_entry = anchor_all_courses_entry();
    // if this entry already exists, we'll get it's address without creating anything new
    let anchor_address = hdk::commit_entry(&anchor_entry)?;

    // create a CourseAnchor entry and commit it to DHT so we have it's address
    let course_anchor_entry =
        CourseAnchor::new(AGENT_ADDRESS.to_string().into(), timestamp).entry();
    let course_anchor_address = hdk::commit_entry(&course_anchor_entry)?;

    // link CourseAnchor to anchor_all_courses for this course to be discoverable
    hdk::link_entries(&anchor_address, &course_anchor_address, "course_list", "")?;

    // create a CourseData instance
    let course_data = CourseData::new(
        title,
        AGENT_ADDRESS.to_string().into(),
        course_anchor_address.clone(),
        timestamp,
    );

    // commit instance to Holochain
    let _course_data_address = commit_course_data(&course_data)?;

    Ok(course_anchor_address)
}

pub fn update(
    title: String,
    modules_addresses: Vec<Address>,
    course_address: &Address,
) -> ZomeApiResult<Address> {
    let (_course_anchor, course_data) = get_anchor_and_latest_data_entries(course_address.clone())?;
    // save address of course_anchor entry separately to be independent of 
    // course_data lifetime
    let course_anchor_addr = course_data.course_anchor.clone();

    // create a new version of CourseData instance
    let new_course_data_entry = CourseData::from(
        title,
        course_data.teacher_address,
        course_data.course_anchor,
        // TODO: we shouldn't be using old timestamp here. Maybe we should receive it as a parameter for update?
        course_data.timestamp,
        modules_addresses,
    );
    let _new_course_data_address = commit_course_data(&new_course_data_entry)?;

    // Since CourseAnchor entry stays the same, we're not returning any new addresses here.
    // And in order for the course_anchor_addr to be valid after this method ends, we clone it
    Ok(course_anchor_addr.clone())
}

pub fn delete(course_address: Address) -> ZomeApiResult<Address> {
    let (course_anchor, course_data) = get_anchor_and_latest_data_entries(course_address.clone())?;
    // save address of course_anchor entry separately to be independent of 
    // course_data lifetime
    let course_anchor_addr = course_data.course_anchor.clone();

    // remove link from all_courses anchor to this course's CourseAnchor entry
    hdk::remove_link(
        &anchor_all_courses_address()?,
        &course_anchor_addr,
        "course_list",
        "",
    )?;

    // find addresses of all CourseData entries for the course
    let course_data_addresses = hdk::get_links(
        &course_anchor_addr,
        LinkMatch::Exactly("course_anchor->course_data"),
        LinkMatch::Any,
    )?
    .addresses();

    // remove all CourseData for the course
    for course_data_addr in course_data_addresses {
        hdk::remove_entry(&course_data_addr)?;
    }

    // now go through all the students linked to this course and remove their links as well
    let students = get_students(course_anchor_addr.clone())?;
    for student in students {
        hdk::remove_link(&student, &course_anchor_addr, "student->course", "")?;
    }
    // remove link between teacher and this course
    hdk::remove_link(
        &course_anchor.teacher_address,
        &course_anchor_addr,
        "teacher->courses",
        "",
    )?;

    hdk::remove_entry(&course_anchor_addr)
}

pub fn list() -> ZomeApiResult<Vec<Address>> {
    // first, retrieve addresses of all CourseAnchor entries
    // that would allow us to get access to the underlying CourseData
    let course_anchor_addresses = hdk::get_links(
        &anchor_all_courses_address()?,
        LinkMatch::Exactly("course_list"),
        LinkMatch::Any,
    )?
    .addresses();

    let mut course_data_addr_vec: Vec<Address> = Vec::new();

    for course_anchor_addr in course_anchor_addresses {
        // get latest CourseData for each CourseAnchor and add it to the list
        let course_data = get_latest_link_addr(&course_anchor_addr, "course_anchor->course_data")?;
        course_data_addr_vec.push(course_data);
    }

    // We're only returning CourseData addresses because each CourseData entry already
    // contains address of the corresponding CourseAnchor and there's no point in repeating this information
    Ok(course_data_addr_vec)
}
/* ********************************************* */

/* *************** Course Helper Functions: Other */
pub fn get_latest_link_addr(base_address: &Address, link_name: &str) -> ZomeApiResult<Address> {
    // NOTE: this method is written with an assumption that we'll always have ordered links vector
    // where latest links are appended to it's end.
    // I don't have a proof of that just yet.
    let latest_addr = hdk::get_links(base_address, LinkMatch::Exactly(link_name), LinkMatch::Any)?
        .addresses()
        .pop()
        .unwrap();
    Ok(latest_addr)
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
    // we don't need course_anchor here so we'll just ignore this variable
    let (_course_anchor, course_data) = get_anchor_and_latest_data_entries(course_address.clone())?;

    let mut updated_course_data: CourseData = CourseData::from_instance(course_data);
    updated_course_data.modules.push(module_address.clone());

    // NOTE: we don't actually need this address here, but Rust won't let us ignore return value
    // of the function, so we'll just ignore this variable.
    // Also If we decide to return address of the updated CourseData, we already have it saved here
    let _updated_course_data_addr = commit_course_data(&updated_course_data);

    Ok(updated_course_data.course_anchor)
}

pub fn delete_module_from_course(
    course_address: &Address,
    module_address: &Address,
) -> ZomeApiResult<Address> {
    // we don't need course_anchor here so we'll just ignore this variable
    let (_course_anchor, course_data) = get_anchor_and_latest_data_entries(course_address.clone())?;

    let mut updated_course_data: CourseData = CourseData::from_instance(course_data);
    // remove module from vec of modules
    updated_course_data.modules.remove_item(&module_address);
    updated_course_data.timestamp += 1; // we need to prevent duplication by changing the array.

    // NOTE: we don't actually need this address here, but Rust won't let us ignore return value
    // of the function, so we'll just ignore this variable.
    // Also If we decide to return address of the updated CourseData, we already have it saved here
    let _updated_course_data_addr = commit_course_data(&updated_course_data);

    Ok(updated_course_data.course_anchor)
}

fn get_anchor_and_latest_data_entries(
    some_address: Address,
) -> ZomeApiResult<(CourseAnchor, CourseData)> {
    // This method expects to receive address of either CourseAnchor or CourseData
    // and it returns tuple with CourseAnchor entry & latest CourseData entry corresponding to the course.
    // It will panic if address doesn't correspond to either CourseAnchor / CourseData

    // we'll need a course_data variable in the outer scope
    // and we'll need to check if it's been initialized already, thus usage of Option
    let mut course_data: Option<CourseData> = None;

    // NOTE: cloning some_address because we'll need to use it several times
    // and get_as_type will consume it otherwise
    let course_anchor: CourseAnchor = match hdk::utils::get_as_type(some_address.clone()) {
        Ok(course_anchor) => course_anchor,
        // if failed to get CourseAnchor entry from some_address, try to get it as CourseData
        Err(_) => {
            let tmp_course_data: CourseData = match hdk::utils::get_as_type(some_address.clone()) {
                Ok(course_data) => course_data,
                // at this point we've tried to retrieve provided address as both CourseAnchor and
                // CourseData, and both options have failed. We can't do anything else than return
                // an error of some sort (and panic is the quickiest way to do so)
                Err(_) => {
                    panic!(
                        "Provided address doesn't correspond to neither CourseData or CourseAnchor"
                    );
                }
            };
            // clone this course_data into outer scope variable
            course_data = Some(tmp_course_data.clone());
            // retrieve course_anchor from course_data's implicit link
            hdk::utils::get_as_type(tmp_course_data.course_anchor)?
        }
    };

    let course_data = match course_data {
        Some(data) => data,
        None => {
            // NOTE: if course_data isn't initialized yet and we're executing this code,
            // it means that we've managed to retrieve course_anchor from some_address
            // and therefore we can assume that some_address is in fact course_anchor's address
            // TODO: instead of ?, should provide the same error text that is in panic a few lines above
            let latest_course_data_addr =
                get_latest_link_addr(&some_address, "course_anchor->course_data")?;
            hdk::utils::get_as_type(latest_course_data_addr)?
        }
    };

    Ok((course_anchor, course_data))
}

fn commit_course_data(course_data: &CourseData) -> ZomeApiResult<Address> {
    // Since every CourseData already has address of corresponding CourseAnchor,
    // we can avoid having it as a second parameter for this func
    // NOTE: this creates a situation where course_anchor field of CourseData
    // entry points to a wrong CourseAnchor and we'll be creating invalid link.
    // However, this means that we're working with a corrupted CourseData instance
    // and I'm not sure that invalid link would be the biggest of problems in this case
    let updated_course_data_address = hdk::commit_entry(&course_data.entry())?;

    // create a link from CourseAnchor at course_data.course_anchor
    // to CourseData at updated_course_data_address
    // This is for CourseData to be discoverable from CourseAnchor
    hdk::link_entries(
        &course_data.course_anchor,
        &updated_course_data_address,
        "course_anchor->course_data",
        "",
    )?;

    Ok(updated_course_data_address)
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
