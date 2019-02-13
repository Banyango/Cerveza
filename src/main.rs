extern crate clap;
extern crate csv;
#[macro_use]
extern crate serde_json;

use clap::{App, Arg};
use std::path::Path;
use std::ffi::OsStr;
use csv::{StringRecord};
use serde_json::{Value, json};
use std::io::prelude::*;

fn main() {
    let matches = App::new("cerveza")
            .version("1.0")
            .author("Kyle Reczek <kyle@banyango.com>")
            .about("Converts CSV files to JSON")
            .arg(Arg::with_name("inputfile")                        
                .required(true)
                .help("CSV file to convert")
                .index(1))
            .arg(Arg::with_name("delimiter")                        
                .takes_value(true)
                .short("d")         
                .possible_values(&["tab", "space", "comma"])               
                .default_value("tabs")
                .long("delimiter")                        
                .help("the delimiter for the csv file")
                )
    .get_matches();
            
    let input_file_name = matches.value_of("inputfile").unwrap();
    
    let delimiter = matches.value_of("delimiter").unwrap();
    
    let mut _byte_delim = b'\t';

    match delimiter {
        "tab" => _byte_delim = b'\t',
        "space" => _byte_delim = b' ',
        "comma" => _byte_delim = b':',
        _ => _byte_delim = b' ',
    }

    let path = std::env::current_dir().unwrap().join(input_file_name);

    let file_to_convert = Path::new(&path);

    println!("Converting file = {}", file_to_convert.display());

    if let Err(e) = parse_csv(&path, _byte_delim) {
        println!("Application error: {}", e);
        std::process::exit(1);
    }
}

fn parse_csv(file_to_convert: &Path, delimiter: u8) -> Result<(), std::io::Error> {
    if file_to_convert.extension().unwrap() == OsStr::new("csv") {        
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .flexible(true).from_path(file_to_convert)?;                           
        
        let headers = rdr.headers().unwrap().clone();

        let mut parent : Vec<Value> = Vec::new();

        for result in rdr.records() {
            let record : StringRecord = result?;

            let mut json_record = serde_json::map::Map::new();
                        
            for (i, item) in record.into_iter().enumerate() {                
                if let Some(header) = headers.get(i) {
                    let value = item.to_string();                    

                    json_record.insert(header.to_string(), json!(value));
                };
            }

            parent.push(json!(json_record));
        } 

        let new_file_name = [file_to_convert.file_stem().unwrap().to_str().unwrap(),".json"].concat();
        
        let path = Path::new(&new_file_name);
        if path.exists() {
            std::fs::remove_file(path)?;
        }

        let mut file = std::fs::OpenOptions::new()
                            .write(true)                            
                            .create_new(true)
                            .open(&new_file_name)?;
        
        let output_json = json!(parent);

        file.write_all(serde_json::to_string_pretty(&output_json).unwrap().as_bytes())?;

        println!("File Converted = {}", path.display());
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "File extension is not csv"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tab() {
        
    }
}
