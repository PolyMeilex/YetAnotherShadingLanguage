use super::YaslScalarType;
use crate::convert::{AsGlsl, Glsl};

#[derive(Debug)]
pub enum YaslVecType {
    Vec2(YaslScalarType),
    Vec3(YaslScalarType),
    Vec4(YaslScalarType),
}

impl YaslVecType {}

impl AsGlsl for YaslVecType {
    fn as_glsl(&self) -> Glsl {
        use YaslVecType::*;

        fn get(s: &YaslScalarType) -> &'static str {
            match s {
                YaslScalarType::Int => "i",
                YaslScalarType::UInt => "u",
                YaslScalarType::Bool => "b",
                YaslScalarType::Float32 => "",
                YaslScalarType::Float64 => "d",
            }
        }

        Glsl::Expr(match self {
            Vec2(s) => format!("{}vec2", get(s)),
            Vec3(s) => format!("{}vec3", get(s)),
            Vec4(s) => format!("{}vec4", get(s)),
        })
    }
}
