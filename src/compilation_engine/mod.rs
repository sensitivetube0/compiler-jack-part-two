
use crate::compilation_engine::Token::{Class, Identifier, Keyword, Symbol};
use crate::handle_file_or_dir_given::{FilesInPathBuf};
use std::io::{self, BufRead, BufReader,Write,Read,Error};
use std::fs::File;
use std::vec;
use crate::symbol_table::{SymbolTable};

#[derive(Debug)]
pub struct CompilationEngine{
    files:FilesInPathBuf,
    current_file:BufReader<File>,
    next_file_index:u32,
    symbol_table:SymbolTable,
    root_token:Token,
}   





#[derive(Debug, Clone)]
pub enum Token {
    Class(Vec<Box<Token>>),
    ClassVarDec(Vec<Box<Token>>),
    SubRoutineDec(Vec<Box<Token>>),
    ParameterList(Vec<Box<Token>>),
    SubRoutineBody(Vec<Box<Token>>),
    VarDec(Vec<Box<Token>>),
    Statements(Vec<Box<Token>>),
    ReturnStatement(Vec<Box<Token>>),
    Keyword(Vec<Box<Token>>),
    KeywordName(String),
    Identifier(Vec<Box<Token>>),
    IdentifierName(String),
    Symbol(Vec<Box<Token>>),
    SymbolName(String),
}


impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Structural containers evaluated by variant type only
            (Token::Class(_), Token::Class(_)) => true,
            (Token::ClassVarDec(_), Token::ClassVarDec(_)) => true,
            (Token::SubRoutineDec(_), Token::SubRoutineDec(_)) => true,
            (Token::ParameterList(_), Token::ParameterList(_)) => true,
            (Token::SubRoutineBody(_), Token::SubRoutineBody(_)) => true,
            (Token::VarDec(_), Token::VarDec(_)) => true,
            (Token::Statements(_), Token::Statements(_)) => true,
            (Token::ReturnStatement(_), Token::ReturnStatement(_)) => true,
            (Token::Keyword(_), Token::Keyword(_)) => true,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (Token::Symbol(_), Token::Symbol(_)) => true,

            // Named variants compared by their string inner values
            (Token::KeywordName(s1), Token::KeywordName(s2)) => s1 == s2,
            (Token::IdentifierName(s1), Token::IdentifierName(s2)) => s1 == s2,
            (Token::SymbolName(s1), Token::SymbolName(s2)) => s1 == s2,

            // Fallback catch-all to prevent stack overflows
            _ => false,
        }
    }
}



impl Token {

    
        
    


    pub fn add_token(&mut self,box_token:Box<Token>){
        match self{

        Token::Class(vec) |
        Token::ClassVarDec(vec) |
        Token::SubRoutineDec(vec) |
        Token::ParameterList(vec) |
        Token::SubRoutineBody(vec) |
        Token::VarDec(vec) |
        Token::Statements(vec) |
        Token::Symbol(vec) |
        Token::Identifier(vec) |
        Token::Keyword(vec) |
        Token::ReturnStatement(vec) => {
            vec.push(box_token);
        }, 
        _ => panic!("unexpected token")

        }


    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "class" => Some(Token::Class(Vec::new())),
            "classVarDec" => Some(Token::ClassVarDec(Vec::new())),
            "subroutineDec" => Some(Token::SubRoutineDec(Vec::new())),
            "parameterList" => Some(Token::ParameterList(Vec::new())),
            "subroutineBody" => Some(Token::SubRoutineBody(Vec::new())),
            "varDec" => Some(Token::VarDec(Vec::new())),
            "statements" => Some(Token::Statements(Vec::new())),
            "returnStatement" => Some(Token::ReturnStatement(Vec::new())),
            "keyword" => Some(Token::Keyword(Vec::new())),
            "identifier" => Some(Token::Identifier(Vec::new())),
            "symbol" => Some(Token::Symbol(Vec::new())),
            _ => None,
        }
    }

}





const IGNORE_CHARS: [char;3] = ['<','/','>'];


impl CompilationEngine{


    pub fn new(files:FilesInPathBuf) -> Result<Self,io::Error>{

        let open_file = File::open(files.get_index_in_paths(0))?;
        let mut compilation_engine = Self{
            files,
            current_file:BufReader::new(open_file),
            next_file_index:1,
            symbol_table:SymbolTable::new(),
            root_token:Token::Class(Vec::new()),
        };


        // ensuring first token is a class token
        assert_eq!(compilation_engine.compile_token().expect("No tokens found in xml"),Token::Class(Vec::new()));

        Ok(compilation_engine)


    }




    pub fn compile_token(&mut self) -> Option<Token>{

        let mut token_as_str:String = String::new();

     
        while let Ok(ch) = self.read_char() {
        if let Some(mut token) = Token::from_str(&token_as_str){
        println!("string {:?}",token);
        
        let token = match token {
            Token::Class(_) if ch == '>' => token,
            Token::Class(_) => {
                if !IGNORE_CHARS.contains(&ch) || !ch.is_whitespace(){
                    token_as_str.push(ch);
                }
                continue;
            },
            Token::ClassVarDec(_) |
            Token::SubRoutineDec(_) |
            Token::ParameterList(_) |
            Token::SubRoutineBody(_) |
            Token::VarDec(_) |
            Token::Statements(_) |
            Token::ReturnStatement(_) => token,
            Token::Identifier(_)  => {
                token.add_token(Box::new(Token::IdentifierName(self.create_string_token())));
                token
            },
            Token::Keyword(_)  => {
                token.add_token(Box::new(Token::KeywordName(self.create_string_token())));
                token
            },
            Token::Symbol(_)  => {
                token.add_token(Box::new(Token::SymbolName(self.create_string_token())));
                token
            },
            _ => panic!("unexpected token found")
            };

         return Some(token);
            
        }

        if IGNORE_CHARS.contains(&ch) || ch.is_whitespace(){
            token_as_str.clear();
            continue;
        }
        // println!("{}",ch);
        token_as_str.push(ch);
        };
        None
    }




    pub fn compile_tree(&mut self,prev:&mut Token){

    let token1 = prev;
    let Some(mut token2) = self.compile_token()else{
        return;
    };
 


    while token1 != &token2{
        self.compile_tree(&mut token2);
        token1.add_token(Box::new(token2));
        token2 = self.compile_token().expect("expected closing class ");

    }

}


    
    pub fn create_string_token(&mut self) -> String{

        let mut result = String::new();
        loop{

            let ch = self.read_char().expect("expected characters not EOF");

            if ch == '<'{
            break
            }

            result.push(ch);

        }
        result
    }




    pub fn begin(&mut self){
    



        let mut root = Token::Class(Vec::new());
        let res = self.compile_tree(&mut root);
        
        self.root_token = root;


        
        // println!("result {:?}",res);
        println!("token founds {:?}",self.root_token);
       
    


    }



    fn read_char(&mut self) -> Result<char,io::Error>{
        let mut buffer = [0u8;1];
        self.current_file.read_exact(&mut buffer)?;

        Ok(buffer[0] as char)
    }
}