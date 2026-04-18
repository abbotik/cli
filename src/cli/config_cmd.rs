use super::*;

#[derive(Args, Debug, Default)]
#[command(after_long_help = CONFIG_AFTER_HELP)]
pub struct ConfigCommand {}
