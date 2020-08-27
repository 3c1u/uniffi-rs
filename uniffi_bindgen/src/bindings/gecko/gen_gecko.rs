/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path::Path;

use anyhow::Result;
use askama::Template;
use heck::{CamelCase, MixedCase};

use crate::interface::*;

// Some config options for the caller to customize the generated Gecko bindings.
// Note that this can only be used to control details *that do not affect the
// underlying component*, since the details of the underlying component are
// entirely determined by the `ComponentInterface`.
pub struct Config {
    // ...
}

impl Config {
    pub fn from(_ci: &ComponentInterface) -> Self {
        Config {
            // ...
        }
    }
}

#[derive(Template)]
#[template(syntax = "c", escape = "none", path = "HeaderTemplate.h")]
pub struct Header<'config, 'ci> {
    config: &'config Config,
    ci: &'ci ComponentInterface,
}

impl<'config, 'ci> Header<'config, 'ci> {
    pub fn new(config: &'config Config, ci: &'ci ComponentInterface) -> Self {
        Self { config: config, ci }
    }
}

#[derive(Template)]
#[template(syntax = "webidl", escape = "none", path = "WebIDLTemplate.webidl")]
pub struct WebIdl<'config, 'ci> {
    config: &'config Config,
    ci: &'ci ComponentInterface,
}

impl<'config, 'ci> WebIdl<'config, 'ci> {
    pub fn new(config: &'config Config, ci: &'ci ComponentInterface) -> Self {
        Self { config: config, ci }
    }
}

