use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, IntoResponse, Request, RequestExt,
};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use serde_json::json;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt() // This is a logging/diagnostic tool
        .with_ansi(false)
        .without_time()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    run(service_fn(roll_dice)).await // Here's where we do our logic
}

pub async fn roll_dice(event: Request) -> Result<impl IntoResponse, Error> {
    /* These variables will go into the response we construct at the end, once we know what their values are */
    let status: StatusCode;
    let message: String;
    let mut rolls: Vec<u8> = Vec::new();

    /* If you're not familiar with Rust, some of the operations from here may seem odd.
     * Without getting too deep into the weeds, Rust has no null value.
     * Instead, optional things in Rust are usually wrapped in an `Option` enum,
     * which always unwraps to either `Some(the_thing)` or `None()`.
     * We're required by the compiler to consider both possibilities.
     */

    /* Let's check our query params. The event that triggered our function has a method that
     * gives them to us as an Option. So, first things first: do we have a Some or a None?
     */
    if let Some(params) = event.query_string_parameters_ref() {
        /* The paramaters are a specialized map type, which has a method which will give us
         * the first value of any given param key in (as usual) another Option.
         * Here, we'll use unwrap_or() to extract the value or use '1' as a default.
         * We'll convert the result of that to an 8-bit unsigned int at the same time. */
        let count = u8::from_str(params.first("count").unwrap_or("1"))?;

        /* We know the finite options to expect from `sides`, so we'll do some pattern matching
         * to convert that to a u8 as well. */
        let sides: u8 = match params.first("sides") {
            Some("d2") => 2,
            Some("d4") => 4,
            Some("d6") => 6,
            Some("d8") => 8,
            Some("d10") => 10,
            Some("d12") => 12,
            Some("d20") => 20,
            _ => 1, // This branch arm handles anything we didn't specifically cover above, including a None
        };

        /* We have our dice. Now let's roll them. */

        /* the rand crate is pretty heavy-duty. We don't need the overhead of a cryptographically-secure RNG here, so SmallRNG will do. */
        let mut rng = SmallRng::from_entropy();

        for _ in 0..count {
            rolls.push(rng.gen_range(1..=sides));
        }

        let sum: u8 = rolls.iter().sum();
        message = format!("You rolled {}", sum);

        status = StatusCode::OK;
    } else {
        /* If we get here, it means our request had no query parameters at all. We'll treat that as a 400 error.
         * And yes, we handled default values for both `size` and `count` earlier, so we're fine if the request is ?bob=hi I guess. */
        status = StatusCode::BAD_REQUEST;
        message = "There was a problem with your dice.".to_string();
    }

    /* Now we build our HTTP response. */
    let response = Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": message,
              "rolls": rolls
            })
            .to_string(),
        )
        .map_err(Box::new)?;

    /* This whole function actually returns a Result, which is a lot like an Option except for fallible things rather than optional.
     * Instead of Some(thing) and None, we have Ok(thing) and Error(err).
     * Anywhere you saw a ? earlier in the function was a Result being either unwrapped if Ok or returned early as the Error. */
    Ok(response)
}
