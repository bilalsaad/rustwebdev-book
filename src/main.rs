#![warn(clippy::all)]

mod profanity;
mod routes;
mod store;
mod types;

use std::env;

use handle_errors::return_error;
use store::Store;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

use clap::Parser;
use dotenv;

/// Q&A web service API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    /// Which errors to log (info warn or error)
    #[clap(short, long, default_value = "warn")]
    log_level: String,
    /// URL for the postgres DB
    #[clap(long, default_value = "localhost")]
    database_host: String,
    /// PORT number for DB.
    #[clap(long, default_value = "9003")]
    database_port: u16,
    /// Database name
    #[clap(long, default_value = "")]
    database_name: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    if let Err(_) = env::var("BAD_WORDS_API_KEY") {
        panic!("BadWords API key not set");
    }
    if let Err(_) = env::var("PASETO_KEY") {
        panic!("PASETO key not set");
    }

    let port = std::env::var("PORT")
        .ok()
        .map(|v| v.parse::<u16>())
        .unwrap_or(Ok(3031));

    let args = Args::parse();
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},book={},warp={}",
            args.log_level, args.log_level, args.log_level
        )
    });

    let store = Store::new(&format!(
        "postgres://postgres:admin1@{}:{}",
        args.database_host, args.database_port
    ))
    .await;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");

    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record
        .with_env_filter(log_filter)
        // Record an event when each span closes, this can be used for routes
        // duration.
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!("get questions request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),)
        }));

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::answer::add_answer);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    let routes = get_questions
        .or(add_question)
        .or(add_answer)
        .or(update_question)
        .or(delete_question)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;
}
