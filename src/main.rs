#![allow(dead_code, unused)]
use std::{collections::HashMap, io::{self, Write}};

#[derive(Debug, Clone)]
enum Token {
    Commands(Commands),
    Value(KVValue)
}

#[derive(Debug, Clone)]
enum Commands {
    GET,
    SET,
    DEL,
    ADD,
    SUB,
}

#[derive(Debug, Clone)]
enum KVValue {
    NUMBER(i64),
    STRING(String),
    BOOL(bool)
}

// SET "ahmet" 23
//
// SET 
// "ahmet" 
// 23
//
// [SET STRING("ahmet") NUMBER(23)]
fn parse_string(input: String) -> Vec<Token> {
    // TODO: "Ahmet Tarik" 2 token olarak aliyor. Bu tek Token olmali.
    // Atduyar
    let token_strings = input.split_whitespace().collect::<Vec<&str>>();
    let mut tokens: Vec<Token> = Vec::new();

    for token in token_strings {
        match token {
            "SET" => tokens.push(Token::Commands(Commands::SET)),
            "GET" => tokens.push(Token::Commands(Commands::GET)),
            "DEL" => tokens.push(Token::Commands(Commands::DEL)),
            "ADD" => tokens.push(Token::Commands(Commands::ADD)),
            "SUB" => tokens.push(Token::Commands(Commands::SUB)),
            "true"  => tokens.push(Token::Value(KVValue::BOOL(true))),
            "false" => tokens.push(Token::Value(KVValue::BOOL(false))),
            _ => {
                let first_char = token.chars().next();
                match first_char {
                    None => {
                        continue;
                    },
                    Some('"') => {
                        let string_literal = token.chars().skip(1).take_while(|&c| c != '"').collect();
                        tokens.push(Token::Value(KVValue::STRING(string_literal)));
                        continue;
                    }
                    Some(c) if c.is_ascii_digit() || c == '-' || c == '+' => {
                        match token.parse::<i64>() {
                            Ok(num) => {
                                let token = Token::Value(KVValue::NUMBER(num));
                                tokens.push(token);
                            }
                            Err(_) => {
                                println!("Error: '{}' is not a valid number.", token);
                            }
                        }
                        continue;
                    }
                    Some(_) => {
                        println!("Error: Unrecognized token '{}'.", token);
                        break;
                    }
                }
            }
        }
    }
    return tokens;
}


// SET "ahmet" 42
// SET <string> <KVValue>
// GET <string>
// DEL <string>
// ADD <string>
// ADD <string> <Number>
fn interpret_tokens(db: &mut HashMap<String, KVValue>, tokens: Vec<Token>){
    match tokens.get(0) {
        None => {
            println!("ERROR: Command is empty");
            return;
        }
        Some(Token::Value(_)) =>  {
            println!("ERROR: First token cannot be a Value");
            return;
        }
        Some(Token::Commands(Commands::GET)) =>  {
            if let Some(Token::Value(KVValue::STRING(s))) = tokens.get(1) {
                // TODO: print the value, not the data structure (eg. 1 instead of Token(Number(1))) 
                // Kayra
                println!("= {:?}", db.get(s));
            }
            else {
                println!("Usage: GET <String>")
            }
        }
        Some(Token::Commands(Commands::SET)) =>  {
            if let Some(Token::Value(KVValue::STRING(s))) = tokens.get(1) && let Some(Token::Value(v)) = tokens.get(2) {
                db.insert(s.clone(), v.clone());
                println!("OK");
            }
            else {
                println!("Usage: SET <String> <Value>")
            }
        }
        Some(Token::Commands(Commands::DEL)) =>  {
            if tokens.len() > 1 {
                match &tokens[1] {
                    Token::Value(KVValue::STRING(key)) => {
                        db.remove(key);
                        println!("Deleted key: {}", key);
                    }
                    _ => println!("Error: Expected a string value"),
                }
            }
            else {
                println!("Usage: DEL <key>");
            }
        }
        Some(Token::Commands(Commands::ADD)) =>  {
            if let Some(Token::Value(KVValue::STRING(s))) = tokens.get(1) {
                let n = match tokens.get(2) {
                    Some(Token::Value(KVValue::NUMBER(num))) => *num,
                    _ => 1,
                };
                if let Some(KVValue::NUMBER(num)) = db.get(s) {
                    db.insert(s.clone(), KVValue::NUMBER(num + n));
                    println!("OK");
                }
            }
            else {
                println!("Usage: ADD <String> [Number]")
            }
        }
        Some(Token::Commands(Commands::SUB)) =>  {
            if let Some(Token::Value(KVValue::STRING(s))) = tokens.get(1) {
                let n = match tokens.get(2) {
                    Some(Token::Value(KVValue::NUMBER(num))) => *num,
                    _ => 1,
                };
                if let Some(KVValue::NUMBER(num)) = db.get(s) {
                    db.insert(s.clone(), KVValue::NUMBER(num - n));
                    println!("OK");
                }
            }
            else {
                println!("Usage: SUB <String> [Number]")
            }
        }
    }
}

fn main() {
    let mut db: HashMap<String, KVValue> = HashMap::new();

    loop {
        let mut input = String::new();
        print!(">> ");
        io::stdout().flush().expect("Failed to flush stdout");


        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nGoodbye (end of file detected)");
                break;
            }
            Err(e) => {
                eprintln!("Error : {}", e);
                break;
            }
            Ok(_) => ()
        }

        if (input.trim() == ".quit") {
            break;
        }

        let tokens = parse_string(input);

        println!("TOKENS = {:?}", tokens);

        interpret_tokens(&mut db, tokens);
    }
}

