use std::io;
mod gruntwork;
fn main() {
    // step 1 show options to users
    // 1. Create index
    // 2. search Contents
    // quit
    println!("Please select an option:");
    println!("Option 1: Create Index");
    println!("Option 2: Search ");
    println!("Option 3: Quit ");
    let mut selected_option = String::new();
    let mut filepath = String::new();
    io::stdin().read_line(&mut selected_option).expect("Failed to read input, please try again");
    if selected_option.trim() == "1" {
        // step 2.1
        // if its create index
        // ask for 
        // folder to index
        // files to exclude
        println!("you selected to create index, please enter the folder path");
        io::stdin().read_line(&mut filepath).expect("Failed to read the folder path, please try again");
        // hardcoded paths
        gruntwork::read_directory::read_directorycode(filepath);
        
    } else {
        // step 2.2
        // if its search ask for 
        // search text 
        // gruntwork::read_directory::databases();
        let mut search_str = String::new();
        println!("you selected search, please enter content to search");
        io::stdin().read_line(&mut search_str).expect("Failed to read the search string, please try again");
        println!("{}", search_str);
        gruntwork::read_index::read_index(search_str);
    }
}


