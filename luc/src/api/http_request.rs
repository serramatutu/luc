use minijinja::Environment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::option::Option;

use crate::context::private::ToJinjaContext;
use crate::context::Context;
use crate::errors::RequestFileError;
use crate::traits::TemplateFile;

#[derive(Debug, Serialize, Deserialize)]
pub enum HttpRequestMethod {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    POST,
    PUT,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRequest {
    method: HttpRequestMethod,
    url: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRequestBuilderHooks {
    before: Option<String>,
    after: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRequestBuilder {
    request: HttpRequest,
    hooks: HttpRequestBuilderHooks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpRequestBuilderSpec {
    api: String,
    spec: HttpRequestBuilder,
}

#[derive(Debug)]
enum MarkdownPreludeType {
    Yaml,
    Json,
}

#[derive(Debug)]
struct MarkdownPrelude {
    kind: MarkdownPreludeType,
    text: String,
}

fn markdown_get_prelude(md_raw: &str) -> Result<MarkdownPrelude, RequestFileError> {
    let mut md_ast = ok!(
        markdown::to_mdast(md_raw, &markdown::ParseOptions::gfm()),
        RequestFileError::SyntaxError
    );

    let children = md_ast.children_mut().ok_or(RequestFileError::EmptyFile)?;

    if children.is_empty() {
        return Err(RequestFileError::EmptyFile);
    }

    let code = match children.swap_remove(0) {
        markdown::mdast::Node::Code(code) => code,
        _ => return Err(RequestFileError::UnsupportedPreludeType),
    };

    let lang = code.lang.ok_or(RequestFileError::UnsupportedPreludeType)?;
    let prelude = MarkdownPrelude {
        kind: match lang.as_str() {
            "yaml" => MarkdownPreludeType::Yaml,
            "json" => MarkdownPreludeType::Json,
            _ => return Err(RequestFileError::UnsupportedPreludeType),
        },
        text: code.value,
    };

    Ok(prelude)
}

impl TemplateFile<HttpRequestBuilder, RequestFileError> for HttpRequestBuilder {
    fn from_template(path: &str, ctx: Context) -> Result<HttpRequestBuilder, RequestFileError> {
        let md_raw = ok!(
            std::fs::read_to_string(path),
            RequestFileError::DoesNotExist
        );

        let prelude = markdown_get_prelude(&md_raw)?;

        let mut env = Environment::new();
        env.add_template(path, &prelude.text).unwrap();
        let md_template = env.get_template(path).unwrap();

        let render = md_template.render(ctx.to_jinja_context()).unwrap();

        let builder_spec: HttpRequestBuilderSpec = match prelude.kind {
            MarkdownPreludeType::Yaml => ok!(
                serde_yaml::from_str(&render),
                RequestFileError::UnsupportedPreludeType
            ),
            MarkdownPreludeType::Json => ok!(
                serde_json::from_str(&render),
                RequestFileError::UnsupportedPreludeType
            ),
        };

        Ok(builder_spec.spec)
    }
}
