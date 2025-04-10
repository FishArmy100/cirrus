use crate::utils::TextPos;


pub trait CompilerError
{
    fn pos(&self) -> Option<TextPos>;
    fn message(&self) -> String;
}

pub trait CompilerResult<T>
{
    fn is_ok(&self) -> bool;
    fn get_result(&self) -> Option<&T>;
    fn get_errors(&self) -> Vec<impl CompilerError>;
    
    
    fn is_err(&self) -> bool
    {
        !self.is_ok()
    }

    fn format_errors(&self, text: &[char], file: Option<impl ToString>) -> Vec<String>
    {
        self.get_errors()
            .iter()
            .map(|e| {
                let loc = e.pos().unwrap_or(TextPos::uniform(text.len())).get_loc(text);
                match &file 
                {
                    Some(file) => format!("[{}:{}]: {}", file.to_string(), loc, e.message()),
                    None => format!("[{}]: \"{}\"", loc, e.message())
                }
            })
            .collect()
    }

    fn print_errors(&self, text: &[char], file: Option<impl ToString>)
    {
        println!("Errors: ");
        for e in self.format_errors(text, file)
        {
            println!(" - {}", e)
        }
    }
}