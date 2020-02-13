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

// see https://developer.holochain.org/api/0.0.42-alpha5/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        match validation_data {
            EntryValidationData::Create {
                validation_data, ..
            } => {
                let agent_address = validation_data.package.chain_header.entry_address();
                match members::is_member_valid(&agent_address)? {
                    true => Ok(()),
                    false => Err(String::from("Error validating agent: agent is not valid")),
                }
            }
            _ => Err(String::from("Error validating agent")),
        }
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
