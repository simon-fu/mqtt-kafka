#[macro_use]
extern crate serde_derive;
extern crate serde;

use anyhow::{Result, Context};
use tracing::metadata::LevelFilter;
use time::UtcOffset;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::EnvFilter;
use time::macros::format_description;

mod mqtt_kafka;
pub mod tokio_rt;
pub mod kafka_util;
pub mod db;
pub mod message;
pub mod tq3;

fn main() -> Result<()> {
    let offset = UtcOffset::current_local_offset()
    .with_context(||"should get local offset!")?;

    // https://time-rs.github.io/book/api/format-description.html
    let fmts = format_description!("[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");
    let timer = OffsetTime::new(offset, fmts);

    tracing_subscriber::fmt::fmt()
    .with_timer(timer)
    .with_target(false)
    .with_env_filter(
        EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
    )
    .init();

    

    tokio_rt::run_multi_thread(async move {
        mqtt_kafka::run().await
    })??;

    Ok(())
}
