use crate::{ast::{Expression, Program}, lexing, parsing::{self, token_reader::TokenReader}, utils::TextPos};

pub fn compile_parse_program(src: &str) -> (Result<Option<Program>, Vec<String>>, Vec<char>)
{
    let lexed = lexing::lex_text(&src);
    if lexed.is_err()
    {
        return (Err(lexed.format_errors(&lexed.text, None)), lexed.text);
    }
    
    let parsed = match parsing::parse(&lexed.tokens)
    {
        Ok(ok) => ok,
        err => return (Err(err.format_errors(&lexed.text, None)), lexed.text)
    };

    (Ok(parsed), lexed.text)
}

pub fn compile_parse_expression(src: &str) -> (Result<Option<Expression>, Vec<String>>, Vec<char>)
{
    let lexed = lexing::lex_text(&src);
    if lexed.is_err()
    {
        return (Err(lexed.format_errors(&lexed.text, None)), lexed.text);
    }
    
    let Some(mut token_reader) = TokenReader::new(&lexed.tokens, None) else {
        return (Ok(None), lexed.text)
    };
    
    let parsed = match parsing::parse_expression(&mut token_reader)
    {
        Ok(ok) => ok,
        err => return (Err(err.format_errors(&lexed.text, None)), lexed.text)
    };

    (Ok(parsed), lexed.text)
}

pub trait CompilerError
{
    fn pos(&self) -> Option<TextPos>;
    fn message(&self) -> String;

    fn format_error(&self, text: &[char], file: Option<&str>) -> String 
    {
        let loc = self.pos().unwrap_or(TextPos::uniform(text.len())).get_loc(text);
        match &file {
            Some(file) => format!("[{}:{}]: {}", file.to_string(), loc, self.message()),
            None => format!("[{}]: \"{}\"", loc, self.message())
        }
    }
}

pub trait CompilerResult
{
    fn format_errors(&self, text: &[char], file: Option<&str>) -> Vec<String>;
    fn print_errors(&self, text: &[char], file: Option<&str>)
    {
        let errors = self.format_errors(text, file.clone());
        if errors.len() > 0
        {
            println!("Errors: ");
            for e in self.format_errors(text, file)
            {
                println!(" - {}", e)
            }
        }
        else 
        {
            println!("No errors found!");    
        }
    }
}

impl<T, E> CompilerResult for Result<T, Vec<E>> 
    where E : CompilerError
{
    fn format_errors(&self, text: &[char], file: Option<&str>) -> Vec<String>
    {
        match self 
        {
            Ok(_) => vec![],
            Err(errs) => errs.iter().map(|err| err.format_error(text, file)).collect(),
        }
    }
}

impl<T, E> CompilerResult for Result<T, E>
    where E : CompilerError
{
    fn format_errors(&self, text: &[char], file: Option<&str>) -> Vec<String> 
    {
        match self 
        {
            Ok(_) => vec![],
            Err(err) => {
                let loc = err.pos().unwrap_or(TextPos::uniform(text.len())).get_loc(text);
                let err = match &file {
                    Some(file) => format!("[{}:{}]: {}", file.to_string(), loc, err.message()),
                    None => format!("[{}]: \"{}\"", loc, err.message())
                };

                vec![err]
            }
        }
    }
}