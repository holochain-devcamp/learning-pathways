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

////////////////////CourseAnchor Definition
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
                    if !validation_data.sources().contains(&entry.teacher_address) {
                        return Err(String::from("Only the teacher can create their courses"));
                    }

                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    if new_entry.teacher_address != old_entry.teacher_address {
                        return Err(String::from("Cannot change the teacher of the course"));
                    }

                    if !validation_data.sources().contains(&old_entry.teacher_address) {
                        return Err(String::from("Only the teacher can modify their courses"));
                    }

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
            to!( // to query this course's latest data
                "%agent_id", // QUESTION: what should I specify here?
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
                EntryValidationData::Create { new_entry, .. } => {
                    validate_course_title(&new_entry.title)
                },
                EntryValidationData::Modify { entry, .. } => {
                    validate_course_title(&entry.title)
                },
                EntryValidationData::Delete { .. } => {
                    Ok(())
                }
            }
        },
        links: []
    )
}

//// Anchor for all courses definition : This Anchor will be used to query all courses
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

pub fn anchor_all_courses_entry() -> Entry {
    Entry::App("anchor-all-courses".into(), "course_anchor".into())
}

pub fn anchor_all_courses_address() -> ZomeApiResult<Address> {
    hdk::entry_address(&anchor_all_courses_entry())
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

    // create a CourseData entry & commit it to DHT
    let course_data_entry = CourseData::new(
        title,
        AGENT_ADDRESS.to_string().into(),
        course_anchor_address.clone(),
        timestamp,
    )
    .entry();
    let course_data_address = hdk::commit_entry(&course_data_entry)?;

    // link CourseData to CourseAnchor entry to be discoverable
    hdk::link_entries(
        &course_anchor_address,
        &course_data_address,
        "course_anchor->course_data",
        "",
    )?;

    Ok(course_anchor_address)
}

pub fn update(
    title: String,
    modules_addresses: Vec<Address>,
    course_address: &Address,
) -> ZomeApiResult<Address> {
    // get latest CourseData entry by looking at links of CourseAnchor entry
    let latest_course_data_addr =
        get_latest_link_addr(course_address, "course_anchor->course_data")?;
    let course_data: CourseData = hdk::utils::get_as_type(latest_course_data_addr)?;

    // create a new version of CourseData entry
    let new_course_data_entry = CourseData::from(
        title,
        course_data.teacher_address,
        course_data.course_anchor,
        // TODO: we shouldn't be using old timestamp here. Maybe we should receive it as a parameter for update?
        course_data.timestamp,
        modules_addresses,
    )
    .entry();
    let new_course_data_address = hdk::commit_entry(&new_course_data_entry)?;

    // link this new CourseData to CourseAnchor for it to be discoverable
    hdk::link_entries(
        &course_address,
        &new_course_data_address,
        "course_anchor->course_data",
        "",
    )?;

    // since CourseAnchor entry stays the same, we're not returning any new addresses here
    // and since we don't have ownership of the course_address in this method, we're cloning it
    // to comply with the return type requirements
    Ok(course_address.clone())
}

pub fn delete(course_address: Address) -> ZomeApiResult<Address> {
    // remove link from all_courses anchor to this course's CourseAnchor entry
    hdk::remove_link(
        &anchor_all_courses_address()?,
        &course_address,
        "course_list",
        "",
    )?;

    // retrieve all CourseData for the course
    let course_data_addresses = hdk::get_links(
        &course_address,
        LinkMatch::Exactly("course_anchor->course_data"),
        LinkMatch::Any,
    )?
    .addresses();

    // remove all CourseData for the course
    for course_data_addr in course_data_addresses {
        hdk::remove_entry(&course_data_addr)?;
    }

    // now go through all the students linked to this course and remove their links as well
    let students = get_students(course_address.clone())?;
    let course_anchor: CourseAnchor = hdk::utils::get_as_type(course_address.clone())?;
    for student in students {
        hdk::remove_link(&student, &course_address, "student->course", "")?;
    }
    // remove link between teacher and this course
    hdk::remove_link(
        &course_anchor.teacher_address,
        &course_address,
        "teacher->courses",
        "",
    )?;

    hdk::remove_entry(&course_address)
}

