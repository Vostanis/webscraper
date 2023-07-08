use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

// scraper
#[derive(Debug)]
struct Scraper {
    status: Status,
    info: Info,
    client: reqwest::Client,
}

#[derive(Debug)]
struct Info {
    key: Uuid,
    name: String,
    location: String,

    time_of_birth: DateTime<Utc>,
    birthplace: String,
    time_of_death: Option<DateTime<Utc>>,
    deathplace: Option<String>,
}

#[derive(Debug)]
enum Status {
    Idle,           // awaiting action
    Retrieving,     // retrieving data
    Scanning,       // scanning a location before hopping, or a file before downloading
    Init,           // initialising ...
    Dead,
    Error,
}

impl Scraper {
    // returns Idle status
    fn spawn() -> Self {
        Self {
            status: Status::Idle,
            info: Info {
                key: Uuid::new_v4(),
                name: String::from("Jak"),               // <<<<<<<< EXAMPLE
                location: String::from("home"),

                time_of_birth: Utc::now(),
                birthplace: String::from("home"),
                time_of_death: None,
                deathplace: None,
            },
            client: reqwest::Client::new(),
        }
    }
/*
    fn current_status(&self) -> Status {
        self.status.clone()
    }
*/
    fn set_status(&mut self, new_status: Status) {
        self.status = new_status;
    }
/*
    fn handle_status(&self) {
        Status::Idle => {},
        Status::Retrieving => {},
        Status::Scanning => {},
        Status::Init => {
            &self.spawn();
        },
        Status::Error => {},
    }
*/
}

fn main() { 
    let mut db: HashMap<Uuid, Scraper> = HashMap::new();
    let mut s: Scraper = Scraper::spawn();   
    println!("{}:{} @ {}", s.info.key, s.info.name, s.info.location);

    db.insert(s.info.key, s);
    for (k,v) in &db {
        println!("DB ~ {:#?}:{:#?}", k, v);
    } 
    //s.current_status();
    //s.set_status(Status::Error);
    //s.current_status();
}
