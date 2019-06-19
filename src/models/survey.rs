use serde::*;

use crate::db_models;
use serde_cbor;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AnswerContent {
    SingleChoice(i32),
    MultiChoice(Vec<i32>),
    Blank(String),
}

pub struct Answer {
    pub mid: i32,
    pub user_id: i32,
    pub user_answer: Vec<AnswerContent>,
}

impl Answer {
    pub fn to_db(&self) -> Result<db_models::Answer, serde_cbor::error::Error> {
        let serialized_answer = serde_cbor::to_vec(&self.user_answer)?;
        Ok(db_models::Answer {
            mid: self.mid,
            user_id: self.user_id,
            user_answer: serialized_answer,
        })
    }
    pub fn from_db(answer: db_models::Answer) -> Result<Self, serde_cbor::error::Error> {
        let deserialized_answer = serde_cbor::from_slice(&answer.user_answer)?;
        Ok(Self {
            mid: answer.mid,
            user_id: answer.user_id,
            user_answer: deserialized_answer,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub enum QuestionContent {
    /// Question description and each choice
    SingleChoice(Vec<String>),
    MultiChoice(Vec<String>),
    /// Question description
    Blank,
}

pub struct Question {
    pub mid: i32,
    pub description: String,
    pub choices: QuestionContent,
}

impl Question {
    pub fn into_db(self, ordering: i32) -> Result<db_models::Question, serde_cbor::error::Error> {
        let serialized_choices = serde_cbor::to_vec(&self.choices)?;
        Ok(db_models::Question {
            mid: self.mid,
            ordering,
            description: self.description,
            choices: serialized_choices,
        })
    }

    pub fn from_db(answer: db_models::Question) -> Result<Self, serde_cbor::error::Error> {
        let deserialized_choices = serde_cbor::from_slice(&answer.choices)?;
        Ok(Self {
            mid: answer.mid,
            description: answer.description,
            choices: deserialized_choices,
        })
    }
}
