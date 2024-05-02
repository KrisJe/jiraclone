mod models;

use crate::models::{DBState, Epic, Status, Story};

use std::{collections::HashMap};

fn main() {


    let mydb = DBState{
        last_item_id : 0,
        epics : HashMap::new(),
        stories: HashMap::new()
    };
    println!("Welcome To My-Jira!");
}

mod tests {
    mod database {
        use std::collections::HashMap;
        
        use crate::models::{DBState, Epic, Status, Story};
        #[test]
        fn create_new_DBState() {
           //todo!()

           let story = Story {
            name: "epic 1".to_owned(),
            description: "epic 1".to_owned(),
            status: Status::Open,
        };

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


       
        }

    }
}