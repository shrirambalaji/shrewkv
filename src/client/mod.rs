use ntex::http::Client;

use crate::server;

#[ntex::main]
pub async fn ping() -> std::io::Result<()> {
    let client = Client::default();
    let mut response = client
        .get(server::ENDPOINT)
        .header("User-Agent", "shrewdb-cli")
        .send()
        .await
        .unwrap();

    let body = response.body().await.unwrap();
    print!("{:?}", std::str::from_utf8(&body).unwrap());
    Ok(())
}
