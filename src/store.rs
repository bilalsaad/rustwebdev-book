use handle_errors::Error;

use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

use crate::types::{
    account::{Account, AccountId},
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
                Err(Error::DatabaseQueryError(format!(
                    "failed to query questions"
                )))
            }
        }
    }

    /// Adds the new question to the store.
    /// The added question is returned.
    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content,  tags)
            VALUES ($1, $2, $3) 
            RETURNING id, title, content, tags
            ",
        )
        .bind(new_question.title.clone())
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
                Err(Error::DatabaseQueryError(format!(
                    "Failed to add question {} ",
                    new_question.title
                )))
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
                Err(Error::DatabaseQueryError(format!(
                    "failed to update question {} ",
                    question_id
                )))
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
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Some(Error::DatabaseQueryError(format!(
                    "failed to delete question {}",
                    question_id
                )))
            }
        }
    }

    // ------ ------- Answer Resource --------
    /// Adds a new answer to the store.
    /// The added answer is returned.
    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, Error> {
        match sqlx::query(
            "INSERT INTO answers (content, question_id)
            VALUES ($1, $2)
            RETURNING id, content, question_id
            ",
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
                Err(Error::DatabaseQueryError(format!(
                    "Failed to add answer for question {} ",
                    new_answer.question_id.0
                )))
            }
        }
    }

    // ------ ------- Account Resource --------
    /// Adds a new account to the store.
    ///
    /// Not idempotent if an email exists an error is returned.
    /// Password is salted
    ///
    /// Returns None on success and Some(err) if fail to add.
    pub async fn add_account(&self, account: Account) -> Option<Error> {
        match sqlx::query(
            "INSERT INTO accounts (email, password)
            VALUES ($1, $2)
            ",
        )
        .bind(account.email.clone())
        .bind(account.password)
        .execute(&self.connection)
        .await
        {
            Ok(_) => None,
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Some(Error::DatabaseQueryError(format!(
                    "Failed to add account for {} ",
                    account.email
                )))
            }
        }
    }

    /// Returns Account from DB.
    pub async fn get_account(&self, email: String) -> Result<Account, Error> {
        match sqlx::query("SELECT * FROM accounts WHERE email = $1")
            .bind(email)
            .map(|row: PgRow| Account {
                id: Some(AccountId(row.get("id"))),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(account) => Ok(account),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(format!(
                    "failed to query accounts"
                )))
            }
        }
    }
}
