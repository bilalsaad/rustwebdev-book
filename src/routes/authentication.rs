use crate::store::Store;
use crate::types::account::Account;

use argon2::Config;
use rand::Rng;
use warp::http::StatusCode;

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

fn hash_password(pwd: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();

    argon2::hash_encoded(pwd, &salt, &config).unwrap()
}
