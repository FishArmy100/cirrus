use std::collections::HashMap;

use super::token::TokenType;

pub const KW_AS:        &str = "as";
pub const KW_BREAK:     &str = "break";
pub const KW_CONST:     &str = "const";
pub const KW_CONTINUE:  &str = "continue";
pub const KW_ELSE:      &str = "else";
pub const KW_ENUM:      &str = "enum";
pub const KW_FALSE:     &str = "false";
pub const KW_FN:        &str = "fn";
pub const KW_FOR:       &str = "for";
pub const KW_IF:        &str = "if";
pub const KW_IMPL:      &str = "impl";
pub const KW_IN:        &str = "in";
pub const KW_INTERFACE: &str = "interface";
pub const KW_LET:       &str = "let";
pub const KW_MATCH:     &str = "match";
pub const KW_MOD:       &str = "mod";
pub const KW_MUT:       &str = "mut";
pub const KW_RETURN:    &str = "return";
pub const KW_PUB:       &str = "pub";
pub const KW_SELF_TYPE: &str = "Self";
pub const KW_SELF_VAL:  &str = "self";
pub const KW_STRUCT:    &str = "struct";
pub const KW_TRUE:      &str = "true";
pub const KW_TYPE:      &str = "type";
pub const KW_USE:       &str = "use";
pub const KW_WHERE:     &str = "where";
pub const KW_WHILE:     &str = "while";
pub const KW_YIELD:     &str = "yield";

lazy_static::lazy_static! 
{
    pub static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut map = HashMap::new();
        map.insert(KW_AS.into(), TokenType::As);
        map.insert(KW_BREAK.into(), TokenType::Break);
        map.insert(KW_CONST.into(), TokenType::Const);
        map.insert(KW_ELSE.into(), TokenType::Else);
        map.insert(KW_ENUM.into(), TokenType::Enum);
        map.insert(KW_FALSE.into(), TokenType::False);
        map.insert(KW_FN.into(), TokenType::Fn);
        map.insert(KW_FOR.into(), TokenType::For);
        map.insert(KW_IF.into(), TokenType::If);
        map.insert(KW_IMPL.into(), TokenType::Impl);
        map.insert(KW_IN.into(), TokenType::In);
        map.insert(KW_INTERFACE.into(), TokenType::Interface);
        map.insert(KW_LET.into(), TokenType::Let);
        map.insert(KW_MATCH.into(), TokenType::Match);
        map.insert(KW_MOD.into(), TokenType::Mod);
        map.insert(KW_RETURN.into(), TokenType::Return);
        map.insert(KW_PUB.into(), TokenType::Pub);
        map.insert(KW_STRUCT.into(), TokenType::Struct);
        map.insert(KW_SELF_TYPE.into(), TokenType::SelfType);
        map.insert(KW_SELF_VAL.into(), TokenType::SelfVal);
        map.insert(KW_TRUE.into(), TokenType::True);
        map.insert(KW_TYPE.into(), TokenType::Type);
        map.insert(KW_USE.into(), TokenType::Use);
        map.insert(KW_MUT.into(), TokenType::Mut);
        map.insert(KW_WHERE.into(), TokenType::Where);
        map.insert(KW_WHILE.into(), TokenType::While);
        map.insert(KW_YIELD.into(), TokenType::Yield);
        map
    };
}