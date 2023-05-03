---
layout: layouts/base.njk
tags: project
eleventyNavigation:
  key: Howto
  title: How Do?
  order: 3
---
## How To Function

So let's say we've got a pretty simple Eleventy application running on Netlify, and we've got some absolutely stellar dynamic functionality to implement. How do we go about that? 

Well, how do we do it with JavaScript? On Netlify, it's pretty simple: we just stick something in `netlify/functions`, and the name of the subdirectory or .js file determines the endpoint. The function has to export a `handler()` method, and that's what gets called when a request comes to the endpoint. Here's an example lifted directly from [Netlify's documentation]((https://docs.netlify.com/functions/build/?fn-language=js)):

```js
exports.handler = async function (event, context) {
  return {
    statusCode: 200,
    body: JSON.stringify({ message: "Hello World" }),
  };
};
```

Netlify supports two other langauges besides JavaScript and TypeScript though, and that's what we're interested in looking at today.

## Go

Setting up a Go function is actually almost exactly the same as our JavaScript function. We do have an additional prerequisite of [installing Go](https://go.dev/doc/install), at least assuming you want to actually run the code locally. Then, we add a new folder with the name we want to use for our endpoint within our function directory, and initialize a new Go project inside it with `go mod init <function name>`. 

(Go modules require a `go.mod` properties file in addition to any code files, similar to `package.json`, so you can't have a function like `netlify/functions/hello.go` -- you'll always need the subdirectory with them.)

Here's Netlify's example Go function:

```go
package main

import (
  "github.com/aws/aws-lambda-go/events"
  "github.com/aws/aws-lambda-go/lambda"
)

func handler(request events.APIGatewayProxyRequest) (*events.APIGatewayProxyResponse, error) {
  return &events.APIGatewayProxyResponse{
    StatusCode:        200,
    Body:              "Hello, World!",
  }, nil
}

func main() {
  lambda.Start(handler)
}

```

This has a bit more going on than the JavaScript version, but we can see it follows the same principles: the program has a special `handler()` function that receives information about the request that called it, and returns an HTTP response with a status code and message. A big chunk of the magic comes from the [AWS Lambda SDK](https://docs.aws.amazon.com/sdk-for-go/api/service/lambda/), which actually runs the handler and defines what the request and response types.

That's a bit more explicit than the JavaScript version, which isn't surprising: Go functions are going to be compiled, standalone binaries, while JavaScript can probably be imported directly into the environment that Netlify spins up for the function.

## Rust

Netlify's support for Rust isn't 100% fully baked yet, but it does work. Like Go, it's going to be a lot easier to actually write our function [if we have Rust installed](https://www.rust-lang.org/tools/install). We'll also need to enable Rust functions for our site with an env variable, which we can do by running `netlify env:set NETLIFY_EXPERIMENTAL_BUILD_RUST_SOURCE true`. (Or by setting it manually in the site settings if you prefer.)

Rust, like Go, is a compiled language that you'll need a full subdirectory for. You can set one up by running `cargo new <function name>` in `netlify functions`. You'll also want to add some [appropriate gitignore rules](https://github.com/github/gitignore/blob/main/Rust.gitignore) at this point, because Rust is going to add a lot of build files you don't want in your repo.

It's worth mentioning that Netlify provides a starter template, so you could use that: `netlify functions:create --language=rust`. Following the wizard, you'll want to set it up as a Serverless function on the default path. The problem you'll run into here is that the template is fairly out of date, and it actually won't build as-is. If you want to use it, you'll need to edit `Cargo.toml`: change the value of `package.edition` from 2018 to 2021, and update simple_logger from 1.16.0 to 4.1. The other dependencies are out of date too but not as critically. 

My advice though would be to grab the example code from the AWS SDK for Rust's [Github repo](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/README.md) for your starting point:

```rust
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}
```

Same as we've seen before: we define a handler function (`func()` this time instead of `handler()`) to do the real work of our function, which returns a very simple JSON response. We pass the handler to the AWS Lambda library, which presumably knows how to pass along the event and context data from the API request that's calling the function.

Next up: [our demo function](/demo)