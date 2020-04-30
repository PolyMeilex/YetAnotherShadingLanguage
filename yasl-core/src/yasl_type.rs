use std::convert::{TryFrom, TryInto};
use syn::spanned::Spanned;
use syn::{Error, Result};

use crate::glsl::Glsl;

mod yasl_scalar;
use yasl_scalar::YaslScalarType;

mod yasl_vec;
use yasl_vec::YaslVecType;

#[derive(Debug)]
pub enum YaslType {
    ScalarType(YaslScalarType),
    Vec(YaslVecType),
    Void,
    // Unknown(String),
}

impl From<&YaslType> for Glsl {
    fn from(ty: &YaslType) -> Glsl {
        use YaslType::*;
        Glsl::Expr(match ty {
            ScalarType(s) => Glsl::from(s).into(),
            Vec(st) => Glsl::from(st).into(),
            Void => "void".into(),
        })
    }
}

impl TryFrom<syn::Type> for YaslType {
    type Error = Error;
    fn try_from(ty: syn::Type) -> Result<Self> {
        use YaslType::*;

        Ok(match ty {
            syn::Type::Path(p) => {
                if let Some(i) = p.path.get_ident() {
                    use YaslScalarType::*;
                    match i.to_string().as_str() {
                        "i32" => ScalarType(Int),
                        "u32" => ScalarType(UInt),
                        "f32" => ScalarType(Float32),
                        "f64" => ScalarType(Float64),
                        "bool" => ScalarType(Bool),
                        _ => return Err(Error::new(i.span(), "Unknown Type")),
                    }
                } else {
                    if p.path.segments.len() == 1 {
                        let segment = &p.path.segments[0];
                        let ident = segment.ident.to_string();
                        let arg = &segment.arguments;

                        if let syn::PathArguments::AngleBracketed(a) = arg {
                            if a.args.len() == 1 {
                                let a = &a.args[0];

                                match ident.as_str() {
                                    "vec2" => YaslType::Vec(YaslVecType::Vec2(a.try_into()?)),
                                    "vec3" => YaslType::Vec(YaslVecType::Vec3(a.try_into()?)),
                                    "vec4" => YaslType::Vec(YaslVecType::Vec4(a.try_into()?)),
                                    _ => return Err(Error::new(ident.span(), "Unknown Type")),
                                }
                            } else {
                                return Err(Error::new(ident.span(), "Unknown Type"));
                            }
                        } else {
                            return Err(Error::new(ident.span(), "Unknown Type"));
                        }
                    } else {
                        return Err(Error::new(p.span(), "Unknown Type"));
                    }
                }
            }
            _ => Void,
        })
    }
}
