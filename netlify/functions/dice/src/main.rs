use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    /* handler_fn is deprecated, but we're not going to worry about that right this moment */
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(
    event: ApiGatewayProxyRequest,
    _ctx: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    let path = event.path.unwrap();

    /* the rand crate is pretty heavy-duty. We don't need the overhead of a cryptographically-secure RNG here, so SmallRNG will do. */
    let mut rng = SmallRng::from_entropy();
    let dice = rng.gen_range(1..=10);

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(format!(
            "Hello from {path}. Your roll is {dice}"
        ))),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}
