use hdk::prelude::*;
use hdk::AGENT_ADDRESS;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Course {
    title: String,
    teacher_address: Address,
    modules: Vec<Address>, // Implicit link, as relationship with module
    timestamp: u64,
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "course",
        description: "this is a course definition",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Course>| {
            match _validation_data {
                // Replace ".." with "entry, .."
                EntryValidationData::Create { .. } => {
                    // Homework: add a validation rule that the title can only contain 50 chars or less
                    Ok(())
                },
                _ => Ok(())
            }
        }
    )
}

pub fn create(_title: String, _timestamp: u64) -> ZomeApiResult<Address> {
    let _teacher_address = AGENT_ADDRESS.clone();

    // Homework: finish the create course zome call
/* 
    let entry = Entry::App("my_entry".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address) */

    Err(ZomeApiError::from(String::from("Do your homework please")))
}

pub fn get_course(_course_address: Address) -> ZomeApiResult<Option<Entry>> {
    // Homework: finish the get course call
    // Hint: use hdk::get_entry

    Err(ZomeApiError::from(String::from("Do your homework please")))
}

pub fn delete_course(_course_address: Address) -> ZomeApiResult<Address> {
    // Homework: finish the delete course call
    // Hint: use hdk::remove_entry

    Err(ZomeApiError::from(String::from("Do your homework please")))
}
