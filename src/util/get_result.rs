use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH, Instant};

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

fn get_cache_file_path() -> PathBuf {
    let mut cache_file_path = env::current_exe().unwrap();
    cache_file_path.pop();
    cache_file_path.push(CACHE_FILE_NAME);
    cache_file_path
}

async fn fetch_result(verbose: bool) -> Result<Vec<SsqResult>, ReqError> {
    let time_1: Instant = Instant::now();
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
    if verbose { println!("[time] (fetch_result)\t\tprepare fetch client:\t{:?}", time_1.elapsed()) }

    let api_params = [
        ("systemType", "PC"),
        ("name", "ssq"),
        ("pageNo", "1"),
        ("pageSize", "65536"),
        ("dayStart", "2000-01-01"),
    ];

    let time_2 = Instant::now();
    match client.get(url_home).send().await {
        Err(err) => panic!("Token 获取失败: {err}"),
        _ => {}
    }
    if verbose { println!("[time] (fetch_result)\t\tget token:\t{:?}", time_2.elapsed()) }

    let time_3 = Instant::now();
    let result = match client.get(url_api).query(&api_params).send().await {
        Err(err) => panic!("历史数据查询失败: {err}"),
        Ok(response) => {
            let SsqResultResponse { result } = response.json().await?;
            Ok(result)
        }
    };
    if verbose { println!("[time] (fetch_result)\t\tfetch history data:\t{:?}", time_3.elapsed()) }

    result
}

async fn update_cache(verbose: bool) -> Vec<SsqResult> {
    let time_1 = Instant::now();
    let new_cache_data = fetch_result(verbose).await.unwrap();
    if verbose { println!("[time] (update_cache)\t\tfetch full result:\t{:?}", time_1.elapsed()) }

    let time_2 = Instant::now();
    let json = serde_json::to_string(&new_cache_data).unwrap();
    if verbose { println!("[time] (update_cache)\t\tserialize json:\t{:?}", time_2.elapsed()) }

    let time_3 = Instant::now();
    match std::fs::write(get_cache_file_path(), json) {
        Err(err) => println!("缓存写入失败: {err}"),
        _ => {}
    }
    if verbose { println!("[time] (update_cache)\t\twrite json:\t{:?}", time_3.elapsed()) }

    new_cache_data
}

fn get_cache(verbose: bool) -> Result<Vec<SsqResult>, Box<dyn Error>> {
    let time_1 = Instant::now();
    let file = fs::File::open(get_cache_file_path())?;
    let result: Vec<SsqResult> = serde_json::from_reader(file)?;
    if verbose { println!("[time] (get_cache)\tread cache:\t\t{:?}", time_1.elapsed()) }
    let last_cache_data = &result[0];

    let time_2 = Instant::now();
    let result = match util::is_outdated(&last_cache_data.date) {
        true => Err("缓存过期")?,
        false => Ok(result),
    };
    if verbose { println!("[time] (get_cache)\tjudge cache is fresh:\t{:?}", time_2.elapsed()) }

    result
}

pub async fn get_result(args: &Args) -> Result<Vec<SsqResult>, ReqError> {
    let time_1 = Instant::now();
    let mut result = match get_cache(args.verbose) {
        Ok(result) => result,
        Err(_) => update_cache(args.verbose).await,
    };
    if args.verbose { println!("[time] (get_result)\tget full result:\t{:?}", time_1.elapsed()) }

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

        let time_2 = Instant::now();
        if code_index_reg.is_match(&from) {
            if args.verbose { println!("[time] (get_result)\t\tregex code:\t{:?}", time_2.elapsed()) }
            let filter = |ssq: &SsqResult| &ssq.code >= from;
            result = result.into_iter().filter(filter).collect();
        } else if date_reg.is_match(&from) {
            if args.verbose { println!("[time] (get_result)\t\tregex date:\t{:?}", time_2.elapsed()) }
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
