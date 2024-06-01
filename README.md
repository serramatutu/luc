# luc

🚧🏗️ Under construction... 

`luc` is a personal CLI-based pre-configured request runner I am building on my free-time.


## Motivation

I am not satisfied with most request runners available today (Postman, Insomnia etc) for a couple of reasons:
1. They force you to adopt a very specific, UI-centric workflow;
2. Scripts are not generic enough: you're constrained to Javascript that runs inside their runtime;
3. Running them in CI/CD workflows is a pain;
4. They're not designed with version control as a source of truth. While you can export request collections, the source of truth is the application itself;
5. They're slowly becoming corporate bloatware.


## My solution

Some design principles I'm taking into consideration:
- 100% free software (MIT license);
- All request configurations live in human-readable files checked into version control;
- Request configurations might be written in markdown to also serve as documentation;
- JSON-based RPC via STDIN/STDOUT for running `before` and `after` request hooks (allows for scripting in any language to be used);
- (Possibly) implement thin RPC clients in many languages (python, node, rust, go...);
- Environment is king. Native support for `.env` files.
- Should be very simple to install via `cargo`, binary download or CI template;
- Should be very fast and efficient (greenthreads over multithreaded runtime).

