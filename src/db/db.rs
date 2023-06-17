use sqlx::{MySqlPool, mysql::MySqlDatabaseError};
use anyhow::Result;

use crate::message::ParsedUplinkMessage;

pub async fn try_insert_message(pool: &MySqlPool, msg: &ParsedUplinkMessage) -> Result<Option<u64>> {
    let r = insert_message(pool, msg).await;
    match r {
        Ok(id) => Ok(Some(id)),
        Err(e) => {
            
            if check_any_dup_entry_err(&e) {
                return Ok(None)
            }

            Err(e)
        },
    }
}



pub async fn insert_message(pool: &MySqlPool, msg: &ParsedUplinkMessage) -> Result<u64> {
    
    // let header = msg.header.as_ref()
    //     .with_context(||"no header when insert msg")?;

    // let packet = msg.packet.as_ref()
    //     .with_context(||"no packet when insert msg")?;

    let connid = format!("{:016X}", msg.header.connid); 

    let id = sqlx::query!(
        r#"
INSERT INTO uplink_messages ( app_id, msg_id, client_id, conn_id, topic, qos, publish_time, payload)
VALUES ( ?, ?, ?, ?, ?, ?, ?, ? )
        "#,
        msg.header.app_id,
        msg.header.msgid,
        msg.header.clientid(),
        connid,
        msg.packet.topic,
        msg.packet.qos as u8,
        msg.ts,
        &msg.packet.payload[..],
    )
    .execute(pool)
    .await?
    .last_insert_id();

    Ok(id)
}

pub struct DbUplinkMessage {
    pub id: u64,
    pub create_time: u64,
    pub app_id: String,
    pub msg_id: u64,
    pub client_id: String,
    pub conn_id: String,
    pub topic: String,
    pub qos: u8,
    pub publish_time: u64,
    pub payload: Vec<u8>,
}


fn  check_any_dup_entry_err(e: &anyhow::Error) -> bool {
    let r: Option<&sqlx::Error> = e.downcast_ref();
    if let Some(sqlx_err) = r {
        if let sqlx::Error::Database(db_err) = sqlx_err {
            if let Some(mysql_err) = db_err.as_ref().try_downcast_ref::<MySqlDatabaseError>() {
                // https://dev.mysql.com/doc/mysql-errors/8.0/en/server-error-reference.html
                if mysql_err.number() == 1062 { // Duplicate entry 
                    return true
                }
            }
        }
    }
    false
}
