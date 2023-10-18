use std::time::Duration;

use serde::Deserialize;
use serde_json::json;
use serde_with::{serde_as, DurationSeconds};

use crate::reqwest::REQWEST_CLIENT;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
    pub playability_status: PlayabilityStatus,
    pub streaming_data: StreamingData,
    pub video_details: VideoDetails,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlayabilityStatus {
    pub status: PlayabilityStatusStatus,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PlayabilityStatusStatus {
    #[serde(rename = "OK")]
    Ok,
    Unknown(String),
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingData {
    #[serde(rename = "expiresInSeconds")]
    #[serde_as(as = "DurationSeconds<String>")]
    pub expires_in: Duration,
    pub formats: Vec<StreamingFormat>,
}

#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingFormat {
    #[serde(rename = "itag")]
    pub id: u32,
    pub url: String,
    pub bitrate: u32,
    pub width: u32,
    pub height: u32,
    pub mime_type: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDetails {
    pub title: String,
}

pub async fn get_video_info(video_id: &str) -> reqwest::Result<VideoInfo> {
    REQWEST_CLIENT
        .post("https://www.youtube.com/youtubei/v1/player?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8&prettyPrint=false")
        .header("User-Agent", "com.google.android.youtube/17.31.35 (Linux; U; Android 11) gzip")
        .json(&json!({
            "videoId": video_id,
            "context": {
                "client": {
                    "hl": "en",
                    "gl": "US",
                    "clientName": "ANDROID",
                    "clientVersion": "17.31.35",
                    "androidSdkVersion": 30,
                    "userAgent": "com.google.android.youtube/17.31.35 (Linux; U; Android 11) gzip"
                }
            },
        }))
        .send()
        .await?
        .json()
        .await
}
