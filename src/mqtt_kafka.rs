
use std::time::Duration;
use anyhow::{Result, Context};
use bytes::Buf;
use rdkafka::{consumer::{BaseConsumer, Consumer}, ClientConfig, Message};
use sqlx::MySqlPool;
use tracing::info;

use crate::{kafka_util::get_offset_for_time_rfc3339, message::{self, Events, ParsedHeader, format_milli, ParsedUplinkMessage}, tq3::{tt::{self, Protocol}, tbytes::PacketDecoder}, db::try_insert_message};

#[derive(Debug)]
pub struct Args {
    brokers: String,
    topic: String,
    timeout_secs: u64, 
    time_str: String,
    group_id: String,
}


pub async fn run() -> Result<()> { 

    // 初始化数据库：
    // export DATABASE_URL="mysql://root:root@localhost/em_mqtt"
    // 在工程目录下，创建 .env 文件，文件内容 
    //    DATABASE_URL="mysql://root:root@localhost/em_mqtt"
    // sqlx db create  如果用 sqllite，用这条命令创建 db 文件
    // sqlx migrate run 创建数据库表

    // 运行 kafka代理
    // 窗口1: ./run-port-map.sh
    // 窗口2: ./run-kafka-proxy-server.sh

    // 运行本程序：
    // export DATABASE_URL="mysql://root:root@localhost/em_mqtt"
    // RUST_LOG=debug,sqlx=warn cargo run


    let args = Args {
        brokers: "127.0.0.1:30001".into(),
        topic: "threeq-uplink-messages1".into(),
        timeout_secs: 10,
        time_str: "2023-06-15T10:11:12+08:00".into(),
        group_id: "example_consumer_group_id".into(),
    };

    info!("{:?}", args);

    let db_url = std::env::var("DATABASE_URL")?;
    let pool = if !db_url.is_empty() {
        let pool = MySqlPool::connect(&db_url).await
            .with_context(||format!("fail to connect to db [{}]", db_url))?;
        info!("connected to db [{}]", db_url);
        Some(pool)
    } else {
        None
    };
    
    
    let timeout = Some(Duration::from_secs(args.timeout_secs));

    let offset_tpl = get_offset_for_time_rfc3339(
        &args.brokers, 
        &args.topic, 
        &args.time_str,
        timeout,
    )?;

    {
        // let offset_list: PartiOffsetList = (&offset_tpl).into();
        info!("offsets for time {}", args.time_str);
        for ele in offset_tpl.elements().iter() {
            info!("  partition[{:02}] = {:?}", ele.partition(), ele.offset());
        }
    }

    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", args.brokers)
        .set("group.id", args.group_id)
        .create()
        .with_context(|| "creation failed: {:?}")?;

    info!("assigning partitions...");
    consumer.assign(&offset_tpl)
        .with_context(||"assign offset list fail")?;
    info!("assign partitions done");
    
    let h = tokio::spawn(async move {
        let msg = consumer.poll(None)
            .with_context(||"consumer poll fail")??;
        info!("got msg, parti[{:02}][{}], {:?}", msg.partition(), msg.offset(), msg.timestamp());

        let payload = msg.payload().with_context(||"none payload")?;
        let mut cursor = payload;
        let ver = cursor.get_u8();
        cursor.advance(2);
        let etype = cursor.get_u8();
        let etype = message::Events::from_i32(etype as i32).with_context(||"decode event type fail")?;

        let ts = {
            let r = msg.timestamp().to_millis();
            if let Some(n) = r {
                format_milli(n as u64)
            } else {
                "None".into()
            }
        };


        match etype {
            Events::Uplink => {
                let m = <message::UplinkMessage as prost::Message>::decode(cursor)?;




                let header = m.header.with_context(||"no header when parsing uplink messag")?;

                let mut app_id = None;
                {
                    let mut split = header.clientid().split("@");
                    while let Some(id) = split.next() {
                        app_id = Some(id);
                    }
                }
                let app_id = app_id.with_context(||"extract app_id fail")?.to_string();

                let mut cursor = &m.packet[..];
                let fixed_header = tt::check(cursor.iter(), 65536)?;
                let packet = tt::Publish::decode(Protocol::from_u8(ver)?, &fixed_header, &mut cursor)?;

                let kmsg = ParsedUplinkMessage {
                    ts,
                    header: ParsedHeader {
                        app_id,
                        header,
                    },
                    packet,
                };


                
                // info!("-- No.{}: [{}], ver [{}], [{:?}], {:?}", n, kmsg.ts, ver, etype, flag.flags);
                info!("  {:?}", kmsg);

                if let Some(pool) = pool.as_ref() {
                    let inserted = try_insert_message(pool, &kmsg).await?;
                    info!("  inserted {:?}", inserted);
                }
                
                info!("");
            },
            _ => {
                info!("  {:?}, ts {}", etype, ts);
                info!("");
            },
        }
        

        Result::<()>::Ok(())
    });
    h.await??;

    Ok(())
}