pub enum WebIdlReturnPosition<'a> {
    OutParam(&'a Type),
    Return(&'a Type),
    Void,
}

impl<'a> WebIdlReturnPosition<'a> {
    pub fn is_out_param(&self) -> bool {
        matches!(self, WebIdlReturnPosition::OutParam(_))
    }
}

#[derive(Template)]
#[template(syntax = "cpp", escape = "none", path = "wrapper.cpp")]
pub struct GeckoWrapper<'config, 'ci> {
    config: &'config Config,
    ci: &'ci ComponentInterface,
}

impl<'config, 'ci> GeckoWrapper<'config, 'ci> {
    pub fn new(config: &'config Config, ci: &'ci ComponentInterface) -> Self {
        Self { config: config, ci }
    }

    /// Indicates how a WebIDL return value is reflected in C++. Some are passed
    /// as out parameters, others are returned directly. This helps the template
    /// generate the correct declaration.
    pub fn ret_position_cpp(&self, func: &'ci Function) -> WebIdlReturnPosition<'ci> {
        func.return_type()
            .map(|type_| match type_ {
                Type::String => WebIdlReturnPosition::OutParam(type_),
                Type::Optional(_) => WebIdlReturnPosition::OutParam(type_),
                Type::Record(_) => WebIdlReturnPosition::OutParam(type_),
                Type::Sequence(_) => WebIdlReturnPosition::OutParam(type_),
                _ => WebIdlReturnPosition::Return(type_),
            })
            .unwrap_or(WebIdlReturnPosition::Void)
    }

    /// Returns a suitable default value from the WebIDL function, based on its
    /// return type. This default value is what's returned if the function
    /// throws an exception.
    pub fn ret_default_value_cpp(&self, func: &Function) -> Option<String> {
        func.return_type().and_then(|type_| {
            Some(match type_ {
                Type::Int8
                | Type::UInt8
                | Type::Int16
                | Type::UInt16
                | Type::Int32
                | Type::UInt32
                | Type::Int64
                | Type::UInt64 => "0".into(),
                Type::Float32 => "0.0f".into(),
                Type::Float64 => "0.0".into(),
                Type::Boolean => "false".into(),
                Type::Enum(_) => panic!("[TODO: ret_default_cpp({:?})]", type_),
                Type::Object(_) => "nullptr".into(),
                Type::String | Type::Record(_) | Type::Optional(_) | Type::Sequence(_) => {
                    return None
                }
                Type::Error(name) => panic!("[TODO: ret_type_cpp({:?})]", type_),
            })
        })
    }
}

/// Filters for our Askama templates above. These output C++, XPIDL, and
/// WebIDL.
mod filters {
    use super::*;
    use std::fmt;

    /// Declares a WebIDL type in the interface for this library.
    pub fn type_webidl(type_: &Type) -> Result<String, askama::Error> {
        Ok(match type_ {
            Type::Int8 => "byte".into(),
            Type::UInt8 => "octet".into(),
            Type::Int16 => "short".into(),
            Type::UInt16 => "unsigned short".into(),
            Type::Int32 => "long".into(),
            Type::UInt32 => "unsigned long".into(),
            Type::Int64 => "long long".into(),
            Type::UInt64 => "unsigned long long".into(),
            Type::Float32 => "float".into(),
            // Note: Not `unrestricted double`; we don't want to allow NaNs
            // and infinity.
            Type::Float64 => "double".into(),
            Type::Boolean => "boolean".into(),
            Type::String => "DOMString".into(),
            Type::Enum(name) | Type::Record(name) | Type::Object(name) => class_name_webidl(name)?,
            Type::Error(name) => panic!("[TODO: type_webidl({:?})]", type_),
            Type::Optional(type_) => format!("{}?", type_webidl(type_)?),
            Type::Sequence(type_) => format!("sequence<{}>", type_webidl(type_)?),
        })
    }

    /// Declares a C type in the `extern` declarations.
    pub fn type_ffi(type_: &FFIType) -> Result<String, askama::Error> {
        Ok(match type_ {
            FFIType::Int8 => "int8_t".into(),
            FFIType::UInt8 => "uint8_t".into(),
            FFIType::Int16 => "int16_t".into(),
            FFIType::UInt16 => "uint16_t".into(),
            FFIType::Int32 => "int32_t".into(),
            FFIType::UInt32 => "uint32_t".into(),
            FFIType::Int64 => "int64_t".into(),
            FFIType::UInt64 => "uint64_t".into(),
            FFIType::Float32 => "float".into(),
            FFIType::Float64 => "double".into(),
            FFIType::RustBuffer => "RustBuffer".into(),
            FFIType::RustString => "char*".into(),
            FFIType::RustError => "NativeRustError".into(),
            FFIType::ForeignStringRef => "const char*".into(),
        })
    }

    /// Declares the type of an argument for the C++ binding.
    pub fn arg_type_cpp(type_: &Type) -> Result<String, askama::Error> {
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
            | Type::Boolean => type_cpp(type_)?,
            Type::String => "const nsAString&".into(),
            Type::Enum(name) => name.into(),
            Type::Record(name) | Type::Object(name) => format!("const {}&", class_name_cpp(name)?),
            Type::Error(name) => panic!("[TODO: type_cpp({:?})]", type_),
            // Nullable objects might be passed as pointers, not sure?
            Type::Optional(type_) => format!("const Nullable<{}>&", type_cpp(type_)?),
            Type::Sequence(type_) => format!("const Sequence<{}>&", type_cpp(type_)?),
        })
    }

    pub fn type_cpp(type_: &Type) -> Result<String, askama::Error> {
        Ok(match type_ {
            Type::Int8 => "int8_t".into(),
            Type::UInt8 => "uint8_t".into(),
            Type::Int16 => "int16_t".into(),
            Type::UInt16 => "uint16_t".into(),
            Type::Int32 => "int32_t".into(),
            Type::UInt32 => "uint32_t".into(),
            Type::Int64 => "int64_t".into(),
            Type::UInt64 => "uint64_t".into(),
            Type::Float32 => "float".into(),
            Type::Float64 => "double".into(),
            Type::Boolean => "bool".into(),
            Type::String => "nsString".into(),
            Type::Enum(name) | Type::Record(name) => class_name_cpp(name)?,
            Type::Object(name) => format!("RefPtr<{}>", class_name_cpp(name)?),
            Type::Error(name) => panic!("[TODO: type_cpp({:?})]", type_),
            Type::Optional(type_) => format!("Nullable<{}>", type_cpp(type_)?),
            Type::Sequence(type_) => format!("nsTArray<{}>", type_cpp(type_)?),
        })
    }

    /// Declares the type of a return value from C++.
    pub fn ret_type_cpp(type_: &Type) -> Result<String, askama::Error> {
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
            | Type::Boolean
            | Type::Enum(_) => type_cpp(type_)?,
            Type::String => "nsAString&".into(),
            Type::Object(name) => format!("already_AddRefed<{}>", class_name_cpp(name)?),
            Type::Error(name) => panic!("[TODO: ret_type_cpp({:?})]", type_),
            Type::Record(_) | Type::Optional(_) | Type::Sequence(_) => {
                format!("{}&", type_cpp(type_)?)
            }
        })
    }

    pub fn var_name_webidl(nm: &dyn fmt::Display) -> Result<String, askama::Error> {
        Ok(nm.to_string().to_mixed_case())
    }

    pub fn enum_variant_webidl(nm: &dyn fmt::Display) -> Result<String, askama::Error> {
        Ok(nm.to_string().to_mixed_case())
    }

    pub fn class_name_webidl(nm: &dyn fmt::Display) -> Result<String, askama::Error> {
        Ok(nm.to_string().to_camel_case())
    }

    pub fn class_name_cpp(nm: &dyn fmt::Display) -> Result<String, askama::Error> {
        Ok(nm.to_string().to_camel_case())
    }

    pub fn fn_name_webidl(nm: &dyn fmt::Display) -> Result<String, askama::Error> {
        Ok(nm.to_string().to_mixed_case())
    }

    /// For interface implementations, function and methods names are
    // UpperCamelCase, even though they're mixedCamelCase in WebIDL.
    pub fn fn_name_cpp(nm: &dyn fmt::Display) -> Result<String, askama::Error> {
        Ok(nm.to_string().to_camel_case())
    }

    pub fn field_name_cpp(nm: &str) -> Result<String, askama::Error> {
        Ok(format!("m{}", nm.to_camel_case()))
    }

    pub fn enum_variant_cpp(nm: &dyn fmt::Display) -> Result<String, askama::Error> {
        Ok(nm.to_string().to_camel_case())
    }

    pub fn lift_cpp(name: &dyn fmt::Display, type_: &Type) -> Result<String, askama::Error> {
        let ffi_type = FFIType::from(type_);
        Ok(format!(
            "detail::ViaFfi<{}, {}>::Lift({})",
            type_cpp(type_)?,
            type_ffi(&ffi_type)?,
            name
        ))
    }

    pub fn lower_cpp(name: &dyn fmt::Display, type_: &Type) -> Result<String, askama::Error> {
        let ffi_type = FFIType::from(type_);
        Ok(format!(
            "detail::ViaFfi<{}, {}>::Lower({})",
            type_cpp(type_)?,
            type_ffi(&ffi_type)?,
            name
        ))
    }
}
