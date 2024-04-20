use handle_errors::Error;

use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

use crate::types::{
    answer::{Answer, AnswerId, NewAnswer},
    question::{NewQuestion, Question, QuestionId},
};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    /// Connects to the postgres DB at the given url. Excpets a questions and answers table.
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Cloudn't establish DB connection: {}", e),
        };
        Store {
            connection: db_pool,
        }
    }

    /// Returns Questions from DB.
    ///
    /// If limit is set we return |limit| questions starting from offset, otherwise return them
    /// all.
    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, Error> {
        match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    /// Adds the new question to the store.
    /// The added question is returned.
    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, Error> {
        match sqlx::query(
            "INSERT INFO questions (title, content,  tags)
            VALUE($1, $2, $3)",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    /// Updates question in store.
    ///
    /// Note that question.id is ignored and question_id is used.
    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32,
    ) -> Result<Question, Error> {
        match sqlx::query(
            "UPDATE questions SET title = $1, content = $2, tags = $3 
            WHERE id = $4
            RETURNING id, title, content, tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    pub async fn delete_question(&self, question_id: i32) -> Option<Error> {
        match sqlx::query("DELETE FROM questions WHERE id = $1")
            .bind(question_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => None,
            Err(e) => Some(Error::DatabaseQueryError(e)),
        }
    }

    // ------ ------- Answer Resource --------
    /// Adds a new answer to the store.
    /// The added answer is returned.
    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, Error> {
        match sqlx::query(
            "INSERT INFO answers (content, question_id)
            VALUE($1, $2)",
        )
        .bind(new_answer.content)
        .bind(new_answer.question_id.0)
        .map(|row: PgRow| Answer {
            id: AnswerId(row.get("id")),
            content: row.get("content"),
            question_id: QuestionId(row.get("question_id")),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(answer) => Ok(answer),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}
