use std::env::{args, var};

mod openai;
use crate::openai::request::request_openai;

fn main() {
    let args: Vec<String> = args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Usage: {} <question>", args[0]);
    }

    let mut prompt: String =
        String::from("Answer precisely, ideally with one word, command or line of code, but prioritize accuracy.");

    if args[0].contains("whatis") {
        prompt.insert_str(0, &format!("What is {}? ", args[1..].join(" ")));
    } else if args[0].contains("howto") {
        prompt.insert_str(0, &format!("How to {}? ", args[1..].join(" ")));
    }

    match var("OPENAI_API_KEY") {
        Ok(api_key) => match request_openai(api_key, prompt) {
            Ok(response) => println!("\n{}\n", response),
            Err(err) => panic!("{}", err),
        },
        Err(_) => {
            panic!("OPENAI_API_KEY environment variable is not set!");
        }
    }
}
