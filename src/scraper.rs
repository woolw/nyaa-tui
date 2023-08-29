use crate::datamodel::*;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use unhtml::FromHtml;

const BASE_URL: &str = "https://nyaa.si/?s=seeders&o=desc";

pub async fn get_body(params: &QueryParameters) -> Body {
    let html = get_response(params).await;

    let t_body = Body::from_html(&html);

    match t_body {
        Ok(res) => res,
        Err(err) => panic!("the extracted Body was not OK {err:#?}"),
    }
}

async fn get_response(params: &QueryParameters) -> String {
    let mut query_url = BASE_URL.to_string();

    query_url.push_str(
        format!(
            "&f={}&c={}&q={}&p={}",
            params.filter.items[params.filter.state.selected().unwrap()].value,
            params.category.items[params.category.state.selected().unwrap()].value,
            params.search_query.search_string,
            params.page
        )
        .as_str(),
    );

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

    html
}
