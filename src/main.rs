use csv;
use std::{env, error::Error, path::Path, process};



fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        process::exit(0);
    }

    let filename = Path::new(&args[1]);
    let _ = read_from_file(filename.to_str().unwrap_or(""));
}


fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;

        println!("{:?}", record);
    }

    Ok(())
}