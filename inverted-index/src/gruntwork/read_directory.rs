use std::io::Read;
use std::fs;
use crate::gruntwork::create_index::create_index;

pub fn read_directorycode(filepath: String) {
    let paths = fs::read_dir(filepath.trim()).unwrap();
    for path in paths {
        let filename = path.unwrap().path();
        let filename_temp = filename.clone();
        let mut contents = String::new();
        let filename_str = filename_temp.to_str().expect("error changing filename temp to string");
        let fileresult = std::fs::File::open(filename);
        match fileresult {
            Ok(file) => {
                let mut file_value = file;
                println!("filename_temp {:?}", filename_temp);
                if filename_str.contains(".html") {
                    file_value.read_to_string(&mut contents).expect("error reading file to string");
                    create_index(contents, &filename_str);
                }
            }
            Err(e) => println!("error {:?}", e),
        }
    }
}
