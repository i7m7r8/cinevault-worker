use worker::*;
use serde_json::json;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/vixsrc", |req, _ctx| async move {
            let query = req.url()?.query_pairs();
            let tmdb = query
                .find(|(k, _)| k == "tmdb")
                .map(|(_, v)| v)
                .unwrap_or_default()
                .to_string();

            // Replace this with your actual stream fetching logic
            let stream_url = format!("https://vixsrc.to/playlist/{}.m3u8?token=...", tmdb);

            Response::from_json(&json!({
                "streams": [{
                    "url": stream_url,
                    "quality": "auto",
                    "type": "hls",
                    "referer": "https://vixsrc.to/"
                }],
                "masterUrl": stream_url
            }))
        })
        .run(req, env)
        .await
}
