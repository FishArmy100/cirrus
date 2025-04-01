use std::{fs::File, io::Read};

pub mod lexing;
pub mod parsing;
pub mod ast;

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
    let text = read_file("tests/test.crs").unwrap();
    let result = lexing::lex_text(&text);
    
    if result.errors.len() > 0
    {
        for e in &result.errors
        {
            println!("Error: {}", e)
        }
    }
    else 
    {
        for t in &result.tokens
        {
            println!("{}", t);
        }
    }
}
