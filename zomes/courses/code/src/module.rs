/************************ Import Required Libraries */
use hdk::prelude::*;

use crate::course::Course;
use std::convert::TryFrom;
/******************************************* */

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Module {
    pub title: String,
    pub timestamp: u64,
    pub course_address: Address    
}

impl Module{
    pub fn new(title: String, course_address: Address, timestamp: u64) -> Self {
        Module {
            title: title,
            course_address: course_address,
            timestamp: timestamp,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("module".into(), self.into())
    }
}

/*********************** Course Validations */
fn validate_module_title(title: &str) -> Result<(), String> {
    if title.len() > 200 {
        Err("Module title is too long".into())
    } else {
        Ok(())
    }
}


// Entry Definition
pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: "module",
        description: "this is the definition of module",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Module>| {
          Ok(())
        },
        links:[
            to!(
                "content",
                link_type: "module->contents",
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                     // TODO: finish validation on Create/Update/Delete.  Only the owner of course, can create/delete/update module of course.
                    Ok(())
                }
            )
        ]
    )
}