use reqwest::Client;
use std::collections::HashMaps;
use uuid::Uuid;

// generals
fn generate_primary_key() -> [u8; 16] {
    let uuid = Uuid::new_v4();
    &uuid.as_bytes();
}

// db
let mut db: HashMap<uuid::Bytes, String> = HashMap::New();

// scraper
struct Scraper {
    state: Status,
    info: Info,
    // purpose
}

struct Info {
    // Known @ compile time
    Keycard: uuid::Bytes,
    Name: String,

    // Known @ exec time
    TimeOfBirth: String,
    Birthplace: String,
    TimeOfDeath: String,
    Deathplace: String,
}

enum Status {
    Idle,           // awaiting action
    Retrieving,     // retrieving data
    Scanning,       // scanning a location before hopping, or a file before downloading
    Init,           // initialising ...
    Error,
}

impl Scraper {

    fn spawn() -> Self {
        let client = Client::new();
        //randomly generate keycard and name
        //let  = generate_primary_key();
    }

    fn live() -> Self {

    }

    fn is_alive() -> Self {

    }

    fn kill() -> Self {

    }

    fn handle_status(&self) {
        Status::Idle => {},
        Status::Retrieving => {},
        Status::Scanning => {},
        Status::Init => {
            &self.spawn();
        },
        Status::Error => {},
    } 
}

fn main() {
    let mut active_state = ScraperState::Init;
}
