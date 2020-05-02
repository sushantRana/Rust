
use mongodb::{Client, options::ClientOptions};
use bson::{doc, Bson};
pub fn create_index(content: String, filepath: &str) {
    // println!("Lets create some indexes");
    let client_options = ClientOptions::parse("mongodb://localhost:27017").expect("failed to get client options");
    let client = Client::with_options(client_options).expect("failed to get client with_options");
    let db = client.database("inverted_index");
    let collection = db.collection("inverted_collection");

    let mut i = 1;
    let ignore_tokens = ["the", "the", "you", "we", "is", "an", "and"];
    for token in content.split_whitespace(){
        if token.len() > 2 && !ignore_tokens.contains(&token) {
            // println!("token {} {}",i,token);
            // tokens to be saved in two steps -
            // Step 1 - search if token exist in the file
            // Step 2 - if yes update to the same the column
            // here save the tokens to a file and use later for searching
            // ------------------------
            // |      |      |        |
            // |      |      |        |
            // |______|______|________|
            // |      |      |        |
            // |      |      |        |
            // ------------------------
            i+=1;
            let value = token;
            let value_str = value.clone();
            let value_str_dup = value.clone();
            // insert_update(token.to_lowercase(), filepath);
            let cursor_value = collection.find_one(doc! {"title": value}, None).expect("failed to find_one");
            match cursor_value {
                Some(document) => {
                    let filepath_non_mut = document.get("filepath").and_then(Bson::as_array).expect("failed to get filepath");
                    let mut filepath_mut = filepath_non_mut.clone();
                    filepath_mut.push(bson::to_bson(&filepath.to_string()).unwrap());
                    // println!("inserting {:?}", value_str_dup);
                    let result_update = collection.update_one(doc! {"title": value_str}, doc! {"title": value_str_dup, "filepath": filepath_mut}, None);
                    match result_update {
                        Ok(result) => {
                            // println!("{:?}", result);
                        }
                        Err(e) => println!("{:?}", e)
                    }
                }
                None => {
                    collection.insert_one(doc! {"title": value_str, "filepath": [filepath]}, None).unwrap();
                }
            }
        }
     }
}

pub fn insert_update(value: String, path: &str) {
    // let value_str = value.clone();
    // let value_str_dup = value.clone();
    // let client_options = ClientOptions::parse("mongodb://localhost:27017").expect("failed to get client options");
    // let client = Client::with_options(client_options).expect("failed to get client with_options");
    // let db = client.database("inverted_index");
    // let collection = db.collection("inverted_collection");
    
    // let cursor_value = collection.find_one(doc! {"title": value}, None).expect("failed to find_one");

    // match cursor_value {
    //     Some(document) => {
    //         let filepath_non_mut = document.get("filepath").and_then(Bson::as_array).expect("failed to get filepath");
    //         let mut filepath_mut = filepath_non_mut.clone();
    //         filepath_mut.push(bson::to_bson(&path.to_string()).unwrap());
    //         println!("inserting {:?}", value_str_dup);
    //         let result_update = collection.update_one(doc! {"title": value_str}, doc! {"title": value_str_dup, "filepath": filepath_mut}, None);
    //         match result_update {
    //             Ok(result) => {
    //                 println!("{:?}", result);
    //             }
    //             Err(e) => println!("{:?}", e)
    //         }
    //     }
    //     None => {
    //         collection.insert_one(doc! {"title": value_str, "filepath": [path]}, None).unwrap();
    //     }
    // }
}