use std::{collections::HashMap, fs::{create_dir_all, File, OpenOptions}, io::{Read, Write}};

use ast::StructDecl;
use compiler::{compile_parse_type, CompilerStepResult};
use parsing::parse;
use uuid::Uuid;
use validation::{type_pattern::{TypePattern, WildCard}, GenericParam, ProgramTypeDefinitions};

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
    compile();
}

fn compile()
{
    let context = ProgramTypeDefinitions::new()
        .append_struct("Vec", vec![GenericParam { name: "T".into() }]).unwrap()
        .append_struct("Pair", vec![GenericParam { name: "A".into() }, GenericParam { name: "B".into() }]).unwrap();

    let type_a = compile_parse_type("Pair[Vec[Int], B]", None).result.unwrap().unwrap();
    let wild_cards_a = make_wildcards(&["A", "B"]);
    let type_a = TypePattern::from_type_name(&type_a, &context, &wild_cards_a).unwrap();

    let type_b = compile_parse_type("Pair[C, Int]", None).result.unwrap().unwrap();
    let wild_cards_b = make_wildcards(&["C", "D"]);
    let type_b = TypePattern::from_type_name(&type_b, &context, &wild_cards_b).unwrap();
    println!("{:#?}", type_a.is_aliasable_as(&type_b));
}

fn make_wildcards(names: &[&str]) -> HashMap<String, WildCard>
{
    let mut wild_cards = HashMap::new();
    for name in names
    {
        wild_cards.insert(name.to_string(), WildCard {
            id: Uuid::new_v4(),
            impls: vec![]
        });
    }

    wild_cards
}
