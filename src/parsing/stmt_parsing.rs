use either::Either;

use crate::{ast::*, lexing::token::{Token, TokenType, ASSIGNMENT_TOKENS}};

use super::{expect_block_expression, expect_expression, expect_type_name, is_expression_and, parse_expression, parse_generic_args, parse_generic_params, pattern_parsing::expect_pattern, token_reader::TokenReader, ParserError, ParserResult};

pub fn expect_statement(reader: &mut TokenReader) -> ParserResult<Statement>
{
    if let Some(statement) = parse_statement(reader)?
    {
        Ok(statement)
    }
    else
    {
        Err(ParserError::ExpectedStatement(reader.current()))    
    }
}

pub fn parse_statement(reader: &mut TokenReader) -> ParserResult<Option<Statement>>
{
    if let Some(expr) = parse_expression_stmt(reader)?
    {
        Ok(Some(expr))
    }
    else if let Some(assign) = parse_assignment(reader)?
    {
        Ok(Some(assign))
    }
    else 
    {
        Ok(None)    
    }
}

fn parse_impl_member(reader: &mut TokenReader) -> ParserResult<Option<(Option<Token>, Statement)>>
{
    if let Some(pub_tok) = reader.check(TokenType::Pub)
    {

    }
    else if let Some(let_stmt) = parse_let(reader)?
    {
        Ok(Some((None, Statement::Let(let_stmt))))
    }
    else if let Some(fn_decl) = parse_fn_decl(reader)?
    {
        Ok(Some((None, Statement::FnDecl(fn_decl))))
    }
    else 
    {
        Ok(None)    
    }
}

fn parse_type_decl(reader: &mut TokenReader) -> ParserResult<Option<TypeDecl>>
{
    let Some(type_tok) = reader.check(TokenType::Type) else {
        return  Ok(None);
    };

    let id = reader.expect(TokenType::Identifier)?;
    let generic_params = parse_generic_params(reader)?;
    let equal = reader.expect(TokenType::Equal)?;
    let type_name = expect_type_name(reader)?;
    let semi_colon = reader.expect(TokenType::SemiColon)?;

    Ok(Some(TypeDecl { 
        type_tok, 
        id, 
        generic_params, 
        equal, 
        type_name, 
        semi_colon
    }))
}

fn parse_enum_decl(reader: &mut TokenReader) -> ParserResult<Option<EnumDecl>>
{
    let Some(enum_tok) = reader.check(TokenType::Enum) else { return Ok(None); };
    let id = reader.expect(TokenType::Identifier)?;
    let generic_params = parse_generic_params(reader)?;
    let where_clause = parse_where_clause(reader)?;
    let open_brace = reader.expect(TokenType::OpenBrace)?;

    let mut members = vec![];
    while let Some(member) = parse_enum_member(reader)?
    {
        members.push(member);
        if reader.check(TokenType::Comma).is_none() { break; }
    }

    let close_brace = reader.expect(TokenType::CloseBrace)?;

    Ok(Some(EnumDecl { 
        enum_tok, 
        id, 
        generic_params, 
        where_clause, 
        open_brace, 
        members, 
        close_brace 
    }))
}

fn parse_enum_member(reader: &mut TokenReader) -> ParserResult<Option<EnumMember>>
{
    let Some(id) = reader.check(TokenType::Identifier) else {
        return Ok(None)
    };

    if let Some(open_brace) = reader.check(TokenType::OpenBrace)
    {
        let mut members = vec![];
        while let Some(member) = parse_enum_struct_member(reader)?
        {
            members.push(member);
            if reader.check(TokenType::Comma).is_none() { break; }
        }

        let close_brace = reader.expect(TokenType::CloseBrace)?;
        
        Ok(Some(EnumMember::Struct { id, open_brace, members, close_brace }))
    }
    else if let Some(open_paren) = reader.check(TokenType::OpenParen)
    {
        let type_name = expect_type_name(reader)?;
        let close_paren = reader.expect(TokenType::CloseParen)?;
        Ok(Some(EnumMember::Single { id, open_paren, type_name, close_paren }))
    }
    else 
    {
        Ok(Some(EnumMember::Basic(id)))    
    }
}

fn parse_enum_struct_member(reader: &mut TokenReader) -> ParserResult<Option<EnumStructMember>>
{
    if !reader.current_is(&[TokenType::Mut, TokenType::Identifier]) { return Ok(None) }

    let mut_tok = reader.check(TokenType::Mut);
    let id = reader.expect(TokenType::Identifier)?;

    let colon = reader.expect(TokenType::Colon)?;
    let type_name = expect_type_name(reader)?;

    let initializer = if let Some(equal) = reader.check(TokenType::Equal) {
        let expression = expect_expression(reader, parse_expression)?;
        Some((equal, expression))
    } else { None };

    Ok(Some(EnumStructMember { 
        mut_tok, 
        id,
        colon, 
        type_name, 
        initializer 
    }))
}

