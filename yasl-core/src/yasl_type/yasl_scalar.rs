use crate::glsl::Glsl;
use std::convert::{TryFrom, TryInto};
use syn::spanned::Spanned;
use syn::{Error, Result};

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

impl From<&YaslScalarType> for Glsl {
    fn from(ty: &YaslScalarType) -> Glsl {
        use YaslScalarType::*;
        Glsl::Expr(
            match ty {
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
