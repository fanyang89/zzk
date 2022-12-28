use crate::cmd::lib::EmptyWatcher;
use crate::Cli;
use anyhow::Result;
use zookeeper::{ZooKeeper, ZooKeeperExt};

pub struct App<'a> {
    pub cli: &'a Cli,
}

impl<'a> App<'a> {
    pub fn new(cli: &'a Cli) -> Self {
        Self { cli }
    }

    pub fn exists(&self, key: &String) -> Result<String> {
        let zk = ZooKeeper::connect(&self.cli.zoo_hosts, self.cli.get_timeout(), EmptyWatcher)?;
        match zk.exists(key, false)? {
            Some(_) => Ok("true".into()),
            None => Ok("false".into()),
        }
    }

    pub fn delete(&self, key: &String) -> Result<String> {
        let zk = ZooKeeper::connect(&self.cli.zoo_hosts, self.cli.get_timeout(), EmptyWatcher)?;
        zk.delete_recursive(key)?;
        Ok("ok".into())
    }

    pub fn set(&self, key: &String, value: &String) -> Result<String> {
        let zk = ZooKeeper::connect(&self.cli.zoo_hosts, self.cli.get_timeout(), EmptyWatcher)?;
        zk.ensure_path(key)?;
        let (_data, stat) = zk.get_data(key, false)?;
        zk.set_data(key, Vec::from(value.as_bytes()), Some(stat.version))?;
        Ok("ok".into())
    }
}

#[macro_export]
macro_rules! output {
    ($self: expr, $x: expr) => {
        match $self.cli.format {
            crate::cmd::lib::Format::JSON => Ok(crate::cmd::lib::into_json_string($x)),
            crate::cmd::lib::Format::YAML => Ok(crate::cmd::lib::into_yaml_string($x)),
            crate::cmd::lib::Format::Table => Ok(crate::cmd::lib::into_table_string($x)),
        }
    };
}
