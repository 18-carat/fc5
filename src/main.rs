mod config;
mod entry;
mod types;
mod writer;

use crate::config::Config;
use pico_args::Arguments;
use roxmltree::Document;
use std::fs::{read_to_string, File};
use std::io::{stdout, Write};

fn println(s: &'static str) {
    let mut stdout = stdout();
    stdout.write_all(s.as_bytes()).unwrap();
    stdout.write_all(b"\n").unwrap();
}

fn get_args() -> (String, String) {
    let mut args = Arguments::from_env();

    let input: Option<String> = args.value_from_str(["-i", "--input"]).ok();
    let config: Option<String> = args.value_from_str(["-c", "--config"]).ok();
    let help: bool = args.contains(["-h", "--help"]);

    if help || input.is_none() || config.is_none() {
        println("USAGE: fc5 --input <input> --config <config>\n");
        println("OPTIONS:");
        println("\t-i, --input <input>    The original XML compendium");
        println("\t-c, --config <config>  The configuration file");
        println("\t-h, --help             Show help information");

        std::process::exit(1);
    }

    (input.unwrap(), config.unwrap())
}

fn main() {
    let args = get_args();
    let config = Config::new(args.1).unwrap();
    let xml = read_to_string(&args.0).unwrap();
    let doc = Document::parse(&xml).unwrap();
    let root = doc.root_element();
    let out = String::from(&args.0[0..6]) + "_Curated.xml";
    let file = File::create(out).unwrap();

    writer::xml(root, config, file);
}
