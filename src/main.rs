use std::{fs::{create_dir_all, File, OpenOptions}, io::{Read, Write}};

use itertools::Itertools;

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

fn write_to_file(filename: &str, content: &str) -> Result<(), String> {
    // Ensure the parent directory exists
    if let Some(parent) = std::path::Path::new(filename).parent() {
        if let Err(e) = create_dir_all(parent) {
            return Err(format!("Failed to create directory: {}", e));
        }
    }

    let Ok(mut file) = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
    else {
        return Err("Failed to create file".into());
    };

    let Ok(()) = file.write_all(content.as_bytes()) else {
        return Err("Failed to write to file".into());
    };

    Ok(())
}

fn main()
{
    // let file_name = "tests/tick-tack-toe.crs";
    let file_name = "tests/test.crs";
    let text = read_file(file_name).unwrap();
    // let text = "utils.Option[Int](1)";
    let tokens = lexing::lex_text(&text);

    if tokens.errors.len() > 0 
    {
        let mut error = "Errors: \n".to_string();
        for e in &tokens.errors
        {
            error += &format!("- Error: {}\n", e);
        }

        println!("{}", error);
        return;
    }
    
    let ast = parsing::parse(tokens.tokens);

    match ast 
    {
        Ok(None) => {
            write_to_file("./logs/log.txt", "Empty AST").unwrap();
            println!("Compilation succeeded");
        },
        Ok(Some(ast)) => {
            write_to_file("./logs/log.txt", &format!("{:#?}", ast)).unwrap();
            println!("Compilation succeeded");
        },
        Err(errors) => 
        {
            let message = "Errors:\n".to_string() + &errors.iter()
                .map(|e| e.format(&tokens.text, file_name))
                .map(|e| format!(" - {}", e))
                .join("\n");
            
            println!("{}", message);
        }
    }
}
