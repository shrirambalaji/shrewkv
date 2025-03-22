use crate::server;
use ntex::http::Client;
use serde_json::json;

#[ntex::main]
pub async fn ping() -> std::io::Result<()> {
    let client = Client::default();
    let mut response = client
        .get(format!("http://{}/ping", server::ENDPOINT))
        .header("User-Agent", "shrewkv-cli")
        .send()
        .await
        .unwrap();

    let body = response.body().await.unwrap();
    print!("{:?}", std::str::from_utf8(&body).unwrap());
    Ok(())
}

#[ntex::main]
pub async fn set(key: String, value: String) -> std::io::Result<()> {
    let client = Client::default();
    let payload = server::SetRequestBody {
        // TODO: avoid clone.
        value: value.clone(),
    };
    let mut response = client
        .put(format!("http://{}/set/{}", server::ENDPOINT, key))
        .header("User-Agent", "shrewkv-cli")
        .send_json(&json!(payload))
        .await
        .unwrap();

    let body = response.body().await.unwrap();
    print!("{:?}", std::str::from_utf8(&body).unwrap());
    Ok(())
}

#[ntex::main]
pub async fn get(key: String) -> std::io::Result<()> {
    let client = Client::default();
    let mut response = client
        .get(format!("http://{}/get/{}", server::ENDPOINT, key))
        .header("User-Agent", "shrewkv-cli")
        .send()
        .await
        .unwrap();

    let body = response.body().await.unwrap();
    print!("{:?}", std::str::from_utf8(&body).unwrap());
    Ok(())
}
