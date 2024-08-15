// src/schema/scalars.rs

use async_graphql::{Scalar, Value, InputValueError, InputValueResult, ScalarType};
use bigdecimal::{BigDecimal, FromPrimitive};
use std::str::FromStr;

pub struct GraphQLBigDecimal(pub BigDecimal); // Newtype wrapper around BigDecimal

#[Scalar(name = "BigDecimal")]
impl ScalarType for GraphQLBigDecimal {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => BigDecimal::from_str(&s)
                .map(Self)
                .map_err(|_| InputValueError::custom("Invalid BigDecimal value")),
            Value::Number(num) => num
                .as_f64()
                .and_then(BigDecimal::from_f64) // Make sure FromPrimitive is imported
                .map(Self)
                .ok_or_else(|| InputValueError::custom("Invalid BigDecimal value")),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
