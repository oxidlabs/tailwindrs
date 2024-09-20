// we are building a tailwindcss clone in rust
//
// for now we just want to parse a .html file and extract the classes from it

use lightningcss::printer::PrinterOptions;
use lightningcss::stylesheet::{MinifyOptions, StyleSheet};
use lightningcss::stylesheet::ParserOptions;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read};

const TAILWIND_BASE: &str = include_str!("../tailwind_base.css");

fn extract_quoted_strings(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut in_double_quotes = false;
    let mut in_single_quotes = false;
    let mut current_string = String::new();

    for c in input.chars() {
        match c {
            '"' if !in_single_quotes => {
                if in_double_quotes {
                    result.push(current_string.trim().to_string());
                    current_string = String::new();
                }
                in_double_quotes = !in_double_quotes;
            }
            '\'' if !in_double_quotes => {
                if in_single_quotes {
                    result.push(current_string.trim().to_string());
                    current_string = String::new();
                }
                in_single_quotes = !in_single_quotes;
            }
            _ if in_double_quotes || in_single_quotes => {
                if c != '"' && c != '\'' && c != '\r' && c != '\n' {
                    current_string.push(c);
                }
            }
            _ => {}
        }
    }

    result
}

fn main() {
    let file = File::open("index.html").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();

    let start = std::time::Instant::now();

    let mut classes: HashSet<String> = HashSet::new();

    let res = extract_quoted_strings(&buffer);
    for r in res {
        let split = r.split(" ");
        for s in split {
            classes.insert(s.to_string());
        }
    }

    println!("Time taken: {:?}", start.elapsed());
    println!("{:?}", classes);

    let start_time = std::time::Instant::now();
    let file = File::open("styles/globals.css").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();

    let mut stylesheet = StyleSheet::parse(&buffer, ParserOptions::default()).unwrap();
    stylesheet.minify(MinifyOptions::default()).unwrap();
    let mut res = stylesheet.to_css(PrinterOptions::default()).unwrap();
    
    if res.code.contains("@tailwind base;") {
        //replace @tailwind base; with TAILWIND_BASE
        res.code = res.code.replace("@tailwind base;", TAILWIND_BASE);
    }

    //let final_css = replace_whitespace(&res.code);
    println!("Time taken: {:?}", start_time.elapsed());
    //println!("{}", res.code);

    //let tokens = shlex::split(&buffer).unwrap();

    /* for (i, token) in tokens.iter().enumerate() {
        if token.contains("class") {
            let mut token = token.to_string();

            // checking the token isnt something like "class"
            if !token.contains("class=") {
                let next_token = tokens.get(i + 1).unwrap();
                if !next_token.contains("=") {
                    continue;
                }

                if next_token.len() > 1 {
                    let next_token = tokens.get(i + 1).unwrap();
                    token = format!("{}{}", token, next_token);
                } else {
                    let next_token = tokens.get(i + 2).unwrap();
                    token = format!("{}={}", token, next_token);
                }
            }

            // remove any consecutive = signs weather its == or === with nothing
            token = token.replace("===", "");
            token = token.replace("==", "");
            let mut split = token.split("=");
            let input = split.nth(1).unwrap();

            // check if class has single quotes and only grab those
            let mut class: &str = if input.contains("'") {
                let re = Regex::new(r"'(.*?)'").unwrap();

                &re.find_iter(input)
                    .map(|m| {
                        // remove the quotes
                        let mut _class = m.as_str();
                        _class = _class.trim_matches('\'');
                        _class
                    })
                    .collect::<Vec<&str>>()
                    .join(" ")
            } else {
                input
            };

            if class.contains(">") {
                let mut split = class.split(">");
                class = split.nth(0).unwrap();
            }

            class = class.trim();
            class = class.trim_matches('\'');
            class = class.trim_matches('"');
            let split = class.split(" ");
            for c in split {
                classes.insert(c.to_string());
            }
        }
    } */

    /* let start_time = std::time::Instant::now();

    let fp = FileProvider::new();
    let mut bundler = Bundler::new(&fp, None, ParserOptions::default());
    let stylesheet = bundler.bundle(Path::new("styles/globals.css")).unwrap();
    let res = stylesheet.to_css(PrinterOptions::default()).unwrap();
    let mut code = res.code;

    println!("Time taken to bundle: {:?}", start_time.elapsed());

    if code.contains("@tailwind base;") {
        //replace @tailwind base; with TAILWIND_BASE
        let re = Regex::new(r"@tailwind base;").unwrap();
        code = re.replace_all(&code, TAILWIND_BASE).to_string();
    }

    println!("Time taken to replace base: {:?}", start_time.elapsed());
    let mut stylesheet = StyleSheet::parse(&code, ParserOptions::default()).unwrap();
    //stylesheet.minify(MinifyOptions::default()).unwrap();
    let res = stylesheet.to_css(PrinterOptions::default()).unwrap();
    println!("Time taken to minify: {:?}", start_time.elapsed());
    //let final_css = replace_whitespace(&res.code);
    println!("Time taken to replace whitespace: {:?}", start_time.elapsed()); */

    //println!("{}", res.code);
    //println!("Time taken: {:?}", start_time.elapsed());
}

fn replace_whitespace(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars() {
        // Check if the character is not a whitespace or newline
        if !c.is_whitespace() {
            result.push(c);
        }
    }

    result
}
