use std::collections::HashMap;

use uuid::Uuid;

use super::{operators::{BinaryOp, BinaryOpType, GlobalOperators}, type_key::TypeKey, TypeContext};

pub fn build_builtins() -> TypeContext
{
    let mut operators = GlobalOperators::new();
    let mut types = HashMap::new();

    let bool_type = build_type(&mut types, "Bool");
    let int_type = build_type(&mut types, "Int");
    let float_type = build_type(&mut types, "Float");

    add_number_ops(&mut operators, int_type.clone(), bool_type.clone());
    add_number_ops(&mut operators, float_type.clone(), bool_type.clone());
    add_bool_ops(&mut operators, bool_type.clone());

    TypeContext { 
        operators, 
        types, 
        int_id: int_type.primary_id().unwrap(), 
        float_id: float_type.primary_id().unwrap(), 
        bool_id: bool_type.primary_id().unwrap(),
    }
}

fn add_number_ops(operators: &mut GlobalOperators, num_type: TypeKey, bool_type: TypeKey)
{
    operators.add_binary_op(BinaryOp::make_uniform(num_type.clone(), BinaryOpType::Plus));
    operators.add_binary_op(BinaryOp::make_uniform(num_type.clone(), BinaryOpType::Minus));
    operators.add_binary_op(BinaryOp::make_uniform(num_type.clone(), BinaryOpType::Multiply));
    operators.add_binary_op(BinaryOp::make_uniform(num_type.clone(), BinaryOpType::Divide));
    operators.add_binary_op(BinaryOp::make_uniform(num_type.clone(), BinaryOpType::Modulus));
    
    operators.add_binary_op(BinaryOp::make_isosceles(num_type.clone(), bool_type.clone(), BinaryOpType::Equal));
    operators.add_binary_op(BinaryOp::make_isosceles(num_type.clone(), bool_type.clone(), BinaryOpType::NotEqual));
    operators.add_binary_op(BinaryOp::make_isosceles(num_type.clone(), bool_type.clone(), BinaryOpType::GreaterThan));
    operators.add_binary_op(BinaryOp::make_isosceles(num_type.clone(), bool_type.clone(), BinaryOpType::GreaterThanEqual));
    operators.add_binary_op(BinaryOp::make_isosceles(num_type.clone(), bool_type.clone(), BinaryOpType::LessThan));
    operators.add_binary_op(BinaryOp::make_isosceles(num_type.clone(), bool_type.clone(), BinaryOpType::LessThanEqual));
}

fn add_bool_ops(context: &mut GlobalOperators, bool_type: TypeKey)
{
    context.add_binary_op(BinaryOp::make_uniform(bool_type.clone(), BinaryOpType::And));
    context.add_binary_op(BinaryOp::make_uniform(bool_type.clone(), BinaryOpType::Or));
    context.add_binary_op(BinaryOp::make_uniform(bool_type.clone(), BinaryOpType::Equal));
    context.add_binary_op(BinaryOp::make_uniform(bool_type.clone(), BinaryOpType::NotEqual));
}

fn build_type(types: &mut HashMap<Uuid, TypeKey>, name: &str) -> TypeKey
{
    let t = TypeKey::new_primary(name);
    types.insert(t.primary_id().unwrap(), t.clone());
    t
}