/************************ Import Required Libraries */
use hdk::prelude::*;

use crate::course::CourseAnchor;
use crate::course::{add_module_to_course, delete_module_from_course};
use std::convert::TryFrom;
/******************************************* */

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Module {
    pub title: String,
    pub timestamp: u64,
    pub course_address: Address,
}

impl Module {
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

pub fn validate_author(signing_addresses: &Vec<Address>, module: &Module) -> ZomeApiResult<()> {
    let course_anchor: CourseAnchor = hdk::utils::get_as_type(module.course_address.clone())?;
    hdk::debug(format!("{:?}", course_anchor))?;
    if !signing_addresses.contains(&course_anchor.teacher_address) {
        return Err(ZomeApiError::from(String::from(
            "Only the teacher can create or modify a module for it",
        )));
    }
    Ok(())
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
        validation: | validation_data: hdk::EntryValidationData<Module>| {
            match  validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    validate_module_title(&entry.title)?;

                    validate_author(&validation_data.sources(), &entry)?;

                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    validate_module_title(&new_entry.title)?;

                    if new_entry.course_address != old_entry.course_address {
                        return Err(String::from("Cannot modify the course of a module"));
                    }
                    validate_author(&validation_data.sources(), &new_entry)?;
                    Ok(())
                },
                EntryValidationData::Delete { old_entry, validation_data, .. } => {
                    validate_author(&validation_data.sources(), &old_entry)?;

                    Ok(())
                }
            }
        },
        links:[
            to!(
                "content",
                link_type: "module->contents",
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                // TODO: Homework. Implement validation rules if required.
                    Ok(())
                }
            )
        ]
    )
}

pub fn create(title: String, course_address: &Address, timestamp: u64) -> ZomeApiResult<Address> {
    let new_module = Module::new(title, course_address.clone(), timestamp);
    let new_module_address = hdk::commit_entry(&new_module.entry())?;

    add_module_to_course(course_address, &new_module_address)?;

    Ok(new_module_address)
}

pub fn update(title: String, module_address: &Address) -> ZomeApiResult<Address> {
    let mut module: Module = hdk::utils::get_as_type(module_address.clone())?;

    module.title = title;

    hdk::update_entry(module.entry(), module_address)
}

pub fn delete(module_address: Address) -> ZomeApiResult<Address> {
    let module: Module = hdk::utils::get_as_type(module_address.clone())?;

    let result = hdk::remove_entry(&module_address)?;

    delete_module_from_course(&module.course_address, &module_address)?;

    Ok(result)
}
