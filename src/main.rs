#![allow(dead_code, unused)]
use std::{collections::HashMap, io::{self, Write}};

// <function> <string> <KVValue>
// SET "ahmet" 23
//
// SET 
// "ahmet" 
// 23
//
// [SET STRING("ahmet") NUMBER(23)]
#[derive(Debug)]
enum Token {
    Commands(Commands),
    Value(KVValue)
}

#[derive(Debug)]
enum Commands {
    GET,
    SET,
    DEL,
    ADD,
    SUB,
}

#[derive(Debug)]
enum KVValue {
    NUMBER(i64),
    STRING(String),
    BOOL(bool)
}

fn main() {
    loop {
        let mut input = String::new();
        print!(">> ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).unwrap();

        if (input.trim() == ".quit") {
            break;
        }

        // TODO: "ahmet Tarik" 2 token olarak aliyor. Bu tek Token olmali.
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
                    match token.get(..1) {
                        Some("\"") => {
                            let mut string_literal = token.to_string();
                            if token.chars().last().unwrap_or(' ') != '"' {
                                println!("Invalid String.");
                                break;
                            }
                            tokens.push(Token::Value(KVValue::STRING(string_literal.trim_matches(&['"'] as &[_]).to_string())));
                        },
                        Some("0")|
                        Some("1")|
                        Some("2")|
                        Some("3")|
                        Some("4")|
                        Some("5")|
                        Some("6")|
                        Some("7")|
                        Some("8")|
                        Some("9")
                        => {
                            let token = Token::Value(KVValue::NUMBER(token.parse::<i64>().unwrap()));
                            tokens.push(token)
                        },
                        _ => println!("Unkonwn token.")
                    }
                }
            }
        }

        println!("TOKENS = {:?}", tokens)
            
    }
}

