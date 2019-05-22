table! {
    emtm_cows (uid) {
        uid -> Integer,
        company -> Varchar,
    }
}

table! {
    emtm_missions (mid) {
        mid -> Integer,
        cow_uid -> Integer,
        bounty -> Integer,
        risk -> Integer,
        name -> Varchar,
        content -> Text,
        post_time -> Datetime,
        deadline -> Datetime,
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

joinable!(emtm_cows -> emtm_users (uid));
joinable!(emtm_missions -> emtm_cows (cow_uid));
joinable!(emtm_participants -> emtm_missions (mid));
joinable!(emtm_participants -> emtm_students (student_uid));
joinable!(emtm_students -> emtm_users (uid));

allow_tables_to_appear_in_same_query!(
    emtm_cows,
    emtm_missions,
    emtm_participants,
    emtm_students,
    emtm_users,
    school_zh,
);
