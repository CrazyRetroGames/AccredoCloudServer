use crate::bug_report_data;
use mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;

pub async fn transmit(report: &bug_report_data::BugReportRequest) -> Result<(), String> {
    // Build a simple multipart message
    let message = MessageBuilder::new()
        .from(("BugReport Sender", "stevenewby459@gmail.com"))
        .to(vec![("Bug Report Recipient", "steve@accredo.co.nz")])
        .subject(report.subject.clone())
        .text_body(report.mail_body.clone())
        .attachment("text/plain", "details_from_rust.txt", report.detail.clone())
        .attachment("image/jpg", "screenshot.jpg", report.screenshot.clone());

    println!("Sending Mail");
    // Connect to the SMTP submissions port, upgrade to TLS and
    // authenticate using the provided credentials.
    SmtpClientBuilder::new("smtp.gmail.com", 587) // 465)
        .implicit_tls(false)
        .credentials(("stevenewby459@gmail.com", "qnyl weob imud wmnj "))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();

    println!("Mail Sent");
    Ok(())
}
