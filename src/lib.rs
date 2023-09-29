use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}


pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Roman Popov  <example@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file names")
                .required(true)
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .help("Number lines")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .help("Number nonblank lines")
                .takes_value(false)
        )
        .get_matches();
    let files = matches.value_of_lossy("files")
                                .unwrap()
                                .split(" ")
                                .map(|file_name| file_name.to_owned())
                                .collect::<Vec<String>>();

    Ok(Config { files: files, 
        number_lines: matches.is_present("number_lines"), 
        number_nonblank_lines: matches.is_present("number_nonblank_lines") })
}