use either::Either;

use crate::{ast::TypeName, compiler::{CompilerStepError, CompilerStepResult}, lexing::token::Token, utils::TextPos};

#[derive(Debug, Clone)]
pub enum TypeError
{
    DuplicateTypeDefinition
    {
        original: Either<Token, String>,
        duplicate: Either<Token, String>,
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
        pos: Option<TextPos>,
    },
    OperatorDoesNotExist
    {
        op: Token,
        left: TypeName,
        right: TypeName,
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

impl CompilerStepError for TypeError
{
    fn pos(&self) -> Option<TextPos> 
    {
        match self 
        {
            TypeError::DuplicateTypeDefinition { original: _, duplicate } => duplicate.as_ref().left().map(|l| l.pos),
            TypeError::NotSupported { feature: _, pos } => Some(*pos),
            TypeError::UnknownType { name: _, pos } => Some(*pos),
            TypeError::GenericCountMismatch { expected: _, got: _, pos } => Some(*pos),
            TypeError::TypeNotAnInterface { name } => Some(name.get_pos()),
            TypeError::TypeNotAnStruct { name } => Some(name.get_pos()),
            TypeError::OverlappingInterfaceImplementation { pos } => *pos,
            TypeError::OperatorDoesNotExist { op, left: _, right: _ } => Some(op.pos)
        }
    }

    fn message(&self) -> String 
    {
        match self 
        {
            TypeError::DuplicateTypeDefinition { original: _, duplicate } => { 
                        let type_name = match duplicate {
                            Either::Left(l) => l.value_string().unwrap(),
                            Either::Right(r) => r
                        };

                        format!("Duplicate type {}", type_name)
                    },
            TypeError::NotSupported { feature, pos: _ } => feature.get_message(),
            TypeError::UnknownType { name, pos: _ } => format!("Unknown type name {}", name.value_string().unwrap()),
            TypeError::GenericCountMismatch { expected, got, pos: _ } => format!("Expected generic count {}, found {}", expected, got),
            TypeError::TypeNotAnInterface { name } => format!("Type {} is not an interface", name.pretty_print()),
            TypeError::TypeNotAnStruct { name } => format!("Type {} is not a struct", name.pretty_print()),
            TypeError::OverlappingInterfaceImplementation { pos: _ } => format!("Interface already implemented for type."),
            TypeError::OperatorDoesNotExist { op, left, right } => format!("Operator {:?} does not exist for types {} and {}", op.token_type, left.pretty_print(), right.pretty_print()),
        }
    }
}

pub type TypeResult<T> = Result<T, Vec<TypeError>>;