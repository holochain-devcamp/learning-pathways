use hdk::prelude::*;
use hdk::AGENT_ADDRESS;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Course {
    title: String,
    teacher_address: Address,
    modules: Vec<Address>, // Implicit link, as relationship with module
    timestamp: u64,
}
impl Course {
    pub fn new(title: String, owner: Address, timestamp: u64) -> Self {
        Course {
            title: title,
            teacher_address: owner,
            modules: Vec::default(),
            timestamp: timestamp,
        }
    }
    pub fn from(title: String, owner: Address, timestamp: u64, modules: Vec<Address>) -> Self {
        Course {
            title: title,
            teacher_address: owner,
            modules: modules,
            timestamp: timestamp,
        }
    }
    pub fn entry(&self) -> Entry {
        Entry::App("course".into(), self.into())
    }
}

/*********************** Course Validations */
fn validate_course_title(title: &str) -> Result<(), String> {
    if title.len() > 50 {
        Err("Course title is too long".into())
    } else {
        Ok(())
    }
}

pub fn entry_definition() -> ValidatingEntryType {
    entry!(
        name: "course",
        description: "this is a course definition",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Course>| {
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                    if !validation_data.sources().contains(&entry.teacher_address) {
                        return Err(String::from("Only the teacher can create their courses"));
                    }

                    validate_course_title(&entry.title)
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    if new_entry.teacher_address != old_entry.teacher_address {
                        return Err(String::from("Cannot change the teacher of the course"));
                    }

                    if !validation_data.sources().contains(&old_entry.teacher_address) {
                        return Err(String::from("Only the teacher can modify their courses"));
                    }

                    validate_course_title(&new_entry.title)?;

                    Ok(())
                },
                EntryValidationData::Delete {old_entry, validation_data, .. } => {
                    if !validation_data.sources().contains(&old_entry.teacher_address) {
                        return Err(String::from("Only the teacher can delete their courses"));
                    }

                    Ok(())
                }
            }
        }
    )
}

pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    let new_course = Course::new(title, AGENT_ADDRESS.to_string().into(), timestamp);
    let new_course_entry = new_course.entry();
    let new_course_address = hdk::commit_entry(&new_course_entry)?;
    return Ok(new_course_address);
}

pub fn get_course(_course_address: Address) -> ZomeApiResult<Option<Entry>> {
    // Homework: finish the get course call
    // Hint: use hdk::get_entry

    Err(ZomeApiError::from(String::from("Do your homework please")))
}

pub fn delete_course(course_address: Address) -> ZomeApiResult<Address> {
    hdk::remove_entry(&course_address)
}

pub fn update(
    title: String,
    modules_addresses: Vec<Address>,
    course_address: &Address,
) -> ZomeApiResult<Address> {
    let course: Course = hdk::utils::get_as_type(course_address.clone())?;

    let new_version_course = Course::from(
        title,
        course.teacher_address,
        course.timestamp,
        modules_addresses,
    );
    let new_version_course_entry = new_version_course.entry();

    hdk::update_entry(new_version_course_entry, course_address)
}
