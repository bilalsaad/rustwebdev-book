use crate::store::Store;
use crate::types::answer::{Answer, AnswerId};
use crate::types::question::QuestionId;
use handle_errors::Error;
use std::collections::HashMap;
use warp::http::StatusCode;

/// Handler for creating answer.
pub async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let answer = Answer {
        // TODO: make this a random uuid or something.
        id: AnswerId(1),
        content: params.get("content").unwrap().to_string(),
        question_id: QuestionId(
            params
                .get("questionId")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::ParseError)?,
        ),
    };
    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);

    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}
