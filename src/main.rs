use std::{collections::HashMap, fs};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    in_file: String,
    #[arg(short, long)]
    out_file: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Snippet {
    prefix: String,
    body: Vec<String>,
    description: String,
}

fn main() {
    let args = Args::parse();

    // Regex for parsing "normal" classes and their content, e.g. what's in the curly braces
    let re = regex::Regex::new("[ ]*[.]([a-zA-Z0-9-]+[:]?[a-zA-Z0-9-]+)[ ]?[{][ ]*([ a-zA-Z0-9;:!%.-]*[a-zA-Z0-9;:!%.-]+)[ ]*[}]").unwrap();

    // Read the content, expected to be minified, and fix escaped : wherever it appears
    let contents = fs::read_to_string(args.in_file)
        .expect("Could not read input file!")
        .replace("\\:", ":");

    let mut output = HashMap::new();

    for cap in re.captures_iter(&contents) {
        let key = &cap[1];
        let val = &cap[2];
        output.insert(
            key.to_string(),
            Snippet {
                prefix: key.to_string(),
                body: vec![key.to_string()],
                description: val.to_string(),
            },
        );
    }

    fs::write(
        args.out_file,
        serde_json::to_string_pretty(&output).expect("Could not serialize JSON!"),
    )
    .expect("Could not write output!");
}
