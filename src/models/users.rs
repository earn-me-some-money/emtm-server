#[derive(Queryable)]
pub struct User {
    pub uid: i32,
    pub wechat_id: String,
    pub phone: String,
    pub personal_info: String,
    pub school: String,
    pub verified: bool
}

