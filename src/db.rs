

use crate::models::{DBState, Epic, Story, Status};
use std::io::{Write, Read};
use std::fs;

use anyhow::{Error, Result};
use anyhow::anyhow;


pub struct JiraDatabase {
    database: Box<dyn Database>,
    readonly : bool
}

#[allow(unused_variables)]
impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        Self {
            database: Box::new(JSONFileDatabase { file_path }),
            readonly: false        
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }
    
    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
            let mut parsed = self.database.read_db()?;

            let lastid = parsed.last_item_id;
            let newid = lastid+1;

            parsed.last_item_id = newid;
            parsed.epics.insert(newid, epic);
        
            if (!self.readonly) {  self.database.write_db(&parsed)?;}
            Ok(newid)
    }
    
    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut parsed = self.database.read_db()?;
    
        let last_id = parsed.last_item_id;
        let new_id = last_id + 1;
        
        parsed.last_item_id = new_id;
        parsed.stories.insert(new_id, story);


        //parsed.epics.get_mut(&epic_id).ok_or_else(|| anyhow!("could not find epic in database!"))?.stories.push(new_id);
        let eid = parsed.epics.get_mut(&epic_id);

        match eid {
            Some(eid)=>{ eid.stories.push(new_id);},
            //None =>  println!("could not find epic in database!"), 
            //None => { return Err("could not find epic in database!".to_string())} ,  
            _ => ()
        }    
    
        self.database.write_db(&parsed)?;
        Ok(new_id)
     
    }
    
    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        todo!()
    }
    
    pub fn delete_story(&self,epic_id: u32, story_id: u32) -> Result<()> {
        todo!()
    }
    
    pub fn update_epic_status(&self, epic_id: u32, status: Status) -> Result<()> {
        todo!()
    }
    
    pub fn update_story_status(&self, story_id: u32, status: Status) -> Result<()> {
        todo!()
    }
}

trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

struct JSONFileDatabase {
    pub file_path: String
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let db_content = fs::read_to_string(&self.file_path);
        match db_content {
            Ok(db_content) =>  {
            let parsed: DBState = serde_json::from_str(&db_content)?;
            return Ok(parsed);
            }
            Err(e) => {
                println!("Error reading file: {}", e);
                return Err(e.into());
                //return Ok(DBState::new());
            }
        }      
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        fs::write(&self.file_path, &serde_json::to_vec(db_state)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod database {
        use std::collections::HashMap;
        use std::io::Write;

        use super::*;

        #[test]
        fn read_db_should_fail_with_invalid_path() {
            let db = JSONFileDatabase { file_path: "INVALID_PATH".to_owned() };
            assert_eq!(db.read_db().is_err(), true);
        }

        #[test]
        fn read_db_should_fail_with_invalid_json() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0 epics: {} stories {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.read_db();

            assert_eq!(result.is_err(), true);
        }

        #[test]
        fn read_db_should_parse_json_file() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let result = db.read_db();

            assert_eq!(result.is_ok(), true);
        }

        #[test]
        fn write_db_should_work() {
            let mut tmpfile = tempfile::NamedTempFile::new().unwrap();

            let file_contents = r#"{ "last_item_id": 0, "epics": {}, "stories": {} }"#;
            write!(tmpfile, "{}", file_contents).unwrap();

            let db = JSONFileDatabase { file_path: tmpfile.path().to_str()
                .expect("failed to convert tmpfile path to str").to_string() };

            let story = Story { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open };
            let epic = Epic { name: "epic 1".to_owned(), description: "epic 1".to_owned(), status: Status::Open, stories: vec![2] };

            let mut stories = HashMap::new();
            stories.insert(2, story);

            let mut epics = HashMap::new();
            epics.insert(1, epic);

            let state = DBState { last_item_id: 2, epics, stories };

            let write_result = db.write_db(&state);
            let read_result = db.read_db().unwrap();

            assert_eq!(write_result.is_ok(), true);
            // TODO: fix this error by deriving the appropriate traits for DBState
            assert_eq!(read_result, state);
        }
    }
}