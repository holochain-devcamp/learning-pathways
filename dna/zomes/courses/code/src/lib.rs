/***************** Required Library */
#![feature(vec_remove_item)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::prelude::*;

//use hdk::holochain_json_api::json::JsonString;

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::AGENT_ADDRESS;
use hdk_proc_macros::zome;

//use std::convert::TryInto;

/******************************** */
mod content;
mod course;
mod module;
use course::CourseAnchor;
#[zome]
mod course_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[zome_fn("hc_public")]
    fn get_my_address() -> ZomeApiResult<Address> {
        Ok(AGENT_ADDRESS.clone())
    }

    /**************************** Course Entry Definition and Functions */
    #[entry_def]
    fn anchor_entry_definition() -> ValidatingEntryType {
        course::anchor_all_courses_entry_def()
    }

    #[entry_def]
    fn course_anchor_entry_definition() -> ValidatingEntryType {
        course::course_anchor_def()
    }

    #[entry_def]
    fn course_data_entry_definition() -> ValidatingEntryType {
        course::course_data_def()
    }

    #[zome_fn("hc_public")]
    fn create_course(title: String, timestamp: u64) -> ZomeApiResult<Address> {
        course::create(title, timestamp)
    }

    #[zome_fn("hc_public")]
    fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

    #[zome_fn("hc_public")]
    fn update_course(
        title: String,
        modules_addresses: Vec<Address>,
        course_address: Address,
    ) -> ZomeApiResult<Address> {
        course::update(title, modules_addresses, &course_address)
    }

    #[zome_fn("hc_public")]
    fn delete_course(course_address: Address) -> ZomeApiResult<Address> {
        course::delete(course_address)
    }

    #[zome_fn("hc_public")]
    fn get_all_courses() -> ZomeApiResult<Vec<Address>> {
        course::list()
    }
    #[zome_fn("hc_public")]
    fn get_my_courses() -> ZomeApiResult<Vec<Address>> {
        course::get_my_courses()
    }

    #[zome_fn("hc_public")]
    fn get_my_enrolled_courses() -> ZomeApiResult<Vec<Address>> {
        course::get_my_enrolled_courses()
    }

    #[zome_fn("hc_public")]
    fn enrol_in_course(course_address: Address) -> ZomeApiResult<Address> {
        course::enrol_in_course(course_address)
    }

    #[zome_fn("hc_public")]
    fn get_all_students(course_address: Address) -> ZomeApiResult<Vec<Address>> {
        course::get_students(course_address)
    }

    /**************************** Module Entry Definition & Functions */
    #[entry_def]
    fn module_entry_definition() -> ValidatingEntryType {
        module::entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_module(
        title: String,
        course_address: Address,
        timestamp: u64,
    ) -> ZomeApiResult<Address> {
        module::create(title, &course_address, timestamp)
    }

    #[zome_fn("hc_public")]
    fn update_module(title: String, module_address: Address) -> ZomeApiResult<Address> {
        module::update(title, &module_address)
    }

    #[zome_fn("hc_public")]
    fn delete_module(module_address: Address) -> ZomeApiResult<Address> {
        module::delete(module_address)
    }

    /**************************** Content Zome Functions */
    #[entry_def]
    fn content_entry_definition() -> ValidatingEntryType {
        content::module_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_content(
        name: String,
        module_address: Address,
        url: String,
        timestamp: u64,
        description: String,
    ) -> ZomeApiResult<Address> {
        content::create(name, module_address, url, timestamp, description)
    }

    #[zome_fn("hc_public")]
    fn get_contents(module_address: Address) -> ZomeApiResult<Vec<Address>> {
        content::get_contents(&module_address)
    }
    #[zome_fn("hc_public")]
    fn delete_content(content_address: Address) -> ZomeApiResult<Address> {
        content::delete(content_address)
    }

    #[zome_fn("hc_public")]
    fn update_content(
        content_address: Address,
        name: String,
        url: String,
        description: String,
    ) -> ZomeApiResult<Address> {
        content::update(content_address, name, url, description)
    }
}
