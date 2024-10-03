use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;

use tokio::time::{sleep, Duration};

use crate::bug_report_data;
//use crate::transmit_report;
//use crate::transmit_via_smtp;
use crate::transmit_via_async_smtp;

pub async fn do_monitoring(pool: &PgPool) -> Result<(), String> {
    println!("Starting Bug Report monitoring!");

    loop {
        // let re = check_for_report(&pool).await;
        // if let Err(res) = re {
        //     if res.0 == 404 {
        //         println!("No new reports found");
        //     };
        // };
        let _re = check_for_reports(&pool).await;
        sleep(Duration::from_millis(30000)).await;
    }

    Ok(())
}

pub async fn _check_for_report(
    pool: &PgPool,
) -> Result<Json<bug_report_data::BugReportRequest>, (StatusCode, String)> {
    println!("Checking for bug report");

    let result: bug_report_data::BugReportRequest =
        sqlx::query_as("select * from bug_report limit 1")
            .fetch_one(pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error is {}", err),
                ),
            })?;
    //        .unwrap();

    println!(
        "Found bug report {} {} - Transmitting",
        result.id, result.mail_address
    );
    //transmit_report::transmit(pool, &result).await;
    //transmit_via_smtp::transmit(&result).await;
    transmit_via_async_smtp::transmit(&result).await;

    println!(
        "Found bug report {} {} - Deleting",
        result.id, result.mail_address
    );
    delete(pool, &result).await;

    Ok(Json(result))
}

pub async fn check_for_reports(pool: &PgPool) -> Result<(), String> {
    println!("Checking for bug reports");

    let bug_reports: Vec<bug_report_data::BugReportRequest> =
        sqlx::query_as("select * from bug_report")
            .fetch_all(pool)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, format!("Error is {}", err)),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error is {}", err),
                ),
            })
            //        ?;
            .unwrap();

    for bug_report in bug_reports {
        println!(
            "Found bug report {} {} - Transmitting",
            bug_report.id, bug_report.mail_address
        );
        transmit_via_async_smtp::transmit(&bug_report).await;

        println!(
            "Found bug report {} {} - Deleting",
            bug_report.id, bug_report.mail_address
        );
        delete(pool, &bug_report).await;
    }

    println!("Finished Checking for bug reports");

    Ok(())
}

pub async fn delete(pool: &PgPool, report: &bug_report_data::BugReportRequest) {
    let _response = sqlx::query(r#"DELETE FROM bug_report where id = $1"#)
        .bind(report.id)
        .execute(pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error is {}", err),
            )
        });
}
