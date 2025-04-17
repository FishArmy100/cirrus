use std::{collections::{HashMap, HashSet}, ops::Deref};

use either::Either;
use itertools::Itertools;
use uuid::Uuid;

use crate::{ast::TypeName, lexing::token::Token};

use super::{TypeDefContext, TypeError};

#[derive(Debug, Clone)]
pub enum TypePattern
{
    Primary 
    {
        type_id: Uuid,
        generic_args: Vec<TypePattern>
    },
    Function
    {
        args: Vec<TypePattern>,
        returned: Box<TypePattern>,
    },
    WildCard(WildCard)
}

impl TypePattern
{
    pub fn from_type_name(type_name: &TypeName, defs: &TypeDefContext, wild_cards: &HashMap<String, WildCard>) -> Result<Self, TypeError>
    {
        let TypeName::Identifier { name: name_tok, args } = type_name else {
            panic!("This no worky")
        };

        let name = name_tok.value_string().unwrap();
        if let Some(type_id) = defs.get_from_name(&name).map(|d| d.get_id().clone())
        {
            let generic_args = args.as_ref()
                .map_or(Ok(vec![]), |args| args.args.iter().map(|a| Self::from_type_name(a, defs, wild_cards))
                .collect::<Result<Vec<_>, _>>())?;

            Ok(Self::Primary { 
                type_id,
                generic_args,
            })
        }
        else if let Some(wild_card) = wild_cards.get(name)
        {
            Ok(Self::WildCard(wild_card.clone()))
        }
        else 
        {
            Err(TypeError::UnknownType { name: name_tok.clone(), pos: type_name.get_pos() })    
        }
    }

    pub fn is_aliasable_as(&self, other: &Self) -> bool
    {
        let mut mappings = WildCardMappings::new();
        self._is_aliasable_as(other, &mut mappings)
    }

    fn _is_aliasable_as(&self, other: &Self, mappings: &mut WildCardMappings) -> bool
    {
        match (self, other)
        {
            (Self::Primary { type_id: a_id, generic_args: a_args }, Self::Primary { type_id: b_id, generic_args: b_args }) => {
                a_id == b_id && // make sure they are the same type
                a_args.len() == b_args.len() &&
                a_args.iter().zip(b_args.iter()).all(|(a, b)| a._is_aliasable_as(b, mappings)) // make sure all the args are aliasable as each other
            },
            (Self::Function { args: a_args, returned: a_ret }, Self::Function { args: b_args, returned: b_ret }) => {
                a_args.len() == b_args.len() &&
                a_args.iter().zip(b_args.iter()).all(|(a, b)| a._is_aliasable_as(b, mappings)) &&
                a_ret._is_aliasable_as(&b_ret, mappings)
            },
            (Self::WildCard(a), Self::WildCard(b)) => a.is_aliasable_as(b, mappings),
            (s, Self::WildCard(wild_card)) => 
            {
                mappings.push_pair(wild_card.id.clone(), Either::Right(s.clone()))
            },
            (Self::WildCard(_), _) => false, // cannot alias a unknown type as a known one
            _ => false,
        }
    }
}

impl PartialEq for TypePattern
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Primary { type_id: l_type_id, generic_args: l_generic_args }, Self::Primary { type_id: r_type_id, generic_args: r_generic_args }) => l_type_id == r_type_id && l_generic_args == r_generic_args,
            (Self::WildCard(l0), Self::WildCard(r0)) => l0 == r0,
            _ => false,
        }
    }
}

pub struct WildCardMappings
{
    seen: HashMap<Uuid, Either<Uuid, TypePattern>>
}

impl WildCardMappings
{
    pub fn new() -> Self 
    {
        Self 
        {
            seen: HashMap::new()
        }
    }

    pub fn push_pair(&mut self, left: Uuid, right: Either<Uuid, TypePattern>) -> bool
    {
        if let Some(val) = self.seen.get(&left)
        {
            *val == right
        }
        else 
        {
            self.seen.insert(left, right).is_none() // should always be true    
        }
    }
}

#[derive(Debug, Clone)]
pub struct WildCard
{
    pub id: Uuid,
    pub impls: Vec<TypePattern> // interface ids
}

impl WildCard
{
    /// if `other` wildcard can be aliased as the `self`, may not be the case the other way around
    /// 
    /// Example: `T : Add + Clone` and `F : Add` so `T.is_aliasable_as(F)` is **true** but `T.is_aliasable_as(F)` is **false**
    pub fn is_aliasable_as(&self, other: &Self, mappings: &mut WildCardMappings) -> bool
    {
        other.impls.len() == self.impls.len() &&
        self.impls.iter().zip(other.impls.iter()).all(|(a, b)| a._is_aliasable_as(b, mappings)) &&
        mappings.push_pair(other.id, Either::Left(self.id.clone()))
    }
}

impl PartialEq for WildCard
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.id == other.id
    }
}