#![allow(dead_code, unused)]
use crate::kv_value::*;

// SET "ahmet" 23
//
// SET
// "ahmet"
// 23
//
// [SET STRING("ahmet") NUMBER(23)]
pub fn parse_string(input: String) -> Vec<Token> {
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
            "true" => tokens.push(Token::Value(KVValue::BOOL(true))),
            "false" => tokens.push(Token::Value(KVValue::BOOL(false))),
            _ => {
                let first_char = token.chars().next();
                match first_char {
                    None => {
                        continue;
                    }
                    Some('"') => {
                        let string_literal =
                            token.chars().skip(1).take_while(|&c| c != '"').collect();
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
