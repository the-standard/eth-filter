use lazy_static::lazy_static;
use redis::{Client, ConnectionLike};
use std::env;

pub type Redis = redis::aio::Connection<>;

lazy_static::lazy_static!{
    pub static ref REDIS_CLIENT: redis::Client = {
        // TODO not localhost - env needed
        redis::Client::open("redis://127.0.0.1/").unwrap()
    };
}

pub fn init() {
    lazy_static::initialize(&REDIS_CLIENT);
}

pub async fn connection() -> Redis {
    return REDIS_CLIENT.get_tokio_connection().await.unwrap();
}
