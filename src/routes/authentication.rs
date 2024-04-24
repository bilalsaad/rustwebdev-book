use crate::store::Store;
use crate::types::account::Account;

use warp::http::StatusCode;

/// Registers a new account.
///
pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_account(account).await {
        None => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Some(e) => Err(warp::reject::custom(e)),
    }
}
