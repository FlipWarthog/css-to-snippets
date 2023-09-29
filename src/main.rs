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

#[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
struct Snippet<'a> {
    prefix: &'a str,
    body: Vec<&'a str>,
    description: &'a str,
}

fn main() {
    let args = Args::parse();

    // Read the content, expected to be minified, and fix escaped : wherever it appears
    let contents = fs::read_to_string(args.in_file)
        .expect("Could not read input file!")
        .replace("\\:", ":");

    let output = parse_css(&contents);

    fs::write(
        args.out_file,
        serde_json::to_string_pretty(&output).expect("Could not serialize JSON!"),
    )
    .expect("Could not write output!");
}

fn parse_css(contents: &str) -> HashMap<&str, Snippet<'_>> {
    // Regex for parsing "normal" classes and their content, e.g. what's in the curly braces
    let re = regex::Regex::new("[ ]*[.]([a-zA-Z0-9-]+[:]?[a-zA-Z0-9-]+)[ ]?[{][ ]*([ a-zA-Z0-9;:!%.-]*[a-zA-Z0-9;:!%.-]+)[ ]*[}]").unwrap();
    let mut output = HashMap::new();

    for cap in re.captures_iter(contents) {
        let key = cap.get(1).unwrap().as_str();
        let val = cap.get(2).unwrap().as_str();
        output.insert(
            key,
            Snippet {
                prefix: key,
                body: vec![key],
                description: val,
            },
        );
    }

    output
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{parse_css, Snippet};

    #[test]
    fn parse_css_test() {
        let input = ".class{border-radius: 1px;} .md:mb-1{margin-bottom: 1rem;}";

        let result = parse_css(input);

        assert!(result.contains_key("class"));
        assert!(result.contains_key("md:mb-1"));

        if let Some(class_snip) = result.get("class") {
            assert_eq!(
                class_snip,
                &Snippet {
                    prefix: "class",
                    body: vec!["class"],
                    description: "border-radius: 1px;"
                }
            );
        } else {
            panic!("Result is missing \"class\"");
        }

        if let Some(md_snip) = result.get("md:mb-1") {
            assert_eq!(
                md_snip,
                &Snippet {
                    prefix: "md:mb-1",
                    body: vec!["md:mb-1"],
                    description: "margin-bottom: 1rem;"
                }
            );
        } else {
            panic!("Result is missing \"md:mb-1\"");
        }
    }

    #[test]
    fn parse_css_invalid() {
        let input = "invalid css";

        let result = parse_css(input);

        assert_eq!(result, HashMap::new());
    }
}