fn parse_interface_decl(reader: &mut TokenReader) -> ParserResult<Option<InterfaceDecl>>
{
    let Some(interface_tok) = reader.check(TokenType::Interface) else { return Ok(None); };
    let id = reader.expect(TokenType::Identifier)?;
    let generic_params = parse_generic_params(reader)?;
    let where_clause = parse_where_clause(reader)?;
    let open_brace = reader.expect(TokenType::OpenBrace)?;

    let mut members = vec![];
    while let Some(member) = parse_fn_decl(reader)?
    {
        members.push(Statement::FnDecl(member));
        if reader.check(TokenType::Comma).is_none() { break; }
    }

    let close_brace = reader.expect(TokenType::CloseBrace)?;

    Ok(Some(InterfaceDecl { 
        interface_tok, 
        id, 
        generic_params, 
        where_clause, 
        open_brace, 
        members, 
        close_brace 
    }))
}

fn parse_struct_decl(reader: &mut TokenReader) -> ParserResult<Option<StructDecl>>
{
    let Some(struct_tok) = reader.check(TokenType::Struct) else { return Ok(None); };
    let id = reader.expect(TokenType::Identifier)?;
    let generic_params = parse_generic_params(reader)?;
    let where_clause = parse_where_clause(reader)?;
    let open_brace = reader.expect(TokenType::OpenBrace)?;

    let mut members = vec![];
    while let Some(member) = parse_struct_member(reader)?
    {
        members.push(member);
        if reader.check(TokenType::Comma).is_none() { break; }
    }

    let close_brace = reader.expect(TokenType::CloseBrace)?;

    Ok(Some(StructDecl { 
        struct_tok, 
        id, 
        generic_params, 
        where_clause, 
        open_brace, 
        members, 
        close_brace 
    }))
}

fn parse_struct_member(reader: &mut TokenReader) -> ParserResult<Option<StructMember>>
{
    if !reader.current_is(&[TokenType::Pub, TokenType::Mut, TokenType::Identifier]) {
        return Ok(None)
    }

    let pub_tok = reader.check(TokenType::Pub);
    let mut_tok = reader.check(TokenType::Mut);
    let id = reader.expect(TokenType::Identifier)?;

    let colon = reader.expect(TokenType::Colon)?;
    let type_name = expect_type_name(reader)?;

    let initializer = if let Some(equal) = reader.check(TokenType::Equal) {
        let expression = expect_expression(reader, parse_expression)?;
        Some((equal, expression))
    } else { None };

    Ok(Some(StructMember { 
        pub_tok, 
        mut_tok, 
        id,
        colon, 
        type_name, 
        initializer 
    }))
}

fn parse_fn_decl(reader: &mut TokenReader) -> ParserResult<Option<FnDecl>>
{
    let Some(fn_tok) = reader.check(TokenType::Fn) else { return Ok(None) };
    let id = reader.expect(TokenType::Identifier)?;
    let generic_params = parse_generic_params(reader)?;

    let open_paren = reader.expect(TokenType::OpenParen)?;
    let mut params = vec![];
    while let Some(param) = parse_fn_param(reader)?
    {
        params.push(param);
        if reader.check(TokenType::Comma).is_none() { break; }
    }
    let close_paren = reader.expect(TokenType::CloseParen)?;

    let arrow = reader.expect(TokenType::ThinArrow)?;
    let return_type = expect_type_name(reader)?;
    let where_clause = parse_where_clause(reader)?;
    let body = if let Some(semi_colon) = reader.check(TokenType::SemiColon) 
    {
        Either::Right(semi_colon)
    }
    else 
    {
        Either::Left(expect_block_expression(reader)?)
    };

    Ok(Some(FnDecl { 
        fn_tok, 
        id, 
        generic_params, 
        open_paren, 
        params, 
        close_paren, 
        arrow, 
        return_type, 
        where_clause, 
        body 
    }))
}

fn parse_fn_param(reader: &mut TokenReader) -> ParserResult<Option<FnParam>>
{
    if reader.is_sequence(&[TokenType::SelfVal]) || reader.is_sequence(&[TokenType::Mut, TokenType::SelfVal])
    {
        let mut_tok = reader.check(TokenType::Mut);
        let self_tok = reader.expect(TokenType::SelfVal)?;

        Ok(Some(FnParam::SelfParam { mut_tok, self_tok }))
    }
    else if reader.is_sequence(&[TokenType::Identifier]) || reader.is_sequence(&[TokenType::Mut, TokenType::Identifier])
    {
        let mut_tok = reader.check(TokenType::Mut);
        let id = reader.expect(TokenType::Identifier)?;
        let colon = reader.expect(TokenType::Colon)?;
        let type_name = expect_type_name(reader)?;

        let default_value = if let Some(equal) = reader.check(TokenType::Equal) {
            let expression = expect_expression(reader, parse_expression)?;
            Some((equal, expression))
        } else { None };

        Ok(Some(FnParam::Normal { 
            mut_tok, 
            id, 
            colon, 
            type_name, 
            default_value
        }))
    }
    else 
    {
        Ok(None)    
    }
}

