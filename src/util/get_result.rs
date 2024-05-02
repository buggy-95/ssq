use std::error::Error;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use regex::Regex;
use reqwest::{header, ClientBuilder, Error as ReqError};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::{lotto::SsqResult, util, Args};

#[derive(Serialize, Deserialize, Debug)]
struct SsqResultResponse {
    result: Vec<SsqResult>,
}

static CACHE_FILE_NAME: &str = "cache_ssq.json";

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
        ("pageSize", "65536"),
        ("dayStart", "2000-01-01"),
    ];

    match client.get(url_home).send().await {
        Err(err) => panic!("Token 获取失败: {err}"),
        _ => {}
    }

    match client.get(url_api).query(&api_params).send().await {
        Err(err) => panic!("历史数据查询失败: {err}"),
        Ok(response) => {
            let SsqResultResponse { result } = response.json().await?;
            Ok(result)
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
    new_cache_data
}

async fn get_cache() -> Result<Vec<SsqResult>, Box<dyn Error>> {
    let file = fs::File::open(CACHE_FILE_NAME)?;
    let result: Vec<SsqResult> = serde_json::from_reader(file)?;
    let last_cache_data = &result[&result.len() - 1];
    match util::is_outdated(&last_cache_data.date) {
        true => return Err("缓存过期")?,
        false => Ok(result),
    }
}

pub async fn get_result(args: &Args) -> Result<Vec<SsqResult>, ReqError> {
    let mut result = match get_cache().await {
        Ok(result) => result,
        Err(_) => update_cache().await,
    };

    if args.code.is_some() {
        let code = args.code.as_ref().unwrap();
        result = result.into_iter().filter(|ssq| &ssq.code == code).collect();
        if result.len() < 1 {
            println!("未查询到指定期号: {}", &code);
            return Ok(vec![]);
        }
    } else if args.recent.is_some() {
        let recent = args.recent.unwrap() as usize;
        result = result.into_iter().take(recent).collect();
    } else if args.from.is_some() {
        let from = args.from.as_ref().unwrap();
        let code_index_reg = Regex::new(r"^20\d{5}$").unwrap();
        let date_reg = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

        if code_index_reg.is_match(&from) {
            let filter = |ssq: &SsqResult| &ssq.code >= from;
            result = result.into_iter().filter(filter).collect();
        } else if date_reg.is_match(&from) {
            let filter = |ssq: &SsqResult| &ssq.date >= from;
            result = result.into_iter().filter(filter).collect();
        } else {
            panic!("参数错误: --from {from}");
        }
    } else if !args.all {
        result = result.into_iter().take(1).collect();
    }

    result.reverse();

    Ok(result)
}
