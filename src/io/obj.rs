use std::fs::File;
use std::io::BufReader;
use obj::{load_obj, Obj};


pub struct ObjParser {}

impl ObjParser {
    pub fn load(path:&str) {
        let input = BufReader::new(File::open(path).unwrap());
        let model: Obj = load_obj(input).unwrap();
        
        // Do whatever you want
        println!("{:#?}",model);
    }
}
