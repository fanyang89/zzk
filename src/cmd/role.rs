use core::fmt;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::{output, App};

#[derive(Tabled, Serialize, Deserialize, Debug)]
pub struct RoleRow {
    host: String,
    role: Role,
}

#[derive(Serialize, Deserialize, Debug)]
enum Role {
    Follower,
    Leader,
    Standalone,
    Unknown,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Role {
    fn as_str(&self) -> &'static str {
        match self {
            Role::Follower => "Follower",
            Role::Leader => "Leader",
            Role::Standalone => "Standalone",
            Role::Unknown => "Unknown",
        }
    }
}

fn parse_srvr(buf: &String) -> Role {
    let mut p = buf.find("Mode: follower");
    if !p.is_none() {
        return Role::Follower;
    }

    p = buf.find("Mode: leader");
    if !p.is_none() {
        return Role::Leader;
    }

    p = buf.find("Mode: standalone");
    if !p.is_none() {
        return Role::Standalone;
    }

    Role::Unknown
}

impl App<'_> {
    pub fn get_role(self) -> Result<String> {
        let mut rows: Vec<RoleRow> = Vec::new();

        for addr in self.cli.get_zoo_hosts() {
            let mut stream = TcpStream::connect(addr.clone())?;
            stream.write("stat".as_bytes())?;

            let mut buffer = String::new();
            stream.read_to_string(&mut buffer)?;
            let mode = parse_srvr(&buffer);

            rows.push(RoleRow {
                host: addr,
                role: mode,
            })
        }

        output!(&self, &rows)
    }
}
