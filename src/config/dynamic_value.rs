use core::fmt;
use std::{env, fs};

use minijinja::{Environment, Error, Value};
use serde::Deserialize;

use crate::config::error::BadConfigError;

#[derive(Deserialize, Clone)]
#[serde(try_from = "String")]
pub struct DynamicValue {
    raw: String,
    tpl: Environment<'static>,
}

impl fmt::Debug for DynamicValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicValue")
            .field("raw", &self.raw)
            .finish()
    }
}

impl DynamicValue {
    pub fn new(input: &str) -> Result<Self, BadConfigError> {
        Self::new_owned(input.to_owned())
    }
    pub fn new_owned(input: String) -> Result<Self, BadConfigError> {
        Self::try_from(input)
    }
}

impl TryFrom<String> for DynamicValue {
    type Error = BadConfigError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut s = Self {
            raw: value.clone(),
            tpl: Environment::new(),
        };
        s.tpl.add_function("file", fn_file);
        s.tpl.add_function("filepath", fn_filepath);
        s.tpl.add_function("env", fn_env);
        s.tpl.add_filter("required", filter_required);
        s.tpl.add_template_owned("", value).map_err(|e| {
            let s = e.template_source().unwrap_or_default();
            BadConfigError::from(format!("template failed to parse: `{s}`: {e}"))
        })?;
        Ok(s)
    }
}

impl DynamicValue {
    pub fn get(&self) -> Result<String, BadConfigError> {
        let s = self
            .tpl
            .get_template("")
            .unwrap()
            .render(serde_yaml::Value::Null)
            .map_err(|e| format!("template failed to execute: `{}`: {e}", self.raw))?;
        Ok(s)
    }
}
impl fmt::Display for DynamicValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.raw.fmt(f)
    }
}

pub fn fn_file(args: &[Value]) -> Result<Value, Error> {
    let file_path = args.get(0).ok_or(Error::new(
        minijinja::ErrorKind::MissingArgument,
        "file path must be supplied",
    ))?;

    let file_path = file_path.as_str().ok_or(Error::new(
        minijinja::ErrorKind::CannotDeserialize,
        "file path must be a string",
    ))?;

    match fs::exists(file_path) {
        Ok(false) | Err(_) => return Ok(Value::UNDEFINED),
        _ => {}
    }

    let value = fs::read_to_string(file_path)
        .map_err(|e| Error::new(minijinja::ErrorKind::CannotDeserialize, e.to_string()))?;

    match value.trim() {
        s if s.is_empty() => Ok(Value::UNDEFINED),
        s => Ok(s.into()),
    }
}
pub fn fn_filepath(args: &[Value]) -> Result<Value, Error> {
    let file_path = args.get(0).ok_or(Error::new(
        minijinja::ErrorKind::MissingArgument,
        "file path must be supplied",
    ))?;

    let fp = file_path.as_str().ok_or(Error::new(
        minijinja::ErrorKind::CannotDeserialize,
        "file path must be a string",
    ))?;

    match fs::exists(fp) {
        Ok(false) | Err(_) => Ok(Value::UNDEFINED),
        _ => Ok(file_path.clone()),
    }
}

pub fn fn_env(args: &[Value]) -> Result<Value, Error> {
    let file_path = args.get(0).ok_or(Error::new(
        minijinja::ErrorKind::MissingArgument,
        "environment variable name must be supplied",
    ))?;

    let file_path = file_path.as_str().ok_or(Error::new(
        minijinja::ErrorKind::CannotDeserialize,
        "environment variable name must be a string",
    ))?;

    let value = match env::var(file_path) {
        Ok(s) => s.into(),
        Err(_) => Value::UNDEFINED,
    };

    Ok(value)
}

pub fn filter_required(value: Value) -> Result<Value, Error> {
    if value.is_undefined() {
        Err(Error::new(
            minijinja::ErrorKind::UndefinedError,
            "this template must render a value, it did not",
        ))
    } else {
        Ok(value)
    }
}
