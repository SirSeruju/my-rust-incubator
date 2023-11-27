#[allow(clippy::single_component_path_imports)]
use reqwest;

#[tokio::main]
async fn main() {
    let server_url = std::env::var("SERVER_URL").expect("SERVER_URL should be set");
    let args = std::env::args().collect::<Vec<_>>();
    let client = reqwest::Client::new();
    let res = client
        .post(server_url)
        .json(&args)
        .send()
        .await
        .expect("failed to get response");
    println!(
        "{}",
        res.text()
            .await
            .expect("failed to parse text from response")
    );
}
