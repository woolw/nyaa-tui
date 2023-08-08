use self::datamodel::{Body, QueryParameters};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use std::{fs::File, io::Write};
use unhtml::FromHtml;

pub mod datamodel;

const BASE_URL: &str = "https://nyaa.si/";

pub async fn extract_body(params: Option<QueryParameters>) -> Body {
    let html = get_response(params).await;

    let t_body = Body::from_html(&html.unwrap_or("".to_string()));

    match t_body {
        Ok(res) => {
            create_demo_files("result.json", format!("{res:?}"));
            res
        }
        Err(err) => panic!("the extracted Body was not OK {err:#?}"),
    }
}

async fn get_response(params: Option<QueryParameters>) -> Result<String, &'static str> {
    let query_url = get_url(params);

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let response = client.get(&query_url).send().await;

    let mut html = "".to_string();
    match response {
        Ok(it) => {
            html = it.text().await.unwrap_or("".to_string());
            create_demo_files("demo.html", format!("{html:#}"));
        }
        Err(err) => {
            create_demo_files("log.txt", format!("{} \n {}", err, query_url));
        }
    };

    Ok(html)
}

fn get_url(params: Option<QueryParameters>) -> String {
    let mut query_url: String = BASE_URL.to_string();

    match params {
        Some(ps) => {
            query_url.push_str(
                format!(
                    "?f={}&c={}&q={}&p={}",
                    ps.filter.value, ps.category.value, ps.search_query, ps.page
                )
                .as_str(),
            );
        }
        None => {}
    }

    query_url
}

// soley for debugging
fn create_demo_files(filename: &str, data: String) {
    let file = File::create(format!("demo_files/{filename:#}"));
    match file {
        Ok(mut f) => match write!(f, "{:#?}", data) {
            Err(err) => println!("{err:#?}"),
            _ => {}
        },
        Err(err) => {
            println!("{err:#?}");
            panic!()
        }
    }
}
