use std::{fs::{self, File}, io::{BufReader, Read, Write}, path::Path};



pub fn main(){
    let file1 = File::open("Cargo.toml").unwrap();
    let mut buf_reader = BufReader::new(file1);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);


    let path = Path::new(&contents);
    
    // let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    // let mut file = match File::open(&path) {
    //     Err(why) => panic!("couldn't open {}: {}", display, why),
    //     Ok(file) => println!("ok"),
    
    let serialized = serde_yaml::to_string(&path).unwrap();
    let mut file = File::create("cargo.yaml").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    
    // println!("{:?}",serialized);
    // let contents = fs::read_to_string(serialized).unwrap();

    // println!("With text:\n{}", contents);

}