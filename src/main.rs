use clap::Parser;
use std::{fs::read_to_string, ptr::read};
use cryptoys;

#[derive(Parser)]
pub struct Args{
    pub filename: std::path::PathBuf,
    pub key: String,
    
}

fn main(){
    let args = Args::parse();

    if !args.filename.to_str().unwrap().is_empty() {
        // convert a string to a path
       let stuff = read_to_string(args.filename).unwrap(); 
       let encrypt = cryptoys::historical::playfair::encrypt(&stuff, &args.key);

       println!("{encrypt}"); 
    }
}