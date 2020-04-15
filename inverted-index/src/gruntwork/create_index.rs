pub fn create_index(content: String) {
    println!("Lets create some indexes");
    // println!("{}", content);
    // let mut i = 1;
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
            // i+=1;
        }
     }
}