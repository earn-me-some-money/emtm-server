use diesel::prelude::*;

use crate::controller::Controller;
use crate::db_error::DbError;
use crate::db_models;
use crate::models;

pub trait SurveyController {
    /// Adds questions to a questionnaire, returns the Question instances
    /// # Arguments
    /// * 'questions' - A vector containing all the question instances to be added
    /// * 'mid' - The mid of the questionnaire to be added questions, all questions should be have the same mid
    fn add_questions(&self, questions: Vec<models::Question>, mid: i32) -> Result<Vec<models::Question>, DbError>;
    /// Query all questions from a questionnaire
    /// # Arguments
    /// 'mid' - The mid of the questionnaire to be retrieved
    fn get_questionnaire(&self, mid: i32) -> Result<Vec<models::Question>, DbError>;
    /// Adds answer to a questionnaire, returns the Answer instances
    /// # Arguments
    /// * 'answer' - An answer to be added
    fn add_answer(&self, answer: &models::Answer) -> Result<(), DbError>;
    /// Get all answers to a questionnaire
    /// # Arguments
    /// 'mid' - The mid of the questionnaire to be retrieved
    fn get_question_answers(&self, mid: i32) -> Result<Vec<models::Answer>, DbError>;
}

impl SurveyController for Controller {
    fn add_questions(&self, questions: Vec<models::Question>, mid: i32) -> Result<Vec<models::Question>, DbError> {
        let mut db_questions: Vec<db_models::Question> = Vec::with_capacity(questions.len());
        for (index, q) in questions.into_iter().enumerate() {
            if *&q.mid != mid {
                return Err(DbError::new(&format!("Wrong mid in question with index {}", index)));
            }
            db_questions.push(match q.into_db(index as i32) {
                Ok(db_q) => db_q,
                Err(parse_err) => {
                    return Err(DbError::new(&format!("Failed to parse object for question with index {}: {}", index, parse_err)));
                }
            })
        }

        use crate::schema::emtm_questions;
        let result = diesel::insert_into(emtm_questions::table)
            .values(&db_questions)
            .execute(&self.connection);

        match result {
            Ok(_) => {
                self.get_questionnaire(mid)
            },
            Err(err) => {
                warn!(
                    "Failed to insert questions: {}",
                    err
                );
                Err(DbError::new(&err.to_string()))
            },
        }

    }

    fn get_questionnaire(&self, mid_: i32) -> Result<Vec<models::Question>, DbError> {
        use crate::schema::emtm_questions::dsl::*;
        let result = emtm_questions
            .filter(mid.eq(mid_))
            .order_by(ordering.asc())
            .load::<db_models::Question>(&self.connection);

        match result {
            Ok(questions) => {
                let questions_result_list: Vec<_> = questions
                    .into_iter()
                    .map(|q| {
                        models::Question::from_db(q)
                    }).collect();

                for res in &questions_result_list {
                    if let Err(error) = res {
                        return Err(DbError::new(&format!("failed to deserialize question: {}", error)));
                    }
                }

                Ok(questions_result_list.into_iter().map(Result::unwrap).collect())
            }
            Err(err) => {
                error!(
                    "Panic when finding questionnaire with mid {}: {}",
                    mid_, err
                );
                panic!(err.to_string());
            }
        }
    }

    fn add_answer(&self, answer: &models::Answer) -> Result<(), DbError> {
        let db_answer = match answer.to_db() {
            Ok(ans) => ans,
            Err(e) => { return Err(DbError::new(&format!("fail to serialize answer: {}", e))); },
        };


        use crate::schema::emtm_answers;
        let result = diesel::insert_into(emtm_answers::table)
            .values(&db_answer)
            .execute(&self.connection);

        match result {
            Ok(_) => {
                Ok(())
            },
            Err(err) => {
                warn!(
                    "Failed to insert answer: {}",
                    err
                );
                Err(DbError::new(&err.to_string()))
            },
        }
    }

    fn get_question_answers(&self, mid_: i32) -> Result<Vec<models::Answer>, DbError> {
        use crate::schema::emtm_answers::dsl::*;
        let result = emtm_answers
            .filter(mid.eq(mid_))
            .load::<db_models::Answer>(&self.connection);

        match result {
            Ok(answers) => {
                let answers_list: Vec<_> = answers
                    .into_iter()
                    .filter_map(|ans| {
                        models::Answer::from_db(ans).ok()
                    }).collect();

                Ok(answers_list)
            }
            Err(err) => {
                error!(
                    "Panic when finding answer for questionnaire with mid {}: {}",
                    mid_, err
                );
                panic!(err.to_string());
            }
        }
    }
}



