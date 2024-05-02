use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;
use reqwest::{header, ClientBuilder, Error as ReqError};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{lotto::SsqResult, Args, util};

#[derive(Serialize, Deserialize, Debug)]
struct SsqResultResponse {
    result: Vec<SsqResult>,
}

static CACHE_FILE_NAME: &str = "cache_ssq_1.json";

async fn fetch_result() -> Result<Vec<SsqResult>, ReqError> {
    let url_home = "https://www.cwl.gov.cn/ygkj/wqkjgg/ssq/";
    let url_api = "https://www.cwl.gov.cn/cwl_admin/front/cwlkj/search/kjxx/findDrawNotice";
    let now = SystemTime::now();
    let timestamp = now
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
        .to_string();
    let user_agent_str = format!("Mozilla/1.0 (Win1.1) {timestamp}");

    let mut default_headers = header::HeaderMap::new();
    default_headers.append(
        header::USER_AGENT,
        header::HeaderValue::from_str(&user_agent_str).unwrap(),
    );
    let client = ClientBuilder::new()
        .cookie_store(true)
        .default_headers(default_headers)
        .build()?;

    let api_params = [
        ("systemType", "PC"),
        ("name", "ssq"),
        ("pageNo", "1"),
        ("pageSize", "3"),
        // ("pageSize", "65536"),
        ("dayStart", "2000-01-01"),
    ];

    match client.get(url_home).send().await {
        Err(err) => panic!("Token 获取失败: {err}"),
        _ => {}
    }

    match client.get(url_api).query(&api_params).send().await {
        Err(err) => panic!("历史数据查询失败: {err}"),
        Ok(response) => {
            let SsqResultResponse { mut result } = response.json().await?;
            result.reverse();
            return Ok(result);
        }
    }
}

async fn update_cache() -> Vec<SsqResult> {
    let new_cache_data = fetch_result().await.unwrap();
    let json = serde_json::to_string(&new_cache_data).unwrap();
    match std::fs::write(CACHE_FILE_NAME, json) {
        Err(err) => println!("缓存写入失败: {err}"),
        _ => {}
    }
    return new_cache_data;
}

async fn get_cache() -> Result<Vec<SsqResult>, Box<dyn Error>> {
    let file = fs::File::open(CACHE_FILE_NAME)?;
    let result: Vec<SsqResult> = serde_json::from_reader(file)?;
    let last_cache_data = &result[&result.len() - 1];
    if util::is_outdated(&last_cache_data.date) { return Err("缓存过期")? }
    return Ok(result);
}

pub async fn get_result(args: &Args) -> Result<Vec<SsqResult>, ReqError> {
    // println!("test start");
    let result = match get_cache().await {
        Ok(result) => println!("read success: {}", result.len()),
        Err(err) => {
            println!("read failed: {err}");
            return Ok(update_cache().await);
        },
    };
    // println!("test end");
    return Ok(vec![]);

    // let url_home = "https://www.cwl.gov.cn/ygkj/wqkjgg/ssq/";
    // let url_api = "https://www.cwl.gov.cn/cwl_admin/front/cwlkj/search/kjxx/findDrawNotice";
    // let mut page_size = String::from('1');
    // let mut code_index = String::new();
    // let now = SystemTime::now();
    // let timestamp = now
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .as_micros()
    //     .to_string();
    // let user_agent_str = format!("Mozilla/1.0 (Win1.1) {timestamp}");

    // let mut default_headers = header::HeaderMap::new();
    // default_headers.append(
    //     header::USER_AGENT,
    //     header::HeaderValue::from_str(&user_agent_str).unwrap(),
    // );

    // let client = ClientBuilder::new()
    //     .cookie_store(true)
    //     .default_headers(default_headers)
    //     .build()?;

    // client.get(url_home).send().await?;

    // let mut params = HashMap::new();
    // params.insert("systemType", "PC");
    // params.insert("name", "ssq");
    // params.insert("pageNo", "1");

    // match (&args.code, &args.recent, &args.from, &args.all) {
    //     (Some(code), _, _, _) => {
    //         code_index = code.clone();
    //         params.insert("issueStart", code);
    //         params.insert("issueEnd", code);
    //     }
    //     (None, Some(recent), _, _) => {
    //         page_size = recent.to_string();
    //     }
    //     (None, None, Some(from), _) => {
    //         let code_index_reg = Regex::new(r"^20\d{5}$").unwrap();
    //         let date_reg = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    //         if code_index_reg.is_match(from) {
    //             params.insert("issueStart", from);
    //         } else if date_reg.is_match(from) {
    //             params.insert("dayStart", from);
    //         }
    //         page_size = "9999".to_string();
    //     }
    //     (None, None, None, true) => {
    //         params.insert("dayStart", "2000-01-01");
    //         page_size = "9999".to_string();
    //     }
    //     _ => {}
    // }

    // params.insert("pageSize", &page_size);

    // let response = client.get(url_api).query(&params).send().await?;
    // let SsqResultResponse { mut result } = response.json().await?;
    // result.reverse();

    // let json = serde_json::to_string(&result).unwrap();
    // let cache_file_name = "cache_ssq.json";
    // match std::fs::write(cache_file_name, json) {
    //     Ok(_) => println!("缓存写入成功"),
    //     Err(err) => println!("缓存写入失败: {err}"),
    // }

    // if result.len() < 1 {
    //     if args.code.is_some() {
    //         println!("未查询到指定期号: {}", &code_index);
    //     }
    // }

    // Ok(result)
}
