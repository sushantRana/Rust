use mongodb::{Client, options::ClientOptions};
use bson::{doc, Bson};
pub fn read_index(search_value: String) {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").expect("failed to get client options");
    let client = Client::with_options(client_options).expect("failed to get client with_options");
    let db = client.database("inverted_index");
    let collection = db.collection("inverted_collection");
    let cursor_value = collection.find(doc! {"title": search_value.trim()}, None).expect("failed to find_one");
    for result in cursor_value {
        match result {
            Ok(document) => {
                if let Some(title) = document.get("title").and_then(Bson::as_str) {
                    println!("title: {}", title);
                }  else {
                    println!("no title found");
                }
            }
            Err(e) => println!("error {:?}", e)
        }
    }
}