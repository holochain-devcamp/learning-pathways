/************************ Import Required Libraries */
use crate::course;
use crate::course::CourseAnchor;
use crate::module::Module;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_core_types::{entry::Entry, validation::EntryValidationData};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::LinkMatch;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};
use holochain_wasm_utils::api_serialization::{
    get_entry::{GetEntryOptions, GetEntryResult},
    get_links::GetLinksOptions,
};
use std::convert::TryFrom;
/******************************************* */

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Content {
    name: String,
    url: String,
    description: String,
    timestamp: u64,
    module_address: Address,
}

impl Content {
    pub fn new(
        name: String,
        module_address: Address,
        url: String,
        timestamp: u64,
        description: String,
    ) -> Self {
        Content {
            name,
            url,
            description,
            timestamp,
            module_address,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("content".into(), self.into())
    }
}

////////////////////Course Entry Definition
pub fn module_entry_def() -> ValidatingEntryType {
    entry!(
        name: "content",
        description: "this is the content for each module",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Content>| {
            match  validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    validate_author(&validation_data.sources(), &entry.module_address)?;
                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    if new_entry.module_address != old_entry.module_address {
                        return Err(String::from("Cannot modify the module of a content"));
                    }
                    validate_author(&validation_data.sources(), &new_entry.module_address)?;
                    Ok(())
                },
                EntryValidationData::Delete { old_entry, validation_data, .. } => {
                    validate_author(&validation_data.sources(), &old_entry.module_address)?;

                    Ok(())
                }
            }
        }
    )
}
/////////////////////////// Validations
pub fn validate_author(
    signing_addresses: &Vec<Address>,
    module_address: &Address,
) -> ZomeApiResult<()> {
    let module: Module = hdk::utils::get_as_type(module_address.clone())?;
    let course_anchor: CourseAnchor = hdk::utils::get_as_type(module.course_address.clone())?;
    if !signing_addresses.contains(&course_anchor.teacher_address) {
        return Err(ZomeApiError::from(String::from(
            "Error: Only the teacher can create or modify a content for module",
        )));
    }
    Ok(())
}

/// Helper Functions
pub fn create(
    name: String,
    module_address: Address,
    url: String,
    timestamp: u64,
    description: String,
) -> ZomeApiResult<Address> {
    let new_content = Content::new(name, module_address.clone(), url, timestamp, description);
    let new_content_entry = new_content.entry();
    let new_content_address = hdk::commit_entry(&new_content_entry)?;
    hdk::link_entries(
        &module_address,
        &new_content_address,
        "module->contents",
        "",
    )?;

    Ok(new_content_address)
}

pub fn get_contents(module_address: &Address) -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &module_address,
        LinkMatch::Exactly("module->contents"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}
pub fn delete(content_address: Address) -> ZomeApiResult<Address> {
    let content: Content = hdk::utils::get_as_type(content_address.clone())?;

    hdk::remove_link(
        &content.module_address,
        &content_address,
        "module->contents",
        "",
    )?;

    hdk::remove_entry(&content_address)
}

pub fn update(
    content_address: Address,
    name: String,
    url: String,
    description: String,
) -> ZomeApiResult<Address> {
    let mut content: Content = hdk::utils::get_as_type(content_address.clone())?;
    content.description = description;
    content.name = name;
    content.url = url;
    hdk::update_entry(content.entry(), &content_address)
}
