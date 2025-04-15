use std::ops::Deref;

use uuid::Uuid;

use crate::ast::TypeName;

use super::{ProgramTypeDefinitions, TypeError};

#[derive(Debug, Clone)]
pub struct TypePattern
{
    pub wild_cards: Vec<WildCard>,
    pub ast: TypePatternAst,
}

impl TypePattern
{
    pub fn new_primary(id: Uuid, wild_cards: Vec<WildCard>) -> Self 
    {
        let len = wild_cards.len();
        Self 
        {
            wild_cards,
            ast: TypePatternAst::Primary { 
                id, 
                generics: (0..len).map(|i| TypePatternAst::WildCardIdx(i)).collect() 
            }
        }
    }

    pub fn from_type_name(type_name: &TypeName, type_defs: &ProgramTypeDefinitions, wild_cards: Vec<WildCard>) -> Result<Self, TypeError>
    {
        
    }

    pub fn is_equivalent(&self, other: &Self) -> bool
    {
        
    }

    pub fn is_interface(&self, type_defs: &ProgramTypeDefinitions) -> bool
    {
        match &self.ast
        {
            TypePatternAst::Primary { id, generics: _ } => type_defs.interfaces.contains_key(id),
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WildCard(pub Vec<Uuid>);

impl WildCard
{
    /// if `other` wildcard can be aliased as the `self`, may not be the case the other way around
    /// 
    /// Example: `T : Add + Clone` and `F : Add` so `T.is_aliasable_as(F)` is **true** but `T.is_aliasable_as(F)` is **false**
    pub fn is_aliasable_as(&self, other: &Self) -> bool
    {
        other.iter().all(|w| self.contains(w))
    }
}

impl Deref for WildCard
{
    type Target = [Uuid];

    fn deref(&self) -> &Self::Target 
    {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum TypePatternAst
{
    Primary
    {
        id: Uuid,
        generics: Vec<TypePatternAst>,
    },
    WildCardIdx(usize)
}

impl TypePatternAst
{
    pub fn is_equivalent(&self, other: &Self, defs: &ProgramTypeDefinitions)
    {
        match (self, other)
        {
            (Self::Primary { id: s_id, generics: s_gen }, Self::Primary { id: o_id, generics: o_gen })
        }
    }
}