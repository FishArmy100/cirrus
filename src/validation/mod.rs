pub mod type_error;
pub mod type_def;

use itertools::Itertools;
pub use type_error::*;
pub use type_def::*;
use uuid::Uuid;

use crate::{ast::{Declaration, Expression, ImplStmt, Program, TypeName}, lexing::token::Token};

pub enum TypePattern
{
    Primary
    {
        id: Uuid,
        generics: Vec<TypePattern>,
    },
}

impl TypePattern
{
    pub fn new(type_name: &TypeName, type_defs: &ProgramTypeDefinitions) -> Result<Self, TypeError>
    {
        match type_name
        {
            TypeName::Identifier { name, args } => {
                let Some(type_def) = type_defs.get_from_name(&name.value_string().unwrap()) else {
                    return Err(TypeError::UnknownType { name: name.clone(), pos: type_name.get_pos() });
                };

                let generics_count = args.as_ref().map_or(0, |args| args.args.len());

                if type_def.get_generic_count() != generics_count
                {
                    return Err(TypeError::GenericCountMismatch { 
                        expected: type_def.get_generic_count(), 
                        got: generics_count, 
                        pos: type_name.get_pos(), 
                    });
                }
                
                let generics: Result<Vec<_>, _> = args.as_ref()
                    .map_or(vec![], |g| g.args.iter().map(|g| Self::new(g, type_defs)).collect())
                    .into_iter().collect();
                
                Ok(Self::Primary { id: type_def.get_id().clone(), generics: generics? })
            },
            t => return Err(TypeError::NotSupported { 
                feature: NotSupportedFeature::NonPrimaryImpls, 
                pos: t.get_pos() 
            })
        }
    }

    pub fn is_equivalent(&self, other: &Self) -> bool
    {
        match (self, other) 
        {
            (TypePattern::Primary { id: s_id, generics: s_generics }, TypePattern::Primary { id, generics }) => {
                s_id == id && 
                s_generics.len() == generics.len() && 
                s_generics.iter().zip(generics.iter()).all(|(a, b)| a.is_equivalent(b))
            }
        }
    }

    pub fn is_interface(&self, type_defs: &ProgramTypeDefinitions) -> bool
    {
        match self 
        {
            TypePattern::Primary { id, generics: _ } => type_defs.interfaces.contains_key(id),
        }
    }
}

pub struct InterfaceImpl
{
    pub implemented: TypePattern,
    pub interface: TypePattern,
    pub token: Token,
}

impl InterfaceImpl
{
    pub fn overlaps(&self, other: &Self) -> bool
    {
        self.implemented.is_equivalent(&other.implemented) && 
        self.interface.is_equivalent(&other.interface)
    }

    pub fn new(impl_stmt: &ImplStmt, type_defs: &ProgramTypeDefinitions) -> Result<Self, TypeError>
    {
        if let Some(generic_params) = &impl_stmt.generic_params {
            return Err(TypeError::NotSupported { 
                feature: NotSupportedFeature::GenericImpls, 
                pos: generic_params.open_bracket.pos + generic_params.close_bracket.pos 
            });
        }

        let Some(for_clause) = &impl_stmt.for_clause else {
            return Err(TypeError::NotSupported { 
                feature: NotSupportedFeature::NonForClauseImpls, 
                pos: impl_stmt.impl_tok.pos,
            });
        };
        
        let interface_pattern = TypePattern::new(&for_clause.1, type_defs)?;
        if !interface_pattern.is_interface(type_defs)
        {
            return Err(TypeError::TypeNotAnInterface { name: for_clause.1.clone() })
        }

        let struct_pattern = TypePattern::new(&impl_stmt.type_name, type_defs)?;
        if struct_pattern.is_interface(type_defs)
        {
            return Err(TypeError::TypeNotAnStruct { name: impl_stmt.type_name.clone() })
        }
        
        Ok(Self { 
            implemented: struct_pattern, 
            interface: interface_pattern,
            token: impl_stmt.impl_tok.clone(),
        })
    }

    pub fn from_ast(program: &Program, type_defs: &ProgramTypeDefinitions) -> TypeResult<Vec<Self>>
    {

        let impls = program.declarations.iter().filter_map(|d| match d
        {
            Declaration::Impl(impl_stmt) => Some(InterfaceImpl::new(impl_stmt, type_defs)),
            _ => None,
        }).collect_vec();

        let mut errors = impls.iter().filter_map(|i| i.as_ref().err().cloned()).collect_vec();
        let impls = impls.into_iter().filter_map(|i| i.ok()).collect_vec();
        
        for i in 0..impls.len()
        {
            for j in (i + 1)..impls.len()
            {
                let a = &impls[i];
                let b = &impls[j];

                if a.overlaps(b)
                {
                    errors.push(TypeError::OverlappingInterfaceImplementation { pos: b.token.pos });
                }
            }
        }

        if errors.len() > 0
        {
            Err(errors)
        }
        else 
        {
            Ok(impls)    
        }
    }
}

pub struct TypeContext
{
    pub defs: ProgramTypeDefinitions,
    pub impls: Vec<InterfaceImpl>,
}

impl TypeContext
{
    pub fn new(program: &Program) -> TypeResult<Self>
    {
        let defs = ProgramTypeDefinitions::new(program)?;
        let impls = InterfaceImpl::from_ast(program, &defs)?;

        Ok(Self {
            defs,
            impls,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp
{
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    NotEqual,
    Equal,
    And,
    Or
}

pub enum TypedExpression
{
    Literal
    {
        returned: TypePattern,
    },
    Binary
    {
        left: Box<TypedExpression>,
        op: BinaryOp,
        right: Box<TypedExpression>,
        returned: TypePattern,
    }
}

impl TypedExpression
{
    pub fn new(expr: &Expression, context: &TypeContext) -> Result<Self, TypeError>
    {
        
    }
}