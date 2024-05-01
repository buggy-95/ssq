use std::collections::HashMap;

use reqwest::{header, ClientBuilder, Error as ReqError};
use serde::{Deserialize, Serialize};

use crate::{lotto::SsqResult, Args};

#[derive(Serialize, Deserialize, Debug)]
struct SsqResultResponse {
    result: Vec<SsqResult>,
}

static USER_AGENT_STR: &str = "Mozilla/1.0 (Win1.0)";

pub async fn get_result(args: &Args) -> Result<Vec<SsqResult>, ReqError> {
    let mut page_size = String::from('1');
    let mut code_index = String::new();
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

    match (&args.code, &args.recent, &args.from, args.all) {
        (Some(code), _, _, _) => {
            code_index = code.clone();
            params.insert("issueStart", code);
            params.insert("issueEnd", code);
        }
        (None, Some(recent), _, _) => {
            page_size = recent.to_string();
        }
        _ => {}
    }

    params.insert("pageSize", &page_size);

    // params.insert("issueCount", "");
    // params.insert("issueStart", "");
    // params.insert("issueEnd", "");
    // params.insert("dayStart", "");
    // params.insert("dayEnd", "");
    // params.insert("week", "");

    let response = client.get(url_api).query(&params).send().await?;
    let SsqResultResponse { mut result } = response.json().await?;
    result.reverse();

    if result.len() < 1 {
        if args.code.is_some() {
            println!("未查询到指定期号: {}", &code_index);
        }
    }

    Ok(result)
}
