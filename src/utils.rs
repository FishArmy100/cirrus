use std::ops::Add;


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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextPos
{
    pub begin: usize,
    pub end: usize
}

impl TextPos
{
    pub fn new(begin: usize, end: usize) -> Self 
    {
        Self 
        {
            begin,
            end,
        }
    }

    pub fn uniform(index: usize) -> Self 
    {
        Self 
        { 
            begin: index, 
            end: index 
        }
    }

    pub fn get_loc(&self, text: &[char]) -> TextLocation
    {
        let mut line = 1;
        let mut column = 1;

        let line_count = text.iter().filter(|f| **f == '\n').count() + 1;

        if self.begin >= text.len()
        {
            return TextLocation { line: line_count, column };
        }

        for i in 0..=self.begin
        {
            if text[i] == '\n'
            {
                line += 1;
                column = 0;
            }
            else 
            {
                column += 1;
            }
        }

        TextLocation { line, column }
    }
}

impl Add for TextPos
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output 
    {
        Self 
        {
            begin: self.begin.min(rhs.begin),
            end: self.end.max(rhs.end),
        }
    }
}

impl From<usize> for TextPos
{
    fn from(value: usize) -> Self 
    {
        Self::uniform(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextLocation
{
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for TextLocation
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}:{}", self.line, self.column)
    }
}

pub fn partition_errors<T, E>(results: impl IntoIterator<Item = Result<T, E>>) -> Result<Vec<T>, Vec<E>>
{
    let mut oks = Vec::new();
    let mut errs = Vec::new();

    for result in results {
        match result {
            Ok(val) => oks.push(val),
            Err(err) => errs.push(err),
        }
    }

    if errs.is_empty() 
    {
        Ok(oks)
    } 
    else {
        Err(errs)
    }
}