pub fn list() -> ZomeApiResult<Vec<Address>> {
    // TODO: need to refactor the way UI would be handling CourseData.
    // In the current implementation, UI would try to get addr of CourseData and pass it
    // to all Course helper functions, while they're expecting only CourseAnchor addresses.
    // Since these have the same datatype, it won't be cathed by Rust compiler or any other automated
    // methods and would only fail when trying to treat CourseData as CourseAnchor.

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

/********************************************** */
/// Course Helper Functions: Other

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
    let current_course_anchor = hdk::get_entry(course_address).unwrap().unwrap();

    // QUESTION: why are we doing it like that?
    // I see advantage of this method in handling a case when we receive address that
    // doesn't correspond to the CourseAnchor.
    // Does it mean course::update needs to be updated accordingly?
    if let Entry::App(_, current_course_anchor) = current_course_anchor {
        // NOTE: now that we're not going to be changing course_anchor_entry we probably should have a less verbose
        // way of throwing an error about type mismatch?
        let _course_anchor_entry = CourseAnchor::try_from(current_course_anchor.clone())
            .expect("Entry at this address is not Course. You sent a wrong address");
        let latest_course_data_addr =
            get_latest_link_addr(course_address, "course_anchor->course_data")?;
        let latest_course_data: CourseData = hdk::utils::get_as_type(latest_course_data_addr)?;

        // create a copy of the latest_course_data and add new module there
        let mut updated_course_data: CourseData = CourseData::from_instance(latest_course_data);
        updated_course_data.modules.push(module_address.clone());

        // commit new course_data
        let updated_course_data_address = hdk::commit_entry(&updated_course_data.entry())?;

        // link this new CourseData to CourseAnchor for it to be discoverable
        hdk::link_entries(
            &course_address,
            &updated_course_data_address,
            "course_anchor->course_data",
            "",
        )?;

        // since CourseAnchor entry stays the same, we're not returning any new addresses here
        // and since we don't have ownership of the course_address in this method, we're cloning it
        // to comply with the return type requirements
        Ok(course_address.clone())
    } else {
        panic!("This address is not a valid address")
    }
}

pub fn delete_module_from_course(
    course_address: &Address,
    module_address: &Address,
) -> ZomeApiResult<Address> {
    // TODO: this method is almost a complete copy-paste of add_module_to_course and needs to be refactored
    let current_course_anchor = hdk::get_entry(course_address).unwrap().unwrap();

    // QUESTION: why are we doing it like that?
    // I see advantage of this method in handling a case when we receive address that
    // doesn't correspond to the CourseAnchor.
    // Does it mean course::update needs to be updated accordingly?
    if let Entry::App(_, current_course_anchor) = current_course_anchor {
        // NOTE: now that we're not going to be changing course_anchor_entry we probably should have a less verbose
        // way of throwing an error about type mismatch?
        let _course_anchor_entry = CourseAnchor::try_from(current_course_anchor.clone())
            .expect("Entry at this address is not Course. You sent a wrong address");
        let latest_course_data_addr =
            get_latest_link_addr(course_address, "course_anchor->course_data")?;
        let latest_course_data: CourseData = hdk::utils::get_as_type(latest_course_data_addr)?;

        // create a copy of the latest_course_data and add new module there
        let mut updated_course_data: CourseData = CourseData::from_instance(latest_course_data);

        // remove module from vec of modules
        updated_course_data.modules.remove_item(&module_address);
        updated_course_data.timestamp += 1; // we need to prevent duplication by changing the array.

        // commit new course_data
        let updated_course_data_address = hdk::commit_entry(&updated_course_data.entry())?;

        // link this new CourseData to CourseAnchor for it to be discoverable
        hdk::link_entries(
            &course_address,
            &updated_course_data_address,
            "course_anchor->course_data",
            "",
        )?;

        // since CourseAnchor entry stays the same, we're not returning any new addresses here
        // and since we don't have ownership of the course_address in this method, we're cloning it
        // to comply with the return type requirements
        Ok(course_address.clone())
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
