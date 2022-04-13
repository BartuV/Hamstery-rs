use regex::Regex;
use std::fs;

pub fn run(filepath:String){
    parse(lex(read_file(filepath)))
}

fn read_file(filename:String) -> Vec<String>{
    let mut res = Vec::new();
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for i in content.split(" ") {
        if i.ends_with("\n"){
            res.push(String::from(i.replace("\n", "")));
            res.push(String::from("NewLine"))
        }else if i == ""{
            continue;
        }else{
            res.push(String::from(i))
        }
    }
    return res
}

fn lex(input:Vec<String>) -> Vec<Token>{
    let mut res: Vec<Token> = Vec::new();
    let mut line = 1;
    let patern = Regex::new("[0-9]").unwrap();
    let expresions: Vec<&str> = vec!["&&","!!","==","<=","=>","<",">"];

    for i in 0..input.len()-1{
        let cur = &input[i];
        if cur == "fn"{
            res.push(Token { token_type:"function".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }else if cur == "repeat"{
            res.push(Token { token_type:"repeat".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }else if cur == "NewLine"{
            res.push(Token { token_type:"newline".to_string(), token_index: i, token_line: line, token_value: cur.to_string() });line+=1;
        }else if patern.is_match(cur){
            res.push(Token { token_type:"number".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }else if cur.starts_with("/"){
            res.push(Token { token_type:"mcfunction".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }else if cur == "let"{
            res.push(Token { token_type:"variable".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }else if cur == "if"{
            res.push(Token { token_type:"if".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }else if expresions.contains(&cur.as_str()){
            res.push(Token { token_type:"expresion".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }else{
            res.push(Token { token_type:"identifier".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }
    }
    return res
}

fn parse(input:Vec<Token>){
    for i in input{
        
    }
}

///Token needed for lexing and parsing this is critical part of the lexer and the parser.
#[derive(Debug)]
struct Token{
    token_type:  String,
    token_index: usize,
    token_line:  usize,
    token_value: String
}