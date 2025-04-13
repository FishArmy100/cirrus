use crate::{ast::TypeName, compiler::{CompilerError, CompilerResult}, lexing::token::Token, utils::TextPos};

#[derive(Debug, Clone)]
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
    UnknownType
    {
        name: Token,
        pos: TextPos,
    },
    GenericCountMismatch
    {
        expected: usize,
        got: usize,
        pos: TextPos,
    },
    TypeNotAnInterface
    {
        name: TypeName,
    },
    TypeNotAnStruct
    {
        name: TypeName,
    },
    OverlappingInterfaceImplementation
    {
        pos: TextPos,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotSupportedFeature
{
    NonPrimaryImpls,
    GenericImpls,
    NonForClauseImpls
}

impl NotSupportedFeature
{
    pub fn get_message(&self) -> String 
    {
        match self 
        {
            NotSupportedFeature::NonPrimaryImpls => "Non primary type implementations like []Int are not currently supported".into(),
            NotSupportedFeature::GenericImpls => "Generic impls are not implemented yet".into(),
            NotSupportedFeature::NonForClauseImpls => "Only impl statements for interfaces are implemented at the moment".into(),
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
            TypeError::UnknownType { name: _, pos } => Some(*pos),
            TypeError::GenericCountMismatch { expected: _, got: _, pos } => Some(*pos),
            TypeError::TypeNotAnInterface { name } => Some(name.get_pos()),
            TypeError::TypeNotAnStruct { name } => Some(name.get_pos()),
            TypeError::OverlappingInterfaceImplementation { pos } => Some(*pos),
        }
    }

    fn message(&self) -> String 
    {
        match self 
        {
            TypeError::DuplicateTypeDefinition { original: _, duplicate } => format!("Duplicate type {}", duplicate.value_string().unwrap()),
            TypeError::NotSupported { feature, pos: _ } => feature.get_message(),
            TypeError::UnknownType { name, pos: _ } => format!("Unknown type name {}", name.value_string().unwrap()),
            TypeError::GenericCountMismatch { expected, got, pos: _ } => format!("Expected generic count {}, found {}", expected, got),
            TypeError::TypeNotAnInterface { name } => format!("Type {} is not an interface", name.pretty_print()),
            TypeError::TypeNotAnStruct { name } => format!("Type {} is not a struct", name.pretty_print()),
            TypeError::OverlappingInterfaceImplementation { pos: _ } => format!("Interface already implemented for type."),
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