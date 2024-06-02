use luc::api::http_request::HttpRequestBuilder;
use luc::context::Context;
use luc::traits::TemplateFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Context::from_environment();

    let builder = HttpRequestBuilder::from_template("examples/python/posts/all.md", ctx)?;

    println!("{:#?}", builder);

    Ok(())
}
