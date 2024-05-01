use std::collections::HashMap;

use reqwest::{ClientBuilder, header, Error as ReqError };
use serde::{Serialize, Deserialize};
use serde_json::to_string_pretty;

use crate::lotto::SsqResult;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct SSqResultResponse {
    result: Vec<SsqResult>,
}

static USER_AGENT_STR: &str = "Mozilla/1.0 (Win1.0)";

pub async fn get_result() -> Result<Vec<SsqResult>, ReqError> {
    let mut default_headers = header::HeaderMap::new();
    default_headers.append(
        header::USER_AGENT,
        header::HeaderValue::from_str(USER_AGENT_STR).unwrap(),
    );

    let client = ClientBuilder::new()
        .cookie_store(true)
        .default_headers(default_headers)
        .build()?;

    client
        .get("https://www.cwl.gov.cn/ygkj/wqkjgg/ssq/")
        .send()
        .await?;

    let mut params = HashMap::new();
    params.insert("name", "ssq");
    params.insert("systemType", "PC");
    params.insert("pageNo", "1");
    params.insert("pageSize", "1");

    // params.insert("issueCount", "");
    // params.insert("issueStart", "");
    // params.insert("issueEnd", "");
    // params.insert("dayStart", "");
    // params.insert("dayEnd", "");
    // params.insert("week", "");

    let response = client
        .get("https://www.cwl.gov.cn/cwl_admin/front/cwlkj/search/kjxx/findDrawNotice")
        .query(&params)
        .send()
        .await?;

    let SSqResultResponse { result } = response.json().await?;
    let json = to_string_pretty(&result).unwrap();

    println!("json: {json}");

    Ok(result)
}
