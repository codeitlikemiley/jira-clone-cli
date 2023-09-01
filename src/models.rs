use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Default)]
pub enum Status {
    // this set the default Status
    #[default]
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Default)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Default)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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
