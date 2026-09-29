
use crate::handle_file_or_dir_given::{FilesInPathBuf};
use std::io::{self, BufRead, BufReader,Write,Read,Error};
use std::fs::File;
use crate::symbol_table::{SymbolTable};

#[derive(Debug)]
pub struct CompilationEngine{
    files:FilesInPathBuf,
    current_file:BufReader<File>,
    next_file_index:u32,
    symbol_table:SymbolTable,
}   



pub enum Token{
    Class(Box<Vec<Token>>),
    Keyword(String),
    Identifier(String),
    Symbol(String),
    ClassVarDec(Box<Vec<Token>>),
    SubRoutineDec(Box<Vec<Token>>),
    ParameterList(Box<Vec<Token>>),
    SubRoutineBody(Box<Vec<Token>>),
    VarDec(Box<Vec<Token>>),
    Statements(Box<Vec<Token>>),
    ReturnStatement(Box<Vec<Token>>),

}


impl Token{


    pub fn from_str(s:&str) -> Option<Self>{

        match s {
              match s {
            "class" => Some(Token::Class(Box::new(Vec::new()))),
            "keyword" => Some(Token::Keyword(s.to_string())),
            "identifier" => Some(Token::Identifier(s.to_string())),
            "symbol" => Some(Token::Symbol(s.to_string())),
            "classVarDec" => Some(Token::ClassVarDec(Box::new(Vec::new()))),
            "subroutineDec" => Some(Token::SubRoutineDec(Box::new(Vec::new()))),
            "parameterList" => Some(Token::ParameterList(Box::new(Vec::new()))),
            "subroutineBody" => Some(Token::SubRoutineBody(Box::new(Vec::new()))),
            "varDec" => Some(Token::VarDec(Box::new(Vec::new()))),
            "statements" => Some(Token::Statements(Box::new(Vec::new()))),
            "returnStatement" => Some(Token::ReturnStatement(Box::new(Vec::new()))),
            _ => None

        }

    }
}




const IGNORECHARS: [char;3] = ['<','/','<'];


impl CompilationEngine{


    pub fn new(files:FilesInPathBuf) -> Result<Self,io::Error>{

        let open_file = File::open(files.get_index_in_paths(0))?;

        Ok(Self{
            files,
            current_file:BufReader::new(open_file),
            next_file_index:1,
            symbol_table:SymbolTable::new()
        })


    }

    pub fn begin(&mut self){

         while let Ok(ch) = self.read_char(){
            if IGNORECHARS.contains(&ch) || ch.is_whitespace(){
                continue;
            }
           
         }

    }


    fn read_char(&mut self) -> Result<char,io::Error>{
        let mut buffer = [0u8;1];
        self.current_file.read_exact(&mut buffer)?;

        Ok(buffer[0] as char)
    }
}