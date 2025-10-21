use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let mut found_lexical_error = false;

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut skip = false;
                for (i, c) in file_contents.chars().enumerate() {
                    if skip {
                        skip = false;
                        continue;
                    }
                    match c {
                        '(' => println!("LEFT_PAREN ( null"),
                        ')' => println!("RIGHT_PAREN ) null"),
                        '{' => println!("LEFT_BRACE {{ null"),
                        '}' => println!("RIGHT_BRACE }} null"),
                        ',' => println!("COMMA , null"),
                        '.' => println!("DOT . null"),
                        '-' => println!("MINUS - null"),
                        '+' => println!("PLUS + null"),
                        ';' => println!("SEMICOLON ; null"),
                        '*' => println!("STAR * null"),
                        '=' => {
                            if i + 1 < file_contents.len() && file_contents.chars().nth(i+1).unwrap() == '=' {
                                println!("EQUAL_EQUAL == null");
                                skip = true;
                            } else {
                                println!("EQUAL = null")
                            }
                        },
                        '!' => {
                            if i + 1 < file_contents.len() && file_contents.chars().nth(i+1).unwrap() == '=' {
                                println!("BANG_EQUAL != null");
                                skip = true;
                            } else {
                                println!("BANG ! null")
                            }
                        },
                        other_char => {
                            eprintln!("[line 1] Error: Unexpected character: {}", other_char);
                            found_lexical_error = true;
                        },
                    }
                }
                println!("EOF  null");
            } else {
                println!("EOF  null"); // Placeholder, replace this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
    if found_lexical_error {
        process::exit(65);
    }
}
