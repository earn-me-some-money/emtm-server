table! {
    emtm_answer (qa_id) {
        qn_id -> Integer,
        qa_id -> Integer,
        user_id -> Integer,
        user_answer -> Varchar,
    }
}

table! {
    emtm_cows (uid) {
        uid -> Integer,
        company -> Varchar,
    }
}

table! {
    emtm_errands (e_id) {
        mid -> Integer,
        e_id -> Integer,
        e_address -> Varchar,
        e_phone_number -> Varchar,
        e_pick_number -> Varchar,
        e_info -> Varchar,
    }
}

table! {
    emtm_missions (mid) {
        mid -> Integer,
        poster_uid -> Integer,
        bounty -> Integer,
        risk -> Integer,
        name -> Varchar,
        mission_type -> Tinyint,
        content -> Text,
        post_time -> Datetime,
        deadline -> Datetime,
        max_participants -> Integer,
        min_grade -> Nullable<Integer>,
        max_grade -> Nullable<Integer>,
        school -> Nullable<Integer>,
        min_finished -> Nullable<Integer>,
    }
}

table! {
    emtm_participants (mid, student_uid) {
        mid -> Integer,
        student_uid -> Integer,
        state -> Tinyint,
    }
}

table! {
    emtm_question (q_id) {
        qn_id -> Integer,
        q_id -> Integer,
        q_type -> Bool,
        q_content -> Varchar,
        q_choice -> Varchar,
    }
}

table! {
    emtm_students (uid) {
        uid -> Integer,
        school_id -> Integer,
        student_id -> Varchar,
        credit -> Integer,
        accepted -> Integer,
        finished -> Integer,
        major -> Varchar,
        year -> Integer,
    }
}

table! {
    emtm_survey (qn_id) {
        mid -> Integer,
        qn_id -> Integer,
    }
}

table! {
    emtm_transaction (t_id) {
        mid -> Integer,
        t_id -> Integer,
        t_type -> Varchar,
        t_info -> Varchar,
        t_loss -> Integer,
    }
}

table! {
    emtm_users (uid) {
        uid -> Integer,
        wechat_id -> Varchar,
        phone -> Varchar,
        personal_info -> Text,
        email -> Varchar,
        username -> Varchar,
        verified -> Bool,
        tokens -> Integer,
        user_type -> Tinyint,
    }
}

table! {
    school_zh (school_id) {
        school_id -> Integer,
        school_name -> Varchar,
    }
}

joinable!(emtm_answer -> emtm_students (user_id));
joinable!(emtm_answer -> emtm_survey (qn_id));
joinable!(emtm_cows -> emtm_users (uid));
joinable!(emtm_errands -> emtm_missions (mid));
joinable!(emtm_missions -> emtm_users (poster_uid));
joinable!(emtm_missions -> school_zh (school));
joinable!(emtm_participants -> emtm_missions (mid));
joinable!(emtm_participants -> emtm_students (student_uid));
joinable!(emtm_question -> emtm_survey (qn_id));
joinable!(emtm_students -> emtm_users (uid));
joinable!(emtm_students -> school_zh (school_id));
joinable!(emtm_survey -> emtm_missions (mid));
joinable!(emtm_transaction -> emtm_missions (mid));

allow_tables_to_appear_in_same_query!(
    emtm_answer,
    emtm_cows,
    emtm_errands,
    emtm_missions,
    emtm_participants,
    emtm_question,
    emtm_students,
    emtm_survey,
    emtm_transaction,
    emtm_users,
    school_zh,
);
