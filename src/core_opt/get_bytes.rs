use bytes::Bytes;
use color_eyre::eyre::Result;
use reqwest::Client;
use std::collections::HashMap;

pub async fn req_bytes(search_params: &str) -> Result<Bytes> {
    let uri = "https://www.upwork.com/ab/feed/jobs/rss";
    let query_data: HashMap<&str, &str> =
        HashMap::from([("sort", "recency"), ("q", search_params)]);

    let response_byte = Client::new()
        .get(uri)
        .query(&query_data)
        .send()
        .await?
        .bytes()
        .await?;

    Ok(response_byte)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_req_bytes() {
        let res = req_bytes("javascript").await;
        dbg!(&res);

        assert!(res.is_ok());
    }
}
