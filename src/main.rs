#![allow(dead_code, unused)]
use std::{collections::HashMap};

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
    let input = "SET \"ahmet\" 23";
    let token_strings = input.split_whitespace().collect::<Vec<&str>>();
    let mut tokens: Vec<Token> = Vec::new();

    for token in token_strings {
        println!("\ngelen token = {}", token);
        match token {
            "SET" => tokens.push(Token::Commands(Commands::SET)),
            "GET" => tokens.push(Token::Commands(Commands::GET)),
            "DEL" => tokens.push(Token::Commands(Commands::DEL)),
            "ADD" => tokens.push(Token::Commands(Commands::ADD)),
            "SUB" => tokens.push(Token::Commands(Commands::SUB)),
            _ => {
                match token.get(..1) {
                    // None => 
                    Some("\"") => tokens.push(Token::Value(KVValue::STRING(token.to_string()))),
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
                    => tokens.push(Token::Value(KVValue::NUMBER(token.parse::<i64>().unwrap()))),
                    _ => println!("Unkonwn token.")
                }
            }
        }
    }

    println!("TOKENS = {:?}", tokens)
}

