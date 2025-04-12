use crate::{compiler::{CompilerError, CompilerResult}, lexing::token::Token, utils::TextPos};

#


[derive(Debug, Clone)]
pub enum TypeError
{
    DuplicateTypeDefinition
    {
        original: Token,
        duplicate: Token,
    },
    NotSupported
    {
        feature: NotSupportedFeature,
        pos: TextPos,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotSupportedFeature
{
    NonPrimaryImpls
}

impl NotSupportedFeature
{
    pub fn get_message(&self) -> String 
    {
        match self 
        {
            NotSupportedFeature::NonPrimaryImpls => "Non primary type implementations like []Int are not currently supported".into(),
        }
    }
}

impl CompilerError for TypeError
{
    fn pos(&self) -> Option<TextPos> 
    {
        match self 
        {
            TypeError::DuplicateTypeDefinition { original: _, duplicate } => Some(duplicate.pos),
            TypeError::NotSupported { feature: _, pos } => Some(*pos),
        }
    }

    fn message(&self) -> String 
    {
        match self 
        {
            TypeError::DuplicateTypeDefinition { original: _, duplicate } => format!("Duplicate type {}", duplicate.value_string().unwrap()),
            TypeError::NotSupported { feature, pos: _ } => feature.get_message(),
        }
    }
}

pub type TypeResult<T> = Result<T, Vec<TypeError>>;

impl<T> CompilerResult<T> for TypeResult<T>
{
    fn is_ok(&self) -> bool 
    {
        self.is_ok()
    }

    fn get_result(&self) -> Option<&T> 
    {
        self.as_ref().ok()
    }

    fn get_errors(&self) -> Vec<impl CompilerError> 
    {
        self.as_ref().err().map_or(vec![], |errs| errs.clone())
    }
}