use axum::{
    extract::Request,
    http::header,
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::IntoResponse,
    response::Response,
    Extension,
};

use super::auth_data::BearerData;

pub async fn authenticate(
    Extension(data): Extension<BearerData>,
    mut req: Request,
    next: Next,
) -> Result<Response, (StatusCode, impl IntoResponse)> {
    println!("middle ware called");

    let header = req.headers().clone();
    let auth_header = header
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let _auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        println!("No Authentication header found");

        let mut res = String::from("Not Ok").into_response();
        let headers = res.headers();
        res.headers_mut().insert(
            "WWW-Authenticate",
            HeaderValue::from_static("Basic realm=<realm>"),
        );
        res.headers_mut()
            .insert("allow", HeaderValue::from_static("GET,HEAD,POST"));

        return Err((StatusCode::UNAUTHORIZED, res));
    };

    let extensions = req.extensions_mut();

    let ext_data = extensions.get::<BearerData>();
    match ext_data {
        None => {
            println!("No existing auth found")
        }
        Some(_data) => {
            println!("Existing auth found")
        }
    }

    let token = &auth_header.unwrap().to_string();

    let data: BearerData = BearerData {
        token: String::from(token),
    };

    extensions.insert(data.clone());

    println!("Authentication processed {}", data.token);
    Ok(next.run(req).await)
}

fn token_is_valid(_token: &str) -> bool {
    true
}
