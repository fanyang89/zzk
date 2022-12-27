use crate::cmd::lib::{EmptyWatcher, TabledStat};
use crate::{output, App};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str;
use tabled::Tabled;
use zookeeper::ZooKeeper;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetResult {
    key: String,
    value: String,
    stat: TabledStat,
}

impl Tabled for GetResult {
    const LENGTH: usize = 13;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        vec![
            self.key.to_string(),
            self.value.to_string(),
            self.stat.czxid.to_string(),
            self.stat.mzxid.to_string(),
            self.stat.ctime.to_string(),
            self.stat.mtime.to_string(),
            self.stat.version.to_string(),
            self.stat.cversion.to_string(),
            self.stat.aversion.to_string(),
            self.stat.ephemeral_owner.to_string(),
            self.stat.data_length.to_string(),
            self.stat.num_children.to_string(),
            self.stat.pzxid.to_string(),
        ]
        .into_iter()
        .map(|v| Cow::Owned(v.to_string()))
        .collect()
    }

    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            "key",
            "value",
            "czxid",
            "mzxid",
            "ctime",
            "mtime",
            "version",
            "cversion",
            "aversion",
            "ephemeral_owner",
            "data_length",
            "num_children",
            "pzxid",
        ]
        .into_iter()
        .map(|v| Cow::Owned(v.to_string()))
        .collect()
    }
}

impl App<'_> {
    pub fn get(&self, key: &String, watch: &bool) -> Result<String> {
        let zk = ZooKeeper::connect(&self.cli.zoo_hosts, self.cli.get_timeout(), EmptyWatcher)?;
        let rc = zk.get_data(key, *watch)?;
        let (data, stat) = rc;
        let rc: GetResult = GetResult {
            key: key.to_string(),
            value: str::from_utf8(data.as_slice()).unwrap().into(),
            stat: stat.into(),
        };
        output!(&self, &vec![rc])
    }
}
