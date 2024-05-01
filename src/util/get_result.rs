use std::collections::HashMap;

use reqwest::{ClientBuilder, header, Error as ReqError };
use serde::{Serialize, Deserialize};

use crate::lotto::SsqResult;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct SSqResultResponse {
    result: Vec<SsqResult>,
}

static USER_AGENT_STR: &str = "Mozilla/1.0 (Win1.0)";

pub async fn get_result() -> Result<Vec<SsqResult>, ReqError> {
    let url_home = "https://www.cwl.gov.cn/ygkj/wqkjgg/ssq/";
    let url_api = "https://www.cwl.gov.cn/cwl_admin/front/cwlkj/search/kjxx/findDrawNotice";

    let mut default_headers = header::HeaderMap::new();
    default_headers.append(
        header::USER_AGENT,
        header::HeaderValue::from_str(USER_AGENT_STR).unwrap(),
    );

    let client = ClientBuilder::new()
        .cookie_store(true)
        .default_headers(default_headers)
        .build()?;

    client.get(url_home).send().await?;

    let mut params = HashMap::new();
    params.insert("systemType", "PC");
    params.insert("name", "ssq");
    params.insert("pageNo", "1");
    params.insert("pageSize", "1");

    // params.insert("issueCount", "");
    // params.insert("issueStart", "");
    // params.insert("issueEnd", "");
    // params.insert("dayStart", "");
    // params.insert("dayEnd", "");
    // params.insert("week", "");

    let response = client.get(url_api).query(&params).send().await?;
    let SSqResultResponse { mut result } = response.json().await?;
    result.reverse();

    Ok(result)
}
