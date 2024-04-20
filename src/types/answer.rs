use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub i32);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: super::question::QuestionId,
}

/// Used to create Answer's as id is an output param.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: super::question::QuestionId,
}
