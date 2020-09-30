/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::MergeWith;
use anyhow::Result;
use askama::Template;
use heck::{CamelCase, MixedCase, ShoutySnakeCase};
use serde::{Deserialize, Serialize};

use crate::interface::*;

// Some config options for it the caller wants to customize the generated C#.
// Note that this can only be used to control details of the C# *that do not affect the underlying component*,
// sine the details of the underlying component are entirely determined by the `ComponentInterface`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    // No config options yet.
}

impl From<&ComponentInterface> for Config {
    fn from(_ci: &ComponentInterface) -> Self {
        Config {}
    }
}

impl Config {
    pub fn from(_ci: &ComponentInterface) -> Self {
        Config {
            // No config options yet
        }
    }
}

impl MergeWith for Config {
    fn merge_with(&self, _other: &Self) -> Self {
        Config {}
    }
}

#[derive(Template)]
#[template(syntax = "cs", escape = "none", path = "wrapper.cs")]
pub struct CSharpWrapper<'config, 'ci> {
    _config: &'config Config,
    _ci: &'ci ComponentInterface,
}
impl<'c, 'a> CSharpWrapper<'c, 'a> {
    pub fn new(_config: &'c Config, _ci: &'a ComponentInterface) -> Self {
        Self { _config, _ci }
    }
}

#[allow(unused)]
mod filters {
    use super::*;
    use std::fmt;

    pub fn type_ffi(type_: &FFIType) -> Result<String, askama::Error> {
        Ok(match type_ {
            FFIType::Int8 => "ctypes.c_int8".to_string(),
            FFIType::UInt8 => "ctypes.c_uint8".to_string(),
            FFIType::Int16 => "ctypes.c_int16".to_string(),
            FFIType::UInt16 => "ctypes.c_uint16".to_string(),
            FFIType::Int32 => "ctypes.c_int32".to_string(),
            FFIType::UInt32 => "ctypes.c_uint32".to_string(),
            FFIType::Int64 => "ctypes.c_int64".to_string(),
            FFIType::UInt64 => "ctypes.c_uint64".to_string(),
            FFIType::Float32 => "ctypes.c_float".to_string(),
            FFIType::Float64 => "ctypes.c_double".to_string(),
            FFIType::RustCString => "ctypes.c_voidp".to_string(),
            FFIType::RustBuffer => "RustBuffer".to_string(),
            FFIType::RustError => "RustError".to_string(),
            FFIType::ForeignBytes => "ForeignBytes".to_string(),
        })
    }

    pub fn class_name_cs(nm: &dyn fmt::Display) -> Result<String, askama::Error> {
        Ok(nm.to_string().to_camel_case())
    }

    pub fn fn_name_cs(nm: &dyn fmt::Display) -> Result<String, askama::Error> {
        Ok(nm.to_string().to_mixed_case())
    }

    pub fn var_name_cs(nm: &dyn fmt::Display) -> Result<String, askama::Error> {
        Ok(nm.to_string().to_mixed_case())
    }

    pub fn enum_name_cs(nm: &dyn fmt::Display) -> Result<String, askama::Error> {
        Ok(nm.to_string().to_shouty_snake_case())
    }

    pub fn coerce_cs(nm: &dyn fmt::Display, type_: &Type) -> Result<String, askama::Error> {
        Ok(match type_ {
            Type::Int8
            | Type::UInt8
            | Type::Int16
            | Type::UInt16
            | Type::Int32
            | Type::UInt32
            | Type::Int64
            | Type::UInt64
            | Type::Float32
            | Type::Float64
            | Type::String
            | Type::Boolean
            | Type::Object(_)
            | Type::Error(_) => format!("{} = {}", nm, nm),
            Type::Enum(type_name) => format!("{} = {}({})", nm, type_name, nm),
            Type::Record(type_name) => format!("{} = {}._coerce({})", nm, type_name, nm),
            Type::Optional(t) => format!("(None if {} is None else {})", nm, coerce_cs(nm, t)?),
            Type::Sequence(t) => format!("({} for x in {})", coerce_cs(&"x", t)?, nm), // TODO: name hygiene,
            Type::Map(t) => format!(
                "({}:{} for (k, v) in {}.items())",
                coerce_cs(&"k", t)?,
                coerce_cs(&"v", t)?,
                nm
            ),
        })
    }

    pub fn lower_cs(nm: &dyn fmt::Display, type_: &Type) -> Result<String, askama::Error> {
        Ok(match type_ {
            Type::Int8
            | Type::Int16
            | Type::Int32
            | Type::Int64
            | Type::UInt8
            | Type::UInt16
            | Type::UInt32
            | Type::UInt64
            | Type::Float32
            | Type::Float64
            | Type::Boolean => nm.to_string(),
            Type::Enum(_) => format!("{}.value", nm),
            Type::String => format!("{}.encode('utf-8')", nm),
            Type::Record(type_name) => format!("{}._lower({})", type_name, nm),
            Type::Optional(_type) => format!(
                "lowerOptional({}, lambda buf, v: {})",
                nm,
                lower_into_cs(&"buf", &"v", type_)?
            ),
            _ => panic!("[TODO: lower_py({:?})]", type_),
        })
    }

    pub fn lower_into_cs(
        nm: &dyn fmt::Display,
        target: &dyn fmt::Display,
        type_: &Type,
    ) -> Result<String, askama::Error> {
        let nm = var_name_cs(nm)?;
        Ok(match type_ {
            Type::Float64 => format!("{}.putDouble({})", target, nm),
            Type::UInt32 => format!("{}.putInt({})", target, nm),
            Type::String => format!("{}.putString({})", target, nm),
            Type::Record(type_name) => format!("{}._lowerInto({}, {})", type_name, nm, target),
            _ => panic!("[TODO: lower_into_py({:?})]", type_),
        })
    }

    pub fn lift_cs(nm: &dyn fmt::Display, type_: &Type) -> Result<String, askama::Error> {
        Ok(match type_ {
            Type::Int8
            | Type::Int16
            | Type::Int32
            | Type::Int64
            | Type::UInt8
            | Type::UInt16
            | Type::UInt32
            | Type::UInt64
            | Type::Float32
            | Type::Float64
            | Type::Boolean => format!("{}", nm),
            Type::Enum(type_name) => format!("{}({})", type_name, nm),
            Type::String => format!("liftString({})", nm),
            Type::Record(type_name) => format!("{}._lift({})", type_name, nm),
            Type::Optional(type_) => format!(
                "liftOptional({}, lambda buf: {})",
                nm,
                lift_from_cs(&"buf", type_)?
            ),
            Type::Sequence(type_) => format!(
                "liftSequence({}, lambda buf: {})",
                nm,
                lift_from_cs(&"buf", type_)?
            ),
            _ => panic!("[TODO: lift_py({:?})]", type_),
        })
    }

    pub fn lift_from_cs(nm: &dyn fmt::Display, type_: &Type) -> Result<String, askama::Error> {
        Ok(match type_ {
            Type::UInt32 => format!("{}.getInt()", nm),
            Type::Float64 => format!("{}.getDouble()", nm),
            Type::Record(type_name) => format!("{}._liftFrom({})", type_name, nm),
            Type::String => format!("{}.getString()", nm),
            _ => panic!("[TODO: lift_from_py({:?})]", type_),
        })
    }
}