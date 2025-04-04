use std::{fs::File, io::Read};

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
    let file_name = "tests/test.crs";
    let text = read_file(file_name).unwrap();
    // let text = "utils.Option[Int](1)";
    let tokens = lexing::lex_text(&text);

    if tokens.errors.len() > 0 
    {
        for e in &tokens.errors
        {
            println!("Error: {}", e)
        }

        return;
    }
    
    let ast = parsing::parse(tokens.tokens);

    match ast 
    {
        Ok(None) => println!("Empty AST"),
        Ok(Some(ast)) => println!("{:#?}", ast),
        Err(err) => 
        {
            println!("{}", err.format(&tokens.text, file_name));
        }
    }
}
