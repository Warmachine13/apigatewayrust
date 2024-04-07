use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    routes: Vec<Route>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Route {
    host: String,
}

pub fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<User, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let user = serde_json::from_reader(reader)?;

    Ok(user)
}
