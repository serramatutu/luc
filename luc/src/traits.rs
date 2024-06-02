use std::error::Error;

use crate::context::Context;

pub trait TemplateFile {
    type T;
    type E: Error;
    fn from_template(path: &str, ctx: &Context) -> Result<Vec<Self::T>, Self::E>;
}
