use std::fs;
use regex::Regex;

fn main() {
    
    let contents = fs::read_to_string("ez-error-logs.txt")
        .expect("Should have been able to read the file");
    
    let re = Regex::new(r"\n").unwrap();
    
    let decode = re.replace_all(&contents, "");

    println!("rust: {:?}", decode);
}
