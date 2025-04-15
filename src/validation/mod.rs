pub mod type_error;
pub mod type_def;
pub mod builtins;

use builtins::get_builtin_impls;
use itertools::Itertools;
pub use type_error::*;
pub use type_def::*;
use uuid::Uuid;

use crate::{ast::{Declaration, Expression, ImplStmt, Program, TypeName}, lexing::token::{Token, TokenType}};

#[derive(Debug, Clone)]
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
    pub fn new_primary(id: Uuid) -> Self 
    {
        Self::Primary { id, generics: vec![] }
    }

    pub fn get_primary_id(&self) -> Option<&Uuid>
    {
        match self 
        {
            Self::Primary { id, generics: _ } => Some(id),
            _ => None,
        }
    }

    pub fn from_type_name(type_name: &TypeName, globals: &ProgramTypeDefinitions) -> Result<Self, TypeError>
    {
        match type_name
        {
            TypeName::Identifier { name, args } => {
                let Some(type_def) = globals.get_from_name(&name.value_string().unwrap()) else {
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
                    .map_or(vec![], |g| g.args.iter().map(|g| Self::from_type_name(g, globals)).collect())
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

    pub fn is_interface(&self, globals: &ProgramTypeDefinitions) -> bool
    {
        match self 
        {
            TypePattern::Primary { id, generics: _ } => globals.interfaces.contains_key(id),
        }
    }
}

pub struct InterfaceImpl
{
    pub implementee: TypePattern,
    pub interface: TypePattern,
    pub token: Option<Token>,
}

impl InterfaceImpl
{
    pub fn overlaps(&self, other: &Self) -> bool
    {
        self.implementee.is_equivalent(&other.implementee) && 
        self.interface.is_equivalent(&other.interface)
    }

    pub fn new_builtin(implementee: TypePattern, interface: TypePattern) -> Self
    {
        Self 
        {
            implementee,
            interface,
            token: None,
        }
    }

    pub fn from_impl_stmt(impl_stmt: &ImplStmt, globals: &ProgramTypeDefinitions) -> Result<Self, TypeError>
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
        
        let interface_pattern = TypePattern::from_type_name(&for_clause.1, globals)?;
        if !interface_pattern.is_interface(globals)
        {
            return Err(TypeError::TypeNotAnInterface { name: for_clause.1.clone() })
        }

        let struct_pattern = TypePattern::from_type_name(&impl_stmt.type_name, globals)?;
        if struct_pattern.is_interface(globals)
        {
            return Err(TypeError::TypeNotAnStruct { name: impl_stmt.type_name.clone() })
        }
        
        Ok(Self { 
            implementee: struct_pattern, 
            interface: interface_pattern,
            token: Some(impl_stmt.impl_tok.clone()),
        })
    }

    pub fn append_from_ast(others: &mut Vec<InterfaceImpl>, program: &Program, globals: &ProgramTypeDefinitions) -> TypeResult<()>
    {
        let impls = program.declarations.iter().filter_map(|d| match d
        {
            Declaration::Impl(impl_stmt) => Some(InterfaceImpl::from_impl_stmt(impl_stmt, globals)),
            _ => None,
        }).collect_vec();

        let mut errors = impls.iter().filter_map(|i| i.as_ref().err().cloned()).collect_vec();
        others.extend(impls.into_iter().filter_map(|i| i.ok()));
        
        for i in 0..others.len()
        {
            for j in (i + 1)..others.len()
            {
                let a = &others[i];
                let b = &others[j];

                if a.overlaps(b)
                {
                    errors.push(TypeError::OverlappingInterfaceImplementation { pos: b.token.as_ref().map(|t| t.pos) });
                }
            }
        }

        if errors.len() > 0
        {
            Err(errors)
        }
        else 
        {
            Ok(())    
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
        let defs = ProgramTypeDefinitions::new().append_program(program)?;
        let mut impls = get_builtin_impls(&defs.builtins);
        InterfaceImpl::append_from_ast(&mut impls, program, &defs)?;

        Ok(Self {
            defs,
            impls,
        })
    }

    pub fn int_id(&self) -> Uuid
    {
        self.defs.builtins.int_id.clone()
    }
    
    pub fn can_add(&self, pattern: &TypePattern) -> bool
    {
        self.impls.iter().find(|i| {
            i.implementee.is_equivalent(pattern) && i.interface.get_primary_id() == Some(&self.defs.builtins.int_id)
        }).is_some()
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
    pub fn returned(&self) -> &TypePattern
    {
        match self 
        {
            TypedExpression::Literal { returned } => returned,
            TypedExpression::Binary { left: _, op: _, right: _, returned } => returned,
        }
    }

    pub fn new(expr: &Expression, context: &TypeContext) -> Result<Self, TypeError>
    {
        match expr 
        {
            Expression::Literal(literal) => {
                if literal.token_type != TokenType::IntegerLiteral { panic!("Literal not an int"); }
                let expr = TypedExpression::Literal { 
                    returned: TypePattern::new_primary(context.int_id()) // TEMP
                };
                Ok(expr)
            },
            Expression::Binary(binary) => {
                let left = Self::new(&binary.left, context)?;
                let right = Self::new(&binary.right, context)?;

                if binary.operator.token_type != TokenType::Plus { panic!("Literal cant be added") }

                if !left.returned().is_equivalent(right.returned()) && context.can_add(pattern)
                {
                    return Err(TypeError::OperatorDoesNotExist { 
                        op: binary.operator.clone(), 
                        left: binary.left.clone(), 
                        right: binary.right.clone(),
                    })
                }
            }
            _ => panic!("Not implemented yet"),
        }
    }
}