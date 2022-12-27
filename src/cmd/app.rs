use crate::Cli;

pub struct App<'a> {
    pub cli: &'a Cli,
}

impl<'a> App<'a> {
    pub fn new(cli: &'a Cli) -> Self {
        Self { cli }
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
