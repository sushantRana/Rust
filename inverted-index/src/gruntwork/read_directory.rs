use std::io::Read;
use std::fs;
use crate::gruntwork::create_index::create_index;
use mongodb::{Client, options::ClientOptions};

pub fn read_directorycode(filepath: String) {
    let paths = fs::read_dir(filepath.trim()).unwrap();
    for path in paths {
        let filename = path.unwrap().path();
        // println!("Name: {:?}", filename);
        let mut file = std::fs::File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        // print!("{}", contents);
        create_index(contents);
        // databases();
        break;
    }
}

pub fn databases() {
//     let mut client_options = ClientOptions::parse("mongodb://localhost:27017").unwrap();
//     println!("{:?}", client_options);
//     // Manually set an option.
//     client_options.app_name = Some("inverted_index".to_string());

// Get a handle to the deployment.
    let client = Client::with_uri_str("mongodb://127.0.0.1:27017").unwrap();
    println!("{:?}", client);
    
    // List the names of the databases in that deployment.
    let db = client.database("inverted_index");
    println!("{:?}", db);
}