
pub struct PrettyPrinter 
{
    indent_level: usize,
    indent_str: String,
    result: String,
}

impl PrettyPrinter 
{
    pub fn new(indent_str: &str) -> Self 
    {
        PrettyPrinter 
        {
            indent_level: 0,
            indent_str: indent_str.to_string(),
            result: String::new(),
        }
    }

    pub fn indent(&mut self) 
    {
        self.indent_level += 1;
    }

    pub fn unindent(&mut self) 
    {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    pub fn append_line(&mut self, line: &str) 
    {
        for _ in 0..self.indent_level 
        {
            self.result.push_str(&self.indent_str);
        }
        if self.indent_level > 0
        {
            self.result.push_str("- ");
        }
        
        self.result.push_str(line);
        self.result.push('\n');
    }

    pub fn to_string(&self) -> String {
        self.result.clone()
    }

    pub fn clear(&mut self) {
        self.result.clear();
        self.indent_level = 0;
    }
}
