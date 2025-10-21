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
                for (line_num, line) in file_contents.split('\n').enumerate() {
                    let mut skip = false;
                    let mut skip_string = 0;
                    for (i, c) in line.chars().enumerate() {
                        if skip {
                            skip = false;
                            continue;
                        }
                        if skip_string > 0 {
                            skip_string -= 1;
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
                                if i + 1 < line.len() && line.chars().nth(i+1).unwrap() == '=' {
                                    println!("EQUAL_EQUAL == null");
                                    skip = true;
                                } else {
                                    println!("EQUAL = null")
                                }
                            },
                            '!' => {
                                if i + 1 < line.len() && line.chars().nth(i+1).unwrap() == '=' {
                                    println!("BANG_EQUAL != null");
                                    skip = true;
                                } else {
                                    println!("BANG ! null")
                                }
                            },
                            '<' => {
                                if i + 1 < line.len() && line.chars().nth(i+1).unwrap() == '=' {
                                    println!("LESS_EQUAL <= null");
                                    skip = true;
                                } else {
                                    println!("LESS < null")
                                }
                            },
                            '>' => {
                                if i + 1 < line.len() && line.chars().nth(i+1).unwrap() == '=' {
                                    println!("GREATER_EQUAL >= null");
                                    skip = true;
                                } else {
                                    println!("GREATER > null")
                                }
                            },
                            '/' => {
                                if i + 1 < line.len() && line.chars().nth(i+1).unwrap() == '/' {
                                    break;
                                } else {
                                    println!("SLASH / null")
                                }
                            },
                            ' ' => (),
                            '\t' => (),
                            '"' => {
                                let literal = line.chars().skip(i+1).take_while(|c| *c != '"').collect::<String>();
                                skip_string = literal.len() + 1;
                                if i + skip_string >= line.len() {
                                    eprintln!("[line {}] Error: Unterminated string.", line_num + 1);
                                    found_lexical_error = true;
                                } else {
                                    println!("STRING \"{}\" {}", literal, literal);
                                }
                            },
                            other_char => {
                                eprintln!("[line {}] Error: Unexpected character: {}", line_num + 1, other_char);
                                found_lexical_error = true;
                            },
                        }
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
