use worker::*;
use serde_json::json;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/vixsrc", |req, _ctx| async move {
            // Parse the URL and extract the "tmdb" query parameter
            let url = req.url()?;
            let tmdb = url
                .query_pairs()
                .find(|(k, _)| k == "tmdb")
                .map(|(_, v)| v.into_owned())
                .unwrap_or_default();

            // Your actual stream fetching logic would go here
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
