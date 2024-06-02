use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestFileError {
    #[error("request file does not exist")]
    DoesNotExist,

    #[error("request file is not valid markdown")]
    SyntaxError,

    #[error("request file is empty")]
    EmptyFile,

    #[error("request file prelude must be a YAML or JSON code block")]
    UnsupportedPreludeType,

    #[error("request file prelude api must be luc.api.v1.HttpRequestBuilder")]
    InvalidPreludeApi,

    #[error("request file prelude spec must conform to struct luc.api.v1.HttpRequestBuilder")]
    InvalidPreludeSpec,
}
