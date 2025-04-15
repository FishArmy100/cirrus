use std::collections::HashMap;

use either::Either::{self, Left, Right};
use itertools::Itertools;
use uuid::Uuid;

use crate::{ast::{Declaration, InterfaceDecl, Program, StructDecl}, lexing::token::Token, utils::TextPos};

use super::{builtins::{get_builtin_types, Builtins, BuiltinsResult}, type_error::TypeError};

#[derive(Debug, Clone, Copy)]
pub enum TypeDefRef<'a>
{
    Struct(&'a StructDef),
    Interface(&'a InterfaceDef),
}

impl<'a> TypeDefRef<'a>
{
    pub fn name_token(&self) -> Option<&Token> 
    {
        match self 
        {
            TypeDefRef::Struct(struct_def) => struct_def.name_tok.as_ref(),
            TypeDefRef::Interface(interface_def) => interface_def.name_tok.as_ref(),
        }
    }

    pub fn name(&self) -> &str 
    {
        match self 
        {
            TypeDefRef::Struct(struct_def) => &struct_def.name,
            TypeDefRef::Interface(interface_def) => &interface_def.name,
        }
    }

    pub fn get_pos(&self) -> Option<TextPos>
    {
        match self 
        {
            TypeDefRef::Struct(struct_def) => struct_def.pos,
            TypeDefRef::Interface(interface_def) => interface_def.pos,
        }
    }

    pub fn get_generic_count(&self) -> usize
    {
        match self 
        {
            TypeDefRef::Struct(struct_def) => struct_def.generic_params.len(),
            TypeDefRef::Interface(interface_def) => interface_def.generic_params.len(),
        }
    }

    pub fn get_id(&self) -> &Uuid
    {
        match self 
        {
            TypeDefRef::Struct(struct_def) => &struct_def.id,
            TypeDefRef::Interface(interface_def) => &interface_def.id,
        }
    }
}



#[derive(Debug)]
pub struct ProgramTypeDefinitions
{
    pub interfaces: HashMap<Uuid, InterfaceDef>,
    pub structs: HashMap<Uuid, StructDef>,
    pub builtins: Builtins,
}

impl ProgramTypeDefinitions
{
    pub fn new() -> Self
    {
        let BuiltinsResult { structs, interfaces, builtins } = get_builtin_types();
        Self 
        {
            structs,
            interfaces,
            builtins,
        }
    }

    pub fn append_struct(mut self, name: &str, params: Vec<GenericParam>) -> Result<Self, Vec<TypeError>>
    {
        if let Some(og) = self.structs.iter().find(|def| def.1.name == name) {
            let err = TypeError::DuplicateTypeDefinition { original: Right(og.1.name.clone()), duplicate: Right(name.to_string()) };
            return Err(vec![err])
        }

        let def = StructDef::new_builtin(name, params);
        self.structs.insert(def.id.clone(), def);
        Ok(self)
    }

    pub fn append_program(mut self, program: &Program) -> Result<Self, Vec<TypeError>>
    {
        let mut errors = vec![];

        for declaration in &program.declarations
        {
            match declaration
            {
                Declaration::Struct(_, decl) => 
                {
                    match StructDef::from_struct_decl(decl)
                    {
                        Ok(ok) => { self.structs.insert(ok.id.clone(), ok); },
                        Err(errs) => errors.extend(errs),
                    }
                }
                Declaration::Interface(_, decl) => 
                {
                    match InterfaceDef::from_interface_decl(decl)
                    {
                        Ok(ok) => { self.interfaces.insert(ok.id.clone(), ok); },
                        Err(errs) => errors.extend(errs),
                    }
                }
                _ => {}
            }
        }

        let mut def_refs = vec![];
        self.structs.values().for_each(|s| def_refs.push(TypeDefRef::Struct(s)));
        self.interfaces.values().for_each(|i| def_refs.push(TypeDefRef::Interface(i)));

        let duplicate_errors = def_refs.iter().duplicates_by(|d| d.name()).map(|def| {
            let og = def_refs.iter().find(|d| d.name() == def.name()).unwrap();

            let original = match og.name_token() {
                Some(token) => Either::Left(token.clone()),
                None => Either::Right(og.name().into())
            };

            let duplicate = match def.name_token() {
                Some(token) => Either::Left(token.clone()),
                None => Either::Right(def.name().into())
            };

            TypeError::DuplicateTypeDefinition { 
                original,
                duplicate,
            }
        }).collect_vec();

        errors.extend(duplicate_errors);
        
        if errors.len() > 0
        {
            Err(errors)
        }
        else 
        {
            Ok(self)
        }
    }

    pub fn get_def(&self, id: &Uuid) -> Option<TypeDefRef>
    {
        if let Some(s) = self.structs.get(id)
        {
            Some(TypeDefRef::Struct(s))
        }
        else if let Some(i) = self.interfaces.get(id)
        {
            Some(TypeDefRef::Interface(i))
        }
        else 
        {
            None    
        }
    }

    pub fn get_from_name(&self, name: &str) -> Option<TypeDefRef>
    {
        if let Some(s) = self.structs.values().find(|s| s.name == name)
        {
            Some(TypeDefRef::Struct(s))
        }
        else if let Some(i) = self.interfaces.values().find(|s| s.name == name)
        {
            Some(TypeDefRef::Interface(i))
        }
        else 
        {
            None    
        }
    }

    pub fn get_interface_from_name(&self, name: &str) -> Option<&InterfaceDef>
    {
        self.interfaces.values().find(|s| s.name == name)
    }
}

#[derive(Debug, Clone)]
pub struct GenericParam 
{
    pub name: String,
}

#[derive(Debug)]
pub struct StructDef
{
    pub id: Uuid,
    pub name: String,
    pub name_tok: Option<Token>,
    pub pos: Option<TextPos>,
    pub generic_params: Vec<GenericParam>
}

impl StructDef
{
    pub fn new_builtin(name: &str, params: Vec<GenericParam>) -> Self 
    {
        Self 
        {
            id: Uuid::new_v4(),
            name: name.into(),
            name_tok: None,
            pos: None,
            generic_params: params
        }
    }

    pub fn from_struct_decl(decl: &StructDecl) -> Result<StructDef, Vec<TypeError>>
    {
        let struct_id = decl.id.value.as_ref().unwrap().as_string().unwrap();
        let generic_params = decl.generic_params.as_ref();

        if let Some(t) = generic_params.map(|p| p.params.iter().find(|p| p.value.as_ref().unwrap().as_string().unwrap() == struct_id)).flatten()
        {
            return Err(vec![TypeError::DuplicateTypeDefinition { 
                original: Either::Left(decl.id.clone()), 
                duplicate: Either::Left(t.clone()) 
            }])
        }

        if let Some(generic_params) = generic_params
        {
            let duplicates = generic_params.params.iter().duplicates_by(|p| p.value_string()).collect::<Vec<_>>();
            if duplicates.len() > 0
            {
                let errors = duplicates.iter().map(|err| TypeError::DuplicateTypeDefinition { 
                    original: Either::Left(generic_params.params.iter().find(|p| p.value == err.value).unwrap().clone()), 
                    duplicate: Either::Left((**err).clone())
                }).collect();

                return Err(errors)
            }
        }

        Ok(StructDef
        {
            id: Uuid::new_v4(),
            name: struct_id.clone(),
            name_tok: Some(decl.id.clone()),
            pos: Some(decl.id.pos + generic_params.map_or(decl.id.pos, |p| p.open_bracket.pos + p.close_bracket.pos)),
            generic_params: generic_params.map(|p| p.params.iter().map(|p|{ 
                let name = p.value_string().unwrap().clone();
                GenericParam {
                    name,
                }
            }).collect()).unwrap_or_default()
        })
    }
}

#[derive(Debug)]
pub struct InterfaceDef
{
    pub id: Uuid,
    pub name: String,
    pub name_tok: Option<Token>,
    pub pos: Option<TextPos>,
    pub generic_params: Vec<GenericParam>,
}

impl InterfaceDef
{
    pub fn new_builtin(name: &str, params: Vec<GenericParam>) -> Self 
    {
        InterfaceDef
        {
            id: Uuid::new_v4(),
            name: name.into(),
            name_tok: None,
            pos: None,
            generic_params: params,
        }
    }

    pub fn from_interface_decl(decl: &InterfaceDecl) -> Result<InterfaceDef, Vec<TypeError>>
    {
        let interface_id = decl.id.value.as_ref().unwrap().as_string().unwrap();
        let generic_params = decl.generic_params.as_ref();

        if let Some(t) = generic_params.map(|p| p.params.iter().find(|p| p.value.as_ref().unwrap().as_string().unwrap() == interface_id)).flatten()
        {
            return Err(vec![TypeError::DuplicateTypeDefinition { original: Either::Left(decl.id.clone()), duplicate: Either::Left(t.clone()) }])
        }

        if let Some(generic_params) = generic_params
        {
            let duplicates = generic_params.params.iter().duplicates_by(|p| p.value_string()).collect::<Vec<_>>();
            if duplicates.len() > 0
            {
                let errors = duplicates.iter().map(|err| TypeError::DuplicateTypeDefinition { 
                    original: Either::Left(generic_params.params.iter().find(|p| p.value == err.value).unwrap().clone()), 
                    duplicate: Either::Left((**err).clone())
                }).collect();

                return Err(errors)
            }
        }

        Ok(InterfaceDef
        {
            id: Uuid::new_v4(),
            name: interface_id.clone(),
            name_tok: Some(decl.id.clone()),
            pos: Some(decl.id.pos + generic_params.map_or(decl.id.pos, |p| p.open_bracket.pos + p.close_bracket.pos)),
            generic_params: generic_params.map(|p| p.params.iter().map(|p|{ 
                let name = p.value_string().unwrap().clone();
                GenericParam {
                    name,
                }
            }).collect()).unwrap_or_default()
        })

    }
}