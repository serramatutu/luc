use std::error::Error;

use crate::context::Context;

pub trait TemplateFile<T, E: Error> {
    fn from_template(path: &str, ctx: Context) -> Result<Vec<T>, E>;
}
