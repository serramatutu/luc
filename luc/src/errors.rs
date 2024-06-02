use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestFileError {
    #[error("request file does not exist")]
    DoesNotExist,

    #[error("request file is not valid markdown")]
    SyntaxError,

    #[error("request file is empty")]
    EmptyFile,

    #[error("request code block must be YAML or JSON")]
    UnsupportedBlockType,

    #[error("request code block api must be luc.api.http_request.HttpRequestBuilder")]
    InvalidBlockApi,

    #[error(
        "request code block spec must conform to struct luc.api.http_request.HttpRequestBuilder"
    )]
    InvalidBlockSpec,
}
