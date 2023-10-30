use leptos::wasm_bindgen::JsValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct File {
    content: String,
}
impl From<File> for JsValue {
    fn from(val: File) -> Self {
        let value = serde_json::to_string(&val).unwrap();
        JsValue::from_str(&value)
    }
}

pub async fn parse(content: &str, api: &str) -> Option<String> {
    // make the request
    let raw = reqwasm::http::Request::post(api)
        .body(File {
            content: content.to_string(),
        })
        .header("Content-Type", "application/json")
        .send()
        .await
        .ok()?
        .json::<ParseResponse>()
        .await
        .ok()?;

    let res = match raw.error {
        None => "all good".to_string(),
        Some(err) => err.message,
    };
    Some(res)
}

#[derive(Deserialize)]
pub struct ParseResponse {
    pub error: Option<ParseError>,
    pub result: Option<String>,
}

#[derive(Deserialize)]
pub struct ParseError {
    pub pos: HurlPos,
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct HurlPos {
    pub line: usize,
    pub column: usize,
}
