use reqwest::{
    multipart::{Form, Part},
    Client,
};

pub async fn append(
    client: &Client,
    url: &str,
    token: &str,
    media_id: &u64,
    data: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Separate the data into chunks of 5MB
    let chunks = data.chunks(5 * 1024 * 1024);

    let mut tasks = Vec::new();

    for (i, chunk) in chunks.enumerate() {
        let form = Form::new()
            .text("command", "APPEND")
            .text("media_id", media_id.to_string())
            .text("segment_index", i.to_string())
            .part("media", Part::bytes(chunk.to_vec()));

        let client = client.clone();
        let url = url.to_string();
        let token = token.to_string();

        let task = tokio::spawn(async move {
            client
                .post(url)
                .bearer_auth(token)
                .multipart(form)
                .send()
                .await
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await??;
    }

    Ok(())
}
