use std::{fs::{create_dir_all, File, OpenOptions}, io::{Read, Write}};

use ast::StructDecl;
use compiler::CompilerStepResult;
use parsing::parse;
use validation::ProgramTypeDefinitions;

pub mod lexing;
pub mod parsing;
pub mod ast;
pub mod utils;
pub mod validation;
pub mod compiler;

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
    // let text = "1 + 1 == false";
    compile(&text, None);
}

fn compile(text: &str, file: Option<&str>)
{
    
}
