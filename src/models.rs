use std::collections::HashMap;

#[derive(Default, Debug, PartialEq)]
pub enum Status {
    // this set the default Status
    #[default]
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Default)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            // this set the default Status
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            ..Default::default()
        }
    }
}

pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epic_creation() {
        let default = Epic::new("".to_string(), "".to_string());
        assert_eq!(default.status, Status::Open);
    }

    #[test]
    fn test_story_creation() {
        let default = Story::new("".to_string(), "".to_string());
        assert_eq!(default.status, Status::Open);
    }
}
