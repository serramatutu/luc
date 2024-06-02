use std::collections::HashMap;
use std::panic;
use std::sync::Arc;
use std::time::Instant;

use serde::Serialize;
use tokio::task::JoinSet;

use crate::api::http_request::{HttpRequest, HttpRequestBuilder};
use crate::context::Context;

/// The result of running a single HTTP request
#[derive(Debug, Serialize)]
pub struct HttpRequestRun {
    /// The builder spec used to produce the request
    builder: Arc<HttpRequestBuilder>,
    /// The context provided to the builder
    ctx: Arc<Context>,
    /// The request result
    result: HttpRequestRunResult,
}

/// All possible completion states for an HTTP request
#[derive(Debug, Serialize)]
pub enum HttpRequestRunResult {
    /// The request ran to completion.
    ///
    /// This does not mean it succeeded as in returning a "good"
    /// status code like 200. It merely means the request ran and we got
    /// results.
    Completed {
        /// The rendered request object produced by the builder
        request: Option<HttpRequest>,
        /// How much time it took to run this request, in milliseconds
        time_ms: u128,
        /// The return status code of this request
        ///
        /// todo: turn this into enum with pretty names like NotFound and Created.
        status_code: u16,
    },
    /// Request got canceled by the runner due to a timeout
    Timeout {
        /// How much time the runner waited before aborting this request
        timeout_ms: u128,
        /// The rendered request object produced by the builder
        request: Option<HttpRequest>,
    },
    /// There was an error during execution of one of the request hooks
    HookError {
        // todo
    },
    /// Request got skipped, even though it was initially provided to the runner.
    Skipped {
        /// The reason why this request was skipped
        reason: String,
    },
    /// Request got canceled by the runner due to something like a keyboard interrupt.
    Cancelled {
        /// The reason why this request was cancelled
        reason: String,
    },
}

/// The result of a suite of requests
#[derive(Debug, Serialize)]
pub struct Run {
    /// How much time it took to run the suite in total.
    ///
    /// Note that this does not necessarily equal the sum of the run
    /// time of all requests, since they might run in parallel or not
    /// depending on the runtime
    time_ms: u128,

    /// The individual runs that were a part of this suite
    runs: Vec<HttpRequestRun>,
}

async fn run_request(builder: Arc<HttpRequestBuilder>, ctx: Arc<Context>) -> HttpRequestRun {
    println!("Running request to {}", builder.request.url);
    HttpRequestRun {
        builder,
        ctx,
        result: HttpRequestRunResult::Cancelled {
            reason: "asdf".to_owned(),
        },
    }
}

/// Run requests with the given context
pub async fn run(builders: Vec<HttpRequestBuilder>, ctx: Context) -> Run {
    let start = Instant::now();
    let len = builders.len();

    let arc_ctx = Arc::new(ctx);

    let mut joinset = JoinSet::new();
    let arc_builders: Vec<_> = builders.into_iter().map(Arc::new).collect();
    let id_map: HashMap<_, _> = arc_builders
        .into_iter()
        .map(|arc_builder| {
            let handle = joinset.spawn(run_request(arc_builder.clone(), arc_ctx.clone()));
            (handle.id(), arc_builder)
        })
        .collect();

    let mut runs = Vec::with_capacity(len);
    while let Some(result) = joinset.join_next().await {
        match result {
            Err(err) => {
                if err.is_cancelled() {
                    runs.push(HttpRequestRun {
                        builder: id_map.get(&err.id()).unwrap().clone(),
                        ctx: arc_ctx,
                        result: HttpRequestRunResult::Cancelled {
                            reason: err.to_string(),
                        },
                    });
                }

                panic::resume_unwind(err.into_panic());
            }
            Ok(val) => {
                runs.push(val);
            }
        }
    }

    Run {
        time_ms: start.elapsed().as_millis(),
        runs,
    }
}
