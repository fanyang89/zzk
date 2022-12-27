use crate::cmd::app::App;
use crate::cmd::lib::EmptyWatcher;
use crate::output;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::str;
use tabled::Tabled;
use zookeeper::{ZooKeeper, ZooKeeperExt};

#[derive(Tabled, Serialize, Deserialize, Debug)]
pub struct ListResultWithValue {
    key: String,
    value: String,
}

#[derive(Tabled, Serialize, Deserialize, Debug)]
pub struct ListResult {
    key: String,
}

impl App<'_> {
    pub fn list(&self, root: &String, recursive: &bool, show_value: &bool) -> Result<String> {
        let zk = ZooKeeper::connect(&self.cli.zoo_hosts, self.cli.get_timeout(), EmptyWatcher)?;
        if *show_value {
            let rc = self.do_list_with_value(&zk, root, recursive)?;
            output!(&self, &rc)
        } else {
            let rc = self.do_list(&zk, root, recursive)?;
            output!(&self, &rc)
        }
    }

    fn do_list_with_value(
        &self,
        zk: &ZooKeeper,
        root: &String,
        recursive: &bool,
    ) -> Result<Vec<ListResultWithValue>> {
        let keys = self.do_list(zk, root, recursive)?;
        let rc: Vec<ListResultWithValue> = keys
            .iter()
            .map(|key| {
                let (data, _stat) = zk
                    .get_data(key.as_str(), false)
                    .context(format!("key: {}", key))
                    .unwrap();
                let value = str::from_utf8(data.as_slice()).unwrap().to_string();
                ListResultWithValue {
                    key: key.to_string(),
                    value,
                }
            })
            .collect();
        Ok(rc)
    }

    fn do_list(&self, zk: &ZooKeeper, root: &String, recursive: &bool) -> Result<Vec<String>> {
        let mut rc: Vec<String>;
        if !(*recursive) {
            rc = zk.get_children(root, false)?;
        } else {
            if root.trim() != "/" {
                rc = zk.get_children_recursive(root)?;
            } else {
                // tricky impl for root
                rc = Vec::new();
                for key in zk.get_children(root, false)?.iter() {
                    let mut children = zk.get_children_recursive(format!("/{}", key).as_str())?;
                    rc.append(&mut children);
                }
            }
        }
        Ok(rc)
    }
}
