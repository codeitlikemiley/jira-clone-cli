pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    name: String,
    descriotion: String,
    status: Status,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        todo!() // by default the status should be set to open and the stories should be an empty vector
    }
}

pub struct Story {
    // TODO: add fields (make sure the fields are public)
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        todo!() // by default the status should be set to open
    }
}

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    // TODO: add fields (make sure the fields are public)
}
