table! {
    emtm_cows (uid) {
        uid -> Integer,
        company -> Varchar,
    }
}

table! {
    emtm_students (uid) {
        uid -> Integer,
        school_id -> Integer,
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
joinable!(emtm_students -> emtm_users (uid));

allow_tables_to_appear_in_same_query!(emtm_cows, emtm_students, emtm_users, school_zh,);
