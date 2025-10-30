#![allow(dead_code, unused)]

#[derive(Debug, Clone)]
pub enum Token {
    Commands(Commands),
    Value(KVValue),
}

#[derive(Debug, Clone)]
pub enum Commands {
    GET,
    SET,
    DEL,
    ADD,
    SUB,
}

#[derive(Debug, Clone)]
pub enum KVValue {
    NUMBER(i64),
    STRING(String),
    BOOL(bool),
}
