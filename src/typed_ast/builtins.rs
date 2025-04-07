use std::collections::HashMap;

use uuid::Uuid;

use super::{BinaryOpType, TypeError, TypeGenerator, TypeInfo, TypeInfoResult, TypeRef, UnaryOpType};

pub fn build_builtins() -> HashMap<Uuid, TypeGenerator>
{
    let mut map = HashMap::new();

    let bool_id = Uuid::new_v4();
    let int_id = Uuid::new_v4();
    let float_id = Uuid::new_v4();

    let bool_type = build_bool(&mut map, bool_id.clone());
    let int_type = build_int(&mut map, int_id.clone(), &bool_id);
    let float_type = build_float(&mut map, float_id.clone(), &bool_id);

    map
}

fn build_bool(map: &mut HashMap<Uuid, TypeGenerator>, bool_id: TypeRef) -> TypeRef
{
    let mut bool_type = TypeInfo::new_primary(bool_id.clone(), "Bool");

    bool_type
        .insert_uniform_binary_ops(&[BinaryOpType::And, BinaryOpType::Or, BinaryOpType::NotEqual, BinaryOpType::Equal])
        .insert_uniform_unary(UnaryOpType::Invert);

    map.insert(bool_id.clone(), no_generics_generator(bool_type.clone()));
    bool_id
}

fn build_int(map: &mut HashMap<Uuid, TypeGenerator>, int_id: TypeRef, bool_id: &TypeRef) -> TypeRef
{
    let mut int_type = TypeInfo::new_primary(int_id.clone(), "Int");

    int_type
        .insert_uniform_binary_ops(&[BinaryOpType::Plus, BinaryOpType::Minus, BinaryOpType::Divide, BinaryOpType::Multiply, BinaryOpType::Modulus])
        .insert_isosceles_binary_ops(&[BinaryOpType::Equal, BinaryOpType::NotEqual, BinaryOpType::GreaterThan, BinaryOpType::GreaterThanEqual, BinaryOpType::LessThan, BinaryOpType::LessThanEqual], *bool_id);


    map.insert(int_id.clone(), no_generics_generator(int_type.clone()));
    int_id
}

fn build_float(map: &mut HashMap<Uuid, TypeGenerator>, float_id: TypeRef, bool_id: &TypeRef) -> TypeRef
{
    let mut float_type = TypeInfo::new_primary(float_id.clone(), "Float");

    float_type
        .insert_uniform_binary_ops(&[BinaryOpType::Plus, BinaryOpType::Minus, BinaryOpType::Divide, BinaryOpType::Multiply, BinaryOpType::Modulus])
        .insert_isosceles_binary_ops(&[BinaryOpType::Equal, BinaryOpType::NotEqual, BinaryOpType::GreaterThan, BinaryOpType::GreaterThanEqual, BinaryOpType::LessThan, BinaryOpType::LessThanEqual], *bool_id);


    map.insert(float_id.clone(), no_generics_generator(float_type.clone()));
    float_id
}

fn no_generics_generator(type_info: TypeInfo) -> TypeGenerator
{
    TypeGenerator(Box::new(move |args: Vec<TypeInfo>| -> TypeInfoResult {
        if args.len() != 0
        {
            Err(TypeError::NoGenerics { type_id: type_info.id.clone() })
        }
        else 
        {
            Ok(type_info.clone())
        }
    }))
}