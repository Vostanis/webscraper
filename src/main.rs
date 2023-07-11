/*
    NOTES
    =================================
    
    1. States & utility of individual webscrapers
        a. get(/post?) schedule
        b. ip/location trail
    2. Active Tracking Database = Arc(Mutex(HashMap))) - comes after;
        replace HashMap with Hashmap<Uuid, Arc<Mutex<Scraper>>> due to borrow issue
        let mut db: HashMap<Uuid, Arc<Mutex, <Scraper>>> = Arc::new(Mutex::new(HashMap::new()));
*/

#![allow(unused_imports)]
#![allow(dead_code)]

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::vec::Vec;
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
    trail: Vec<TimeAndPlace>,
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

#[derive(Debug)]
struct TimeAndPlace {
    time: DateTime<Utc>,
    location: String,
}

impl Scraper {
    // returns Idle status
    fn spawn() -> Self {
        Self {
            status: Status::Idle,
            info: Info {
                key: Uuid::new_v4(),
                name: String::from("Jak"),               // <<<<<<<< EXAMPLE
                trail: vec![
                    TimeAndPlace {
                        time: Utc::now(),
                        location: String::from("Home"),
                    }
                ],
            },
            client: reqwest::Client::new(),
        }
    }

    // geography
    fn origin(&self) -> Option<&TimeAndPlace> {
        self.info.trail.first()
    }

    fn current_location(&self) -> Option<&TimeAndPlace> {
        self.info.trail.last()
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
    let s: Scraper = Scraper::spawn();   
    println!("{}:{} @ {:#?}", s.info.key, s.info.name, s.info.trail);
 }
