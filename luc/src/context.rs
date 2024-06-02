use minijinja::Value;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    env: HashMap<String, String>,
    secrets: HashMap<String, String>,
}

impl Context {
    pub fn from_environment() -> Self {
        Context {
            env: std::env::vars().collect(),
            // todo: differentiate env vars from secrets
            secrets: HashMap::new(),
        }
    }
}

pub(crate) mod private {
    pub trait ToJinjaContext {
        fn to_jinja_context(&self) -> minijinja::Value;
    }
}

impl private::ToJinjaContext for Context {
    fn to_jinja_context(&self) -> Value {
        Value::from_serialize(self)
    }
}
