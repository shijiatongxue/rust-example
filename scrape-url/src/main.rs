use std::{ fs, env };
use reqwest::blocking;
use html2md::parse_html;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        panic!("give a url and output filename, 'cargo run <url> <filename>'")
    }
    let url = &args[1];
    let output = &args[2];

    println!("get url: {}", url);
    let body = blocking::get(url);

    let body = match body {
        Ok(file) => file.text(),
        Err(err) => {
            panic!("get url content error {}", err);
        }
    };

    println!("html2md...");
    let md = match body {
        Ok(body) => parse_html(&body),
        Err(err) => {
            panic!("parse content error {}", err);
        }
    };

    fs::write(output, md.as_bytes()).unwrap();
    println!("save markdown {}", output);
}
