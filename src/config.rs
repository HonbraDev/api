use std::net::SocketAddr;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: SocketAddr,
}

fn default_listen_addr() -> SocketAddr {
    ([0, 0, 0, 0], 3000).into()
}
