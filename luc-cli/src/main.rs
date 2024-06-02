use clap::Parser;

use luc::api::http_request::HttpRequestBuilder;
use luc::context::Context;
use luc::errors::RequestFileError;
use luc::traits::TemplateFile;

mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::parse();

    let cmd_result: Result<(), Box<dyn std::error::Error>> = match args.command {
        cli::Commands::Run { files } => {
            let ctx = Context::from_environment();

            let builders_raw: Vec<Vec<HttpRequestBuilder>> = files
                .iter()
                .map(|file| HttpRequestBuilder::from_template(file, &ctx))
                .collect::<Result<Vec<Vec<HttpRequestBuilder>>, RequestFileError>>()?;

            let builders: Vec<HttpRequestBuilder> = builders_raw.into_iter().flatten().collect();

            let results = luc::runner::run(builders, ctx).await;

            println!("{:#?}", results);

            Ok(())
        }
    };

    cmd_result?;

    Ok(())
}
