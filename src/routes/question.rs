use crate::profanity::check_profanity;
use crate::store::Store;
use crate::types::pagination::Pagination;
use crate::types::question::NewQuestion;
use crate::types::{pagination::extract_pagination, question::Question};
use std::collections::HashMap;
use tracing::{event, info, instrument, Level};
use warp::http::StatusCode;

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "book", Level::INFO, "querying questions");

    let mut pagination = Pagination::default();

    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;
    } else {
        info!(pagination = false);
    }

    let res: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let title = match check_profanity(new_question.title).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let content = match check_profanity(new_question.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let new_question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };

    if let Err(e) = store.add_question(new_question).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

/// Update handler for Question resource.
pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let title = check_profanity(question.title);
    let content = check_profanity(question.content);

    let (title, content) = tokio::join!(title, content);

    if title.is_err() {
        return Err(warp::reject::custom(title.unwrap_err()));
    }

    if content.is_err() {
        return Err(warp::reject::custom(content.unwrap_err()));
    }

    let question = Question {
        id: question.id,
        title: title.unwrap(),
        content: content.unwrap(),
        tags: question.tags,
    };

    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

/// Delete handler for Question
pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_question(id).await {
        None => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Some(e) => Err(warp::reject::custom(e)),
    }
}
