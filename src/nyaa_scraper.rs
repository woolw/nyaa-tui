use crate::datamodel::*;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use unhtml::FromHtml;

const BASE_URL: &str = "https://nyaa.si/";

pub async fn get_body(params: Option<QueryParameters>) -> Body {
    let html = get_response(params).await;

    let t_body = Body::from_html(&html.unwrap_or("".to_string()));

    match t_body {
        Ok(res) => res,
        Err(err) => panic!("the extracted Body was not OK {err:#?}"),
    }
}

pub async fn get_response(params: Option<QueryParameters>) -> Result<String, &'static str> {
    let mut query_url = BASE_URL.to_string();

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

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let response = client.get(&query_url).send().await;

    let mut html = "".to_string();
    match response {
        Ok(it) => {
            html = it.text().await.unwrap_or("".to_string());
        }
        Err(err) => {
            println!("failed after 5 retries: {err:#?}")
        }
    };

    Ok(html)
}
