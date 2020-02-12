use hdk::prelude::*;

pub fn get_valid_members() -> ZomeApiResult<Vec<Address>> {
    let valid_members_json = hdk::property("valid_members")?;
    let valid_members: Result<Vec<Address>, _> =
        serde_json::from_str(&valid_members_json.to_string());

    match valid_members {
        Ok(valid_members_addresses) => Ok(valid_members_addresses),
        Err(_) => Err(ZomeApiError::from(String::from(
            "Could not get the valid members for this app",
        ))),
    }
}

pub fn is_member_valid(agent_address: &Address) -> ZomeApiResult<bool> {
    let valid_members = get_valid_members()?;

    Ok(valid_members.contains(agent_address))
}
