use std::{env, future};

use crate::store::Store;
use crate::types::account::{Account, AccountId, Session};

use chrono::Utc;

use argon2::Config;
use handle_errors::Error;
use rand::Rng;
use tracing::{instrument, Level};
use warp::http::StatusCode;
use warp::Filter;

/// Registers a new account.
///
pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());

    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };

    match store.add_account(account).await {
        None => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Some(e) => Err(warp::reject::custom(e)),
    }
}

/// Logs user in.
/// User stays logged in for X
pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(Error::ArgonLibraryError(e))),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn hash_password(pwd: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();

    argon2::hash_encoded(pwd, &salt, &config).unwrap()
}

fn verify_password(hash: &str, pwd: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, pwd)
}

fn issue_token(account_id: AccountId) -> String {
    let key = env::var("PASETO_KEY").unwrap();
    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::Duration::days(1);

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(key.as_bytes()))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}

#[instrument]
pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| {
        let token = match verify_token(token) {
            Ok(t) => t,
            Err(e) => {
                tracing::event!(target:"book", Level::ERROR, "error when auth {:?}", e);
                return future::ready(Err(warp::reject::custom(e)));
            }
        };
        future::ready(Ok(token))
    })
}

#[instrument]
fn verify_token(token: String) -> Result<Session, handle_errors::Error> {
    let key = env::var("PASETO_KEY").unwrap();
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        key.as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|e| {
        tracing::event!(target:"book", Level::ERROR, "error when validate token: {}", e);
        handle_errors::Error::CannotDecryptToken
    })?;

    serde_json::from_value::<Session>(token).map_err(|_| handle_errors::Error::CannotDecryptToken)
}
