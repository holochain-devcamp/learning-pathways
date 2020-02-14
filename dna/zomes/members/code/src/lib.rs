#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate holochain_json_derive;

use hdk::holochain_persistence_api::cas::content::Address;

use hdk::prelude::*;
use hdk_proc_macros::zome;

pub mod members;

#[zome]
mod members_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
       Ok(())
    }

    #[zome_fn("hc_public")]
    fn get_valid_members() -> ZomeApiResult<Vec<Address>> {
        members::get_valid_members()
    }

    #[zome_fn("hc_public")]
    fn is_member_valid(agent_address: Address) -> ZomeApiResult<bool> {
        members::is_member_valid(&agent_address)
    }
}
