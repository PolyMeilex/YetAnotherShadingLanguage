use std::convert::{TryFrom, TryInto};
use syn::spanned::Spanned;
use syn::{Error, Result};

use crate::convert::{AsGlsl, Glsl};

mod yasl_vec;
use yasl_vec::YaslVecType;

#[derive(Debug)]
pub enum YaslType {
    ScalarType(YaslScalarType),
    Vec(YaslVecType),
    // Vec2(YaslScalarType),
    // Vec2,
    // Vec3,
    // Vec4,
    Void,
    // Unknown(String),
}

#[derive(Debug)]
pub enum YaslScalarType {
    Int,
    UInt,
    Float32,
    Float64,
    Bool,
}
impl TryFrom<syn::Type> for YaslScalarType {
    type Error = Error;
    fn try_from(ty: syn::Type) -> Result<Self> {
        Ok(match ty {
            syn::Type::Path(p) => {
                if let Some(i) = p.path.get_ident() {
                    use YaslScalarType::*;
                    match i.to_string().as_str() {
                        "i32" => Int,
                        "u32" => UInt,
                        "f32" => Float32,
                        "f64" => Float64,
                        "bool" => Bool,
                        _ => return Err(Error::new(i.span(), "Unknown Type")),
                    }
                } else {
                    return Err(Error::new(p.span(), "Unknown Type"));
                }
            }
            _ => return Err(Error::new(ty.span(), "Unknown Type")),
        })
    }
}
impl TryFrom<&syn::GenericArgument> for YaslScalarType {
    type Error = Error;
    fn try_from(ty: &syn::GenericArgument) -> Result<Self> {
        match ty {
            syn::GenericArgument::Type(t) => Ok(t.to_owned().try_into()?),
            _ => Err(Error::new(ty.span(), "Unknown Type")),
        }
    }
}
impl AsGlsl for YaslScalarType {
    fn as_glsl(&self) -> Glsl {
        use YaslScalarType::*;
        Glsl::Expr(
            match self {
                Int => "int",
                UInt => "uint",
                Float32 => "float",
                Float64 => "double",
                Bool => "bool",
            }
            .into(),
        )
    }
}

impl AsGlsl for YaslType {
    fn as_glsl(&self) -> Glsl {
        use YaslType::*;
        Glsl::Expr(match self {
            ScalarType(s) => s.as_glsl().to_string(),
            Vec(st) => st.as_glsl().to_string(),
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

        // Ok(match name.as_str() {
        //     "i32" => Int,
        //     "u32" => UInt,
        //     "f32" => Float32,
        //     "f64" => Float64,
        //     "bool" => Bool,
        //     "vec2" => Vec2,
        //     "vec3" => Vec3,
        //     "vec4" => Vec4,
        //     "void" => Void,
        //     _ => Unknown(name),
        // })
    }
}
