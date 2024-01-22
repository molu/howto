use std::env::{args, var};

mod openai;
use crate::openai::request::request_openai;

fn main() {
    let mut args: Vec<String> = args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Usage: {} <question>", args[0]);
    }

    // check if verbose flag is set, if so, remove it from args vector
    let mut verbose: bool = false;
    match args.iter().position(|x| *x == "-v" || *x == "--verbose") {
        Some(index) => {
            args.remove(index);
            verbose = true;
        }
        None => {}
    }

    // prepare GPT-4 prompt
    let mut prompt: String = String::from("Answer precisely, ideally with one word, command or line of code without any intro or explanation. Prioritize accuracy.");

    if verbose {
        prompt = String::from(
            "Answer precisely, add a few word of explanation with one most accurate example. Prioritize accuracy.",
        );
    }

    if args[0].contains("whatis") {
        prompt.insert_str(0, &format!("What is {}? ", args[1..].join(" ")));
    } else if args[0].contains("howto") {
        prompt.insert_str(0, &format!("How to {}? ", args[1..].join(" ")));
    }

    // request OpenAI API (if OPENAI_API_KEY env variable is set) and print response
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
