extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    let command_line_args: Vec<String> = env::args().collect();

    if command_line_args.len() != 3 {
        eprintln!("Usage: 'source' 'target'");
        return;
    }

    // Get full path to file

    let data_file_name = &command_line_args[1];

    let path: String = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let full_path = path + "/" + data_file_name;

    // Outcome file

    let outcome = File::create(env::args().nth(2).unwrap()).unwrap();

    // 0pen file

    let data_file = File::open(full_path).expect("No file found");

    // BufReader reading that file

    let mut file_buff_reader = BufReader::new(data_file);

    // Encoding file with GzEncode

    let mut encode = GzEncoder::new(outcome, Compression::default());

    // Started timer

    let start = Instant::now();

    // writing Encoded data to targe file

    copy(&mut file_buff_reader, &mut encode).unwrap();

    // Finished

    let output = encode.finish().unwrap();

    // Printing stats
    println!(
        "Source length: {:?}",
        file_buff_reader.get_ref().metadata().unwrap().len()
    );
    println!("Target length: {:?}", output.metadata().unwrap().len());

    println!("Elapsed: {:?}", start.elapsed());
}
