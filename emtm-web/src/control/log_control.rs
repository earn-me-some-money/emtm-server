/*
* Emtm-Controller Modules -- Log Control
*/
extern crate emtm_db;
extern crate json;

use actix_web::{web, HttpResponse};

use crate::control::json_objs;
use crate::control::main_control;
use emtm_db::controller::{
    school_controller_zh::SchoolControllerZh, user_controller::UserController, Controller,
};
// Model Schemas
use emtm_db::models::users::{Cow, Student, User, UserId};

pub fn logup_cow(
    username: &str,
    userid: &str,
    phone: &str,
    email: &str,
    infos: &str,
) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };
    // Init DB Control
    let db_control = Controller::new();
    // Define Duplication error message
    let dup_errors = ["UserID", "Email", "Phone", "Logup Error!", "Duplication!"];
    // Variable order: userid, email, organization
    let mut dup_array = [false, false, false];
    let mut logup_enable = true;

    // Check email format
    if !main_control::email_format(email) {
        result_obj.code = false;
        result_obj.err_message = "Cannot Pass Email Format Checking!".to_string();
        return HttpResponse::Ok().json(result_obj);
    } else if !main_control::phone_format(phone) {
        // Checking phone format
        result_obj.code = false;
        result_obj.err_message = "Cannot Pass Phone Format Checking!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Check registered infos duplication
    let check_user_id: UserId = UserId::WechatId(userid);
    match db_control.get_user_from_identifier(check_user_id) {
        Some(_x) => dup_array[0] = true,
        None => (),
    }

    let check_user_email: UserId = UserId::Email(email);
    match db_control.get_user_from_identifier(check_user_email) {
        Some(_x) => dup_array[1] = true,
        None => (),
    }

    let check_user_phone: UserId = UserId::Phone(phone);
    match db_control.get_user_from_identifier(check_user_phone) {
        Some(_x) => dup_array[2] = true,
        None => (),
    }

    for index in 0..3 {
        if dup_array[index] {
            logup_enable = false;
            result_obj.err_message = [dup_errors[3], dup_errors[index], dup_errors[4]]
                .join(" ")
                .to_string();
            break;
        }
    }

    // Pass checking, do db-storing
    if logup_enable {
        // New Adding Cow Vector
        let cows = vec![Cow {
            uid: 0,
            wechat_id: userid.to_string(),
            phone: phone.to_string(),
            personal_info: infos.to_string(),
            email: email.to_string(),
            username: username.to_string(),
            verified: false,
            tokens: 0,
            company: "".to_string(),
        }];

        if let Err(err) = db_control.add_cows(&cows).remove(0) {
            result_obj.code = false;
            result_obj.err_message = format!("{}", err);
        }
    } else {
        result_obj.code = false;
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn logup_student(data: web::Json<json_objs::StuLogupObj>) -> HttpResponse {
    let mut result_obj = json_objs::OriginObj {
        code: true,
        err_message: "".to_string(),
    };
    // Init DB Control
    let db_control = Controller::new();
    // Define Duplication error message
    let dup_errors = ["UserID", "Email", "Phone", "Logup Error!", "Duplication!"];
    // Variable order: userid, email, organization
    let mut dup_array = [false, false, false, false];
    let mut logup_enable = true;

    // Check email format
    if !main_control::email_format(&data.email) {
        result_obj.code = false;
        result_obj.err_message = "Cannot Pass Email Format Checking!".to_string();
        return HttpResponse::Ok().json(result_obj);
    } else if !main_control::phone_format(&data.phone) {
        // Checking phone format
        result_obj.code = false;
        result_obj.err_message = "Cannot Pass Phone Format Checking!".to_string();
        return HttpResponse::Ok().json(result_obj);
    }

    // Make sure not-double logup
    // Check registered infos duplication
    let check_user_id: UserId = UserId::WechatId(&data.userid);
    match db_control.get_user_from_identifier(check_user_id) {
        Some(_x) => dup_array[0] = true,
        None => (),
    }

    let check_user_email: UserId = UserId::Email(&data.email);
    match db_control.get_user_from_identifier(check_user_email) {
        Some(_x) => dup_array[1] = true,
        None => (),
    }

    let check_user_phone: UserId = UserId::Phone(&data.phone);
    match db_control.get_user_from_identifier(check_user_phone) {
        Some(_x) => dup_array[2] = true,
        None => (),
    }

    for index in 0..3 {
        if dup_array[index] {
            logup_enable = false;
            result_obj.err_message = [dup_errors[3], dup_errors[index], dup_errors[4]]
                .join(" ")
                .to_string();
            break;
        }
    }

    // Pass checking, do db-stroing
    if logup_enable {
        // New Adding Cow Vector
        let school_id = match db_control.get_school_id("中山大学") {
            Some(_x) => _x,
            None => 0,
        };

        let rand_student_id: String = (0..15)
            .map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char)
            .collect();

        let students = vec![Student {
            uid: 0,
            wechat_id: data.userid.clone(),
            phone: data.phone.clone(),
            personal_info: data.infos.clone(),
            email: data.email.clone(),
            username: data.username.clone(),
            verified: false,
            tokens: 0,
            school_id: school_id,
            student_id: rand_student_id,
            credit: 100, // Init Credit Score is 100%
            accepted: 0,
            finished: 0,
            major: data.major.clone(),
            year: data.year.clone(),
        }];

        if let Err(err) = db_control.add_students(&students).remove(0) {
            result_obj.code = false;
            result_obj.err_message = format!("{}", err);
        }
    } else {
        result_obj.code = false;
    }

    HttpResponse::Ok().json(result_obj)
}

pub fn login(userid: &str) -> HttpResponse {
    let mut result_obj = json_objs::LoginResultObj {
        code: true,
        user_type: 2,
        err_message: "".to_string(),
    };

    // Init DB Control
    let db_control = Controller::new();

    // Check user registered or not
    let login_user_id: UserId = UserId::WechatId(userid);
    let login_enable = match db_control.get_user_from_identifier(login_user_id) {
        Some(_x) => {
            match _x {
                User::Cow(_cow) => result_obj.user_type = 0,
                User::Student(_stu) => result_obj.user_type = 1,
            };
            true
        }
        None => false,
    };

    // Pass checking
    if !login_enable {
        result_obj.code = false;
        result_obj.err_message = "Login Fail! Taret User Not Registered!".to_string();
    }

    HttpResponse::Ok().json(result_obj)
}
