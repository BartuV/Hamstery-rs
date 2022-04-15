/*
    Made by BartuV
    github: https://github.com/BartuV
*/
use colored::Colorize;
use regex::Regex;
use std::{fs, io::Write, path::Path, str::FromStr};
use serde_json::*;

const PACK_INFO_NAME:&str = r"\packinfo.json";

pub fn run(filepath:String){
    parse(lex(read_file(&get_main_hamy_file(&filepath))),&filepath)
}

fn read_file(filename:&String) -> Vec<String>{
    let mut res:Vec<String> = Vec::new();
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file".red().bold().to_string().as_str());
    
    for i in content.lines() {
        for n in i.split(" ") {
            if !(n == ""){
                res.push(n.to_string());
            }
        }
        res.push("NewLine".to_string());
    }
    return res
}

fn lex(input:Vec<String>) -> Vec<Token>{
    let mut res: Vec<Token> = Vec::new();
    let mut line = 1;
    let patern = Regex::new("[0-9]").unwrap();
    let expresions: Vec<&str> = vec!["&&","!!","==","<=","=>","<",">"];
    let math: Vec<&str> = vec!["=","+","-","*","/"];

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
        }else if cur == "global"{
            res.push(Token { token_type:"globalint".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }else if math.contains(&cur.as_str()){
            res.push(Token { token_type:"mathematical".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }else if cur == "end"{
            res.push(Token { token_type:"end".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }
        else{
            res.push(Token { token_type:"identifier".to_string(), token_index: i, token_line: line, token_value: cur.to_string() })
        }
    }
    return res
}

fn parse(input:Vec<Token>,filepath:&String){
    let mut varlist:Vec<Variable> = Vec::new();
    for i in 0..input.len()-1{
        let cur = &input[i];

        //Variable detection
        if cur.token_type == "variable"{
            let name = input[i+1].token_value.replace("=","");
            let mut start = i + 2;
            if input[start].token_value == "="{
                start += 1;
            }
            let end = get_nearest_newline_index(i,&input);
            let value:Vec<Token> = input[start..end].to_vec();
            let mut string = String::new();
            for n in value.iter(){
                string += &n.token_value.replace("=","");
                string += ""
            }
            let valstruct:Variable = Variable { name, value:string};
            varlist.push(valstruct);
        }

        if cur.token_type == "function"{
            let content = std::fs::read_to_string(filepath.to_owned()+PACK_INFO_NAME).expect("Error: Can't read file".red().bold().to_string().as_str());
            let json = Value::from_str(&content).expect("Error: Can't read file".red().bold().to_string().as_str());
            let function_name = &(input[i+1].token_value).replace("(","").replace(")","");
            let function_path = format!("{}{}{}{}",filepath,json["functionfolder"].to_string().replace("/",r"\").replace('"',"").as_str(),r"\" ,function_name.to_owned()+".mcfunction");
            let mut file = create_file(&function_path);
            let start = i + 2;
            let content = function_contents(start, &input,&varlist);
            file.write_all(content.replace("/", "").as_bytes()).unwrap();
        }
    }
}

fn function_contents(start: usize,input:&Vec<Token>,varlist: &Vec<Variable>) -> String {
    let mut end = 0;
    let mut res = String::new();

    for i in start..input.len(){
        if input[i].token_type == "end"{
            end = i;
            break
        }
    }

    for i in start..end{
        let cur = &input[i];
        if cur.token_type == "newline"{continue}

        if varlist.is_empty(){
            res += cur.token_value.as_str();
            res += &" ".to_string()
        }

        for b in varlist{
            if &b.name == &cur.token_value{
                res += &b.value;
            }else{
                res += cur.token_value.as_str();
                res += " ";
            }
        }
    }

    return res;
}

fn get_nearest_newline_index(start:usize,input:&Vec<Token>)->usize{
    let mut res = 0;

    for i in start..input.len(){
        let cur = &input[i];
        if cur.token_type == "newline"{
            res = i;
            break
        }else{
            continue;
        }
    }

    return res;
}

fn create_file(filepath:&String) -> fs::File{
    let file = fs::File::create(filepath.to_owned()).expect("Error: Can't read file".red().bold().to_string().as_str());
    return file;
}

fn get_main_hamy_file(filepath:&String)->String{
    let f = filepath.to_owned()+r"\indexfiles\main.hamy";
    let path = Path::new(&f);
    return path.to_str().unwrap().to_string();
}

////////Structs////////
///Token needed for lexing and parsing this is critical part of the lexer and the parser.
#[derive(Debug, Clone)]
struct Token{
    token_type:  String,
    token_index: usize,
    token_line:  usize,
    token_value: String
}

#[derive(Debug, Clone)]
struct Variable{
    name:String,
    value:String,
}

/////////Traits\\\\\\\\\
impl Variable {
    fn new(name:&str, value:&str) -> Variable {
        Variable{name:name.to_string(),value:value.to_string()}
    }
}