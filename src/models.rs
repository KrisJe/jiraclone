

use std::{collections::HashMap};

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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
            status: Status::Open,
            stories: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
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
            status: Status::Open,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>
}



impl DBState {
    pub fn new() -> Self {
        Self {
            last_item_id  : 0,
            epics: HashMap::new(),    
            stories: HashMap::new(),       
        }
    }
}

mod tests {
    mod database {
        use std::collections::HashMap;
        
        use crate::models::{DBState, Epic, Status, Story};
        #[test]
        fn create_new_DBState() {
           let story = Story {
            name: "epic 1".to_owned(),
            description: "epic 1".to_owned(),
            status: Status::Open,
        };

        let apic2 = Epic::new("epic 1".to_owned(),"epic 1".to_owned());

        let epic = Epic {
            name: "epic 1".to_owned(),
            description: "epic 1".to_owned(),
            status: Status::Open,
            stories: vec![2],
        };
         //assert_eq!(epic.status, Status::Open); doesn't work
        //assert_eq!(story.status, Status::Open);
        assert!(matches!(epic.status,Status::Open));
        assert!(matches!(story.status,Status::Open));

        let mut stories = HashMap::new();
        stories.insert(2, story);

        let mut epics = HashMap::new();
        epics.insert(1, epic);

        let state = DBState {
            last_item_id: 2,
            epics,
            stories,
        };       

        assert!(matches!(state.last_item_id,2));

       
        }

    }
}