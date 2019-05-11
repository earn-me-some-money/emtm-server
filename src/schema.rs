table! {
    emtm_cows (uid) {
        uid -> Integer,
        company -> Varchar,
    }
}

table! {
    emtm_students (uid) {
        uid -> Integer,
        school -> Varchar,
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
        username -> Varchar,
        verified -> Bool,
        tokens -> Integer,
    }
}

joinable!(emtm_cows -> emtm_users (uid));
joinable!(emtm_students -> emtm_users (uid));

allow_tables_to_appear_in_same_query!(
    emtm_cows,
    emtm_students,
    emtm_users,
);