fn parse_where_clause(reader: &mut TokenReader) -> ParserResult<Option<WhereClause>>
{
    let Some(where_tok) = reader.check(TokenType::Where) else {
        return Ok(None)
    };

    let mut sub_clauses = vec![parse_where_sub_clause(reader)?];
    while let Some(_comma) = reader.check(TokenType::Comma)
    {
        if !reader.current_is(&[TokenType::Identifier]) { break; }
        sub_clauses.push(parse_where_sub_clause(reader)?);
    }

    Ok(Some(WhereClause { where_tok, sub_clauses }))
}

fn parse_where_sub_clause(reader: &mut TokenReader) -> ParserResult<WhereSubClause>
{
    let id = reader.expect(TokenType::Identifier)?;
    let colon = reader.expect(TokenType::Colon)?;

    let mut types = vec![expect_type_name(reader)?];
    while let Some(_plus) = reader.check(TokenType::Plus)
    {
        types.push(expect_type_name(reader)?);
    }

    Ok(WhereSubClause { id, colon, types })
}

fn parse_let(reader: &mut TokenReader) -> ParserResult<Option<LetStmt>>
{
    if let Some(let_tok) = reader.check(TokenType::Let)
    {
        let pattern = expect_pattern(reader)?;

        let type_name = if let Some(colon) = reader.check(TokenType::Colon)
        {
            let type_name = expect_type_name(reader)?;
            Some((colon, type_name))
        }
        else 
        {
            None
        };

        let equal = reader.expect_many(ASSIGNMENT_TOKENS)?;
        let expression = expect_expression(reader, parse_expression)?;
        let else_clause = if let Some(else_tok) = reader.check(TokenType::Else) 
        {
            let block = expect_block_expression(reader)?;
            Some((else_tok, block))
        }
        else 
        {
            None
        };

        let semi_colon = reader.expect(TokenType::SemiColon)?;

        if let Pattern::Identifier { mut_tok, id } = pattern
        {
            Ok(Some(LetStmt {
                let_tok,
                binding: LetBinding::Variable { mut_tok, id },
                type_name,
                equal,
                expression,
                else_clause,
                semi_colon,
            }))
        }
        else 
        {
            Ok(Some(LetStmt {
                let_tok,
                binding: LetBinding::Pattern(pattern),
                type_name,
                equal,
                expression,
                else_clause,
                semi_colon,
            }))
        }
    }
    else 
    {
        Ok(None)
    }
}

fn parse_expression_stmt(reader: &mut TokenReader) -> ParserResult<Option<Statement>>
{
    if let Some(expression) = is_expression_and(reader, |r| r.current_is(&[TokenType::SemiColon]))
    {
        let semi_colon = reader.expect(TokenType::SemiColon)?;
        Ok(Some(Statement::Expression(ExpressionStmt {
            expression,
            semi_colon
        })))
    }
    else 
    {
        Ok(None)    
    }
}

fn parse_use_stmt(reader: &mut TokenReader) -> ParserResult<Option<Statement>>
{
    if let Some(use_tok) = reader.check(TokenType::Use)
    {
        let mut ids = vec![];
        let mut has_dot = false;
        while let Some(id) = reader.check(TokenType::Identifier)
        {
            ids.push(id);
            has_dot = reader.check(TokenType::Dot).is_some()
        }

        if ids.len() == 0
        {
            return Err(ParserError::ExpectedToken(TokenType::Identifier, reader.current()));
        }

        let star = if has_dot { Some(reader.expect(TokenType::Multiply)?) } else { None };
        let semi_colon = reader.expect(TokenType::SemiColon)?;

        Ok(Some(Statement::Use(UseStmt {
            use_tok,
            ids,
            star,
            semi_colon
        })))
    }
    else
    {
        Ok(None)    
    }
}


fn parse_assignment(reader: &mut TokenReader) -> ParserResult<Option<Statement>>
{
    if reader.is_sequence(&[TokenType::Identifier, TokenType::Equal]) ||
       reader.is_sequence(&[TokenType::Identifier, TokenType::PlusEqual]) ||
       reader.is_sequence(&[TokenType::Identifier, TokenType::MinusEqual]) ||
       reader.is_sequence(&[TokenType::Identifier, TokenType::MultiplyEqual]) ||
       reader.is_sequence(&[TokenType::Identifier, TokenType::DivideEqual]) ||
       reader.is_sequence(&[TokenType::Identifier, TokenType::ModulusEqual]) ||
       reader.is_sequence(&[TokenType::Identifier, TokenType::AndEqual]) ||
       reader.is_sequence(&[TokenType::Identifier, TokenType::OrEqual])
    {
        let identifier = reader.advance().unwrap();
        let equal = reader.advance().unwrap();
        let expression = expect_expression(reader, parse_expression)?;
        let semi_colon = reader.expect(TokenType::SemiColon)?;

        Ok(Some(Statement::Assign(AssignStmt {
            identifier,
            equal,
            expression,
            semi_colon
        })))
    }
    else 
    {
        Ok(None)    
    }
}