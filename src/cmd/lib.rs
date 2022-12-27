use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tabled::object::{Columns, Object, Rows};
use tabled::{Alignment, ModifyObject, Style, Table, Tabled};
use zookeeper::{Stat, Watcher};

pub fn into_table_string<'a, I, T>(data: &'a I) -> String
where
    &'a I: IntoIterator<Item = T>,
    T: Tabled,
{
    Table::new(data)
        .with(Style::re_structured_text())
        .with(
            Rows::new(1..)
                .not(Columns::first())
                .modify()
                .with(Alignment::left()),
        )
        .to_string()
}

pub fn into_json_string<T>(data: &T) -> String
where
    T: serde::ser::Serialize,
{
    serde_json::to_string_pretty(data).unwrap()
}

pub fn into_yaml_string<T>(data: &T) -> String
where
    T: serde::ser::Serialize,
{
    serde_yaml::to_string(data).unwrap()
}

pub struct EmptyWatcher;
impl Watcher for EmptyWatcher {
    fn handle(&self, _event: zookeeper::WatchedEvent) {}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TabledStat {
    pub czxid: i64,
    pub mzxid: i64,
    pub ctime: i64,
    pub mtime: i64,
    pub version: i32,
    pub cversion: i32,
    pub aversion: i32,
    pub ephemeral_owner: i64,
    pub data_length: i32,
    pub num_children: i32,
    pub pzxid: i64,
}

impl From<Stat> for TabledStat {
    fn from(value: Stat) -> Self {
        Self {
            czxid: value.czxid,
            mzxid: value.mzxid,
            ctime: value.ctime,
            mtime: value.mtime,
            version: value.version,
            cversion: value.cversion,
            aversion: value.aversion,
            ephemeral_owner: value.ephemeral_owner,
            data_length: value.data_length,
            num_children: value.num_children,
            pzxid: value.pzxid,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Format {
    Table,
    JSON,
    YAML,
}
