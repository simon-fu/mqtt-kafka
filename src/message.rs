use std::time::Duration;

use crate::tq3::tt;


include!(concat!(env!("OUT_DIR"), "/mqtt.data.rs"));

pub struct ParsedHeader {
    pub(crate) app_id: String,
    pub(crate) header: ControlHeaderInfo,
}

impl std::ops::Deref for ParsedHeader {
    type Target = ControlHeaderInfo;

    fn deref(&self) -> &Self::Target {
        &self.header
    }
}

impl std::fmt::Debug for ParsedHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = &self.header;
        let mut f = f.debug_struct("Header");
        let f = f.field("msgid", &format!("{:016X?}", header.msgid()));
        let f = f.field("connid", &format!("{:016X?}", header.connid));
        let f = f.field("timestamp", &format_milli(header.timestamp));
        let f = f.field("clientid", &header.clientid());
        let f = f.field("user", &header.user);
        f.finish()
    }
}

#[derive(Debug, Default)]
pub struct ParsedMessage {
    pub _n: u64,
    pub ts: String,
    pub _ver: u8,
    pub _etype: Events,
    pub header: Option<ParsedHeader>,
    pub packet: Option<tt::Packet>,
    pub code: Option<String>,
}

#[derive(Debug)]
pub struct ParsedUplinkMessage {
    pub ts: String,
    pub header:ParsedHeader,
    pub packet: tt::Publish,
}


#[inline]
pub fn format_milli(milli: u64) -> String{
    const MILLI_TIME_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3f";
    let t = std::time::SystemTime::UNIX_EPOCH + Duration::from_millis(milli);
    let datetime: chrono::DateTime<chrono::Local> = t.into();
    format!("{}", datetime.format(MILLI_TIME_FORMAT))
}

