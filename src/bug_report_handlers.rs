use axum::extract::Multipart;
use axum::{extract::State, http::StatusCode, Extension};
use sqlx::postgres::PgPool;

use crate::auth_data::BearerData;

pub async fn receive_bug_report(
    Extension(data): Extension<BearerData>,
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> Result<String, (StatusCode, String)> {
    println!("{:?}", data);

    let mut mail_from = String::from("");
    let mut mail_subject = String::from("");
    let mut mail_body = String::from("");
    let mut mail_detail = String::from("");
    let mut mail_screenshot: Vec<u8> = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        // let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        println!("{name}:{content_type}");

        if name == "ScreenShot" {
            mail_screenshot = field.bytes().await.unwrap().to_vec();
        } else {
            let text = field.text().await.unwrap();
            if name == "MailFrom" {
                mail_from = String::from(text.clone());
            };
            if name == "MailSubject" {
                mail_subject = String::from(text.clone());
            };
            if name == "MailBody" {
                mail_body = String::from(text.clone());
            };
            if name == "details" {
                mail_detail = String::from(text.clone());
            }
        }
    }

    println!("bug report recieved from: {mail_from}::{mail_subject}");
    let _response = sqlx::query(r#"INSERT INTO bug_report (mail_address, subject, mail_body, detail, screenshot) values ($1, $2, $3, $4, $5)"#)
        .bind(&mail_from)
        .bind(&mail_subject)
        .bind(&mail_body)
        .bind(&mail_detail)
        .bind(&mail_screenshot)
        .execute(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            )
        }).unwrap();

    println!("bug report saved: {mail_from}::{mail_subject}");

    Ok((String::from("Ok")))
}
