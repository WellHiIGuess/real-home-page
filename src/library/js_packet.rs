#[derive(PartialEq)]
pub struct JSPacket {
    pub content: String,
    pub path: String,
}

impl JSPacket {
    // Takes in the file's name absolute path
    pub fn new(file_path: &str) -> JSPacket {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        let b_path = "src/js/".to_owned() + file_path;
        let path = Path::new(b_path.as_str());

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {} because {}", file_path, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read from {} because {}", file_path, why),
            Ok(_) => (),
        }
        
        JSPacket {
            content: s,
            path: file_path.to_string(),
        }
    }
}

impl ToString for JSPacket {
    fn to_string(&self) -> String {
        self.content.clone()
    }
}
