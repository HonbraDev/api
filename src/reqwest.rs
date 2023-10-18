use lazy_static::lazy_static;

lazy_static! {
    pub static ref REQWEST_CLIENT: reqwest::Client = reqwest::Client::new();
}
