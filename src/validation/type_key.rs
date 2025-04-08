use itertools::Itertools;
use uuid::Uuid;

#[derive(Debug, Clone, Hash)]
pub enum TypeKey
{
    Primary
    {
        id: Uuid,
        name: String,
        generic_args: Vec<TypeKey>,
    },
    Array(Box<TypeKey>),
    Function
    {
        parameters: Vec<TypeKey>,
        return_type: Box<TypeKey>,
    },
    WildCard
}

impl TypeKey
{
    pub fn new_primary(name: impl ToString) -> Self 
    {
        Self::Primary 
        { 
            id: Uuid::new_v4(), 
            name: name.to_string(), 
            generic_args: vec![]
        }
    }

    pub fn primary_id(&self) -> Option<Uuid>
    {
        match self 
        {
            Self::Primary { id, name: _, generic_args: _ } => Some(id.clone()),
            _ => None
        }
    }

    pub fn is_equivalent(&self, other: &Self) -> bool
    {
        match (self, other) 
        {
            (Self::Primary { id: a, name: _, generic_args: a_args }, Self::Primary { id: b, name: _, generic_args: b_args }) => 
            {
                a == b && a_args == b_args
            },
            (Self::Array(a), Self::Array(b)) => 
            {
                *a == *b
            },
            (Self::Function { parameters: a_ps, return_type: a_r }, Self::Function { parameters: b_ps, return_type: b_r }) => 
            {
                a_ps == b_ps && a_r == b_r
            },
            (Self::WildCard, _) => true,
            (_, Self::WildCard) => true,
            _ => false
        }
    }

    pub fn is_definite(&self) -> bool
    {
        match self 
        {
            TypeKey::Primary { id: _, name: _, generic_args } => generic_args.iter().all(|a| a.is_definite()),
            TypeKey::Array(type_key) => type_key.is_definite(),
            TypeKey::Function { parameters, return_type } => parameters.iter().all(|p| p.is_definite()) && return_type.is_definite(),
            TypeKey::WildCard => false,
        }
    }

    fn stringify(&self, wildcard_count: &mut usize) -> String
    {
        match self 
        {
            TypeKey::Primary { id: _, name, generic_args } => if generic_args.len() > 0
            {
                format!("{}[{}]", name, generic_args.iter().map(|a| a.stringify(wildcard_count)).join(", "))
            }
            else
            {
                name.clone()
            },
            TypeKey::Array(type_key) => format!("[]{}", type_key.stringify(wildcard_count)),
            TypeKey::Function { parameters, return_type } => {
                format!("fn({}) -> {}", parameters.iter().map(|p| p.stringify(wildcard_count)).join(", "), return_type.stringify(wildcard_count))
            },
            TypeKey::WildCard => {
                let r = format!("T{}", *wildcard_count);
                *wildcard_count += 1;
                r
            },
        }
    }
}

impl PartialEq for TypeKey
{
    fn eq(&self, other: &Self) -> bool 
    {
        match (self, other) 
        {
            (Self::Primary { id: a, name: _, generic_args: a_args }, Self::Primary { id: b, name: _, generic_args: b_args }) => 
            {
                a == b && a_args == b_args
            },
            (Self::Array(a), Self::Array(b)) => 
            {
                *a == *b
            },
            (Self::Function { parameters: a_ps, return_type: a_r }, Self::Function { parameters: b_ps, return_type: b_r }) => 
            {
                a_ps == b_ps && a_r == b_r
            },
            (Self::WildCard, Self::WildCard) => true,
            _ => false
        }
    }
}

impl std::fmt::Display for TypeKey
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        let mut count = 0;
        write!(f, "{}", self.stringify(&mut count))
    }
}