use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use eyre::eyre;
use maud::{html, DOCTYPE};
use moka::{future::Cache, Expiry};
use tracing::{debug, error, field, instrument};

use crate::{
    reqwest::REQWEST_CLIENT,
    youtube::{get_video_info, VideoInfo},
};

type VideoCache = Cache<String, Arc<CachedVideo>>;

#[derive(Clone, Debug)]
struct CachedVideo {
    url: String,
    expires_in: Duration,
    title: String,
    width: u32,
    height: u32,
    mime_type: String,
}

impl Expiry<String, CachedVideo> for CachedVideo {
    fn expire_after_create(
        &self,
        _id: &String,
        video: &CachedVideo,
        _created_at: Instant,
    ) -> Option<Duration> {
        Some(video.expires_in)
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/:video_id", get(embed_video))
        .with_state(VideoCache::new(100))
}

async fn embed_video(Path(video_id): Path<String>, State(cache): State<VideoCache>) -> Response {
    match get_cached_video(&video_id, cache).await {
        Ok(video) => html! {
            (DOCTYPE)
            html {
                head {
                    meta charset="utf-8";
                    meta name="robots" content="noindex";
                    title { (video.title) }
                    meta property="og:title" content=(video.title);
                    meta property="og:site_name" content="YouTube";
                    meta property="theme-color" content="#ff0000";
                    meta property="og:type" content="video.other";
                    meta property="og:url" content=(format!("https://www.youtube.com/watch?v={video_id}"));
                    meta property="og:video" content=(video.url);
                    meta property="og:video:type" content=(video.mime_type);
                    meta property="og:video:width" content=(video.width);
                    meta property="og:video:height" content=(video.height);
                }
                body {
                    video preload="none" style="display: none;" {
                        source src="https://valve-software.com/videos/gabeNewell.mp4" type="video/mp4";
                    }
                    h1 { (video.title) }
                    video controls width=(video.width) height=(video.height) {
                        source src=(video.url) type=(video.mime_type);
                    }
                    p {
                        a href=(format!("https://www.youtube.com/watch?v={video_id}")) {
                            "Watch on YouTube"
                        }
                    }
                }
            }
        }
        .into_response(),
        Err(err) => {
            error!(error = field::display(err));
            Redirect::temporary(&format!("https://www.youtube.com/watch?v={video_id}")).into_response()
        },
    }
}

#[instrument(skip(cache))]
async fn get_cached_video(video_id: &str, cache: VideoCache) -> eyre::Result<Arc<CachedVideo>> {
    if let Some(video) = cache.get(video_id).await {
        if REQWEST_CLIENT
            .head(&video.url)
            .send()
            .await?
            .status()
            .is_success()
        {
            debug!("cache hit");
            return Ok(video);
        }
        debug!("cache invalidated");
        cache.invalidate(video_id).await;
    } else {
        debug!("cache miss");
    }

    let VideoInfo {
        video_details,
        streaming_data,
        ..
    } = get_video_info(video_id).await?;

    let mut formats = streaming_data.formats;
    formats.sort_unstable_by(|a, b| b.bitrate.cmp(&a.bitrate));

    let format = formats
        .into_iter()
        .next()
        .ok_or(eyre!("no formats found"))?;

    let video = Arc::new(CachedVideo {
        url: format.url,
        expires_in: streaming_data.expires_in,
        title: video_details.title,
        width: format.width,
        height: format.height,
        mime_type: format.mime_type,
    });

    cache.insert(video_id.to_string(), video.clone()).await;

    Ok(video)
}
