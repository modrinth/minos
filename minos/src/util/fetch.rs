use std::time;

use lazy_static::lazy_static;

lazy_static! {
    static ref REQWEST_CLIENT: reqwest::Client = {
        let mut headers = reqwest::header::HeaderMap::new();
        let header = reqwest::header::HeaderValue::from_str(&format!(
            "modrinth/minos/{} (support@modrinth.com)",
            env!("CARGO_PKG_VERSION")
        ))
        .unwrap();
        headers.insert(reqwest::header::USER_AGENT, header);
        reqwest::Client::builder()
            .tcp_keepalive(Some(time::Duration::from_secs(10)))
            .default_headers(headers)
            .build()
            .expect("Reqwest Client Building Failed")
    };
}
