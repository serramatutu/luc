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
pub struct Spec<T> {
    api: String,
    spec: T,
}

type HttpRequestBuilderSpec = Spec<HttpRequestBuilder>;

#[derive(Debug)]
enum MarkdownConfigBlockType {
    Yaml,
    Json,
}

#[derive(Debug)]
struct MarkdownConfigBlock {
    kind: MarkdownConfigBlockType,
    text: String,
}

fn markdown_get_config_blocks(md_raw: &str) -> Result<Vec<MarkdownConfigBlock>, RequestFileError> {
    let mut md_ast = ok!(
        markdown::to_mdast(md_raw, &markdown::ParseOptions::gfm()),
        RequestFileError::SyntaxError
    );

    let children = md_ast.children_mut().ok_or(RequestFileError::EmptyFile)?;

    if children.is_empty() {
        return Err(RequestFileError::EmptyFile);
    }

    children
        .iter_mut()
        .filter_map(|block| match block {
            markdown::mdast::Node::Code(code) => match code.meta.as_deref() {
                Some("luc") => Some(code),
                _ => None,
            },
            _ => None,
        })
        .map(|code| {
            let lang = code
                .lang
                .as_ref()
                .ok_or(RequestFileError::UnsupportedBlockType)?;
            let block = MarkdownConfigBlock {
                kind: match lang.as_str() {
                    "yaml" => MarkdownConfigBlockType::Yaml,
                    "json" => MarkdownConfigBlockType::Json,
                    _ => return Err(RequestFileError::UnsupportedBlockType),
                },
                text: std::mem::take(&mut code.value),
            };

            Ok(block)
        })
        .collect()
}

impl TemplateFile for HttpRequestBuilder {
    type T = HttpRequestBuilder;
    type E = RequestFileError;
    fn from_template(
        path: &str,
        ctx: &Context,
    ) -> Result<Vec<HttpRequestBuilder>, RequestFileError> {
        let md_raw = ok!(
            std::fs::read_to_string(path),
            RequestFileError::DoesNotExist
        );

        let blocks = markdown_get_config_blocks(&md_raw)?;

        let mut env = Environment::new();
        let builders: Result<Vec<HttpRequestBuilder>, RequestFileError> = blocks
            .into_iter()
            .enumerate()
            .map(|(i, block)| {
                let template_name = format!("{}.{}", path, i);

                env.add_template_owned(template_name.to_owned(), block.text)
                    .unwrap();
                let md_template = env.get_template(template_name.as_str()).unwrap();
                let render = md_template.render(ctx.to_jinja_context()).unwrap();

                let builder_spec: HttpRequestBuilderSpec = match block.kind {
                    MarkdownConfigBlockType::Yaml => ok!(
                        serde_yaml::from_str(&render),
                        RequestFileError::InvalidBlockSpec
                    ),
                    MarkdownConfigBlockType::Json => ok!(
                        serde_json::from_str(&render),
                        RequestFileError::InvalidBlockSpec
                    ),
                };

                if builder_spec.api != "luc.api.http_request.HttpRequestBuilder" {
                    return Err(RequestFileError::InvalidBlockApi);
                }

                Ok(builder_spec.spec)
            })
            .collect();

        builders
    }
}
