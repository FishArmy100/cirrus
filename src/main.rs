use std::{fs::File, io::Read};

use parsing::token_reader::TokenReader;

pub mod lexing;
pub mod parsing;
pub mod ast;
pub mod utils;

fn read_file(path: &str) -> Result<String, String> 
{
    let Ok(mut file) = File::open(path) else {
        return Err(format!("Could not open file: `{}`", path));
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents)
    {
        return Err(format!("Invalid file format"));
    }

    Ok(contents)
}

fn main() 
{
    // let text = read_file("tests/test.crs").unwrap();
    let text = "[]fn(Map[Int, String], String) -> []Int";
    let tokens = lexing::lex_text(&text);

    if tokens.errors.len() > 0 
    {
        for e in &tokens.errors
        {
            println!("Error: {}", e)
        }

        return;
    }
    
    let mut reader = TokenReader::new(&tokens.tokens, None).unwrap();
    let ast = parsing::parse_type_name(&mut reader);

    match ast 
    {
        Ok(None) => println!("Empty AST"),
        Ok(Some(ast)) => println!("{:#?}", ast),
        Err(err) => 
        {
            println!("{:?}", err);
        }
    }
}
