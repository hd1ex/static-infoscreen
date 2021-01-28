use std::fs;

use clap::{App, Arg};

pub mod static_infoscreen;

const PROGRAM_NAME: &str = "Static HTML Infoscreen Generator";
const PROGRAM_DESC: &str = "Generate html files for a static infoscreen";
const CONFIG_FILE: &str = "infoscreen.conf";
const OUTPUT_DIR: &str = "html/";

fn main() {
    let matches = App::new(PROGRAM_NAME)
        .author("Alexander Sommer <dev@alexandersommer.eu>")
        .about(PROGRAM_DESC)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .default_value(CONFIG_FILE),
        )
        .arg(
            Arg::with_name("output-dir")
                .short("o")
                .long("output-dir")
                .value_name("DIR")
                .help("Sets a custom output directory")
                .takes_value(true)
                .default_value(OUTPUT_DIR),
        )
        .get_matches();

    let config_file = matches.value_of("config").unwrap();
    let output_dir = matches.value_of("output-dir").unwrap();

    let config = static_infoscreen::Config::parse_from(config_file);

    fs::create_dir_all(output_dir).expect("Can not create output directory.");
    config.create_html(output_dir);
}
