use super::*;

#[derive(Args, Debug, Default)]
pub struct ConfigCommand {
    #[command(subcommand)]
    pub command: Option<ConfigSubcommand>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigSubcommand {
    /// Create a named config profile
    Create(ConfigCreateCommand),
    /// Set the current default profile
    Use(ConfigUseCommand),
    /// List named config profiles
    List,
    /// Show one named config profile
    Show(ConfigShowCommand),
    /// Set or unset one config key on a named profile
    Set(ConfigSetCommand),
    /// Get one config key from a named profile
    Get(ConfigGetCommand),
    /// Delete a named config profile
    Delete(ConfigDeleteCommand),
    /// Run a local integrity check on config data only
    Doctor,
}

#[derive(Args, Debug)]
pub struct ConfigCreateCommand {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Args, Debug)]
pub struct ConfigUseCommand {
    pub name: String,
}

#[derive(Args, Debug)]
pub struct ConfigShowCommand {
    pub name: String,
}

#[derive(Args, Debug)]
pub struct ConfigSetCommand {
    pub name: String,
    pub key: String,
    pub value: Option<String>,

    #[arg(long, conflicts_with = "value")]
    pub unset: bool,
}

#[derive(Args, Debug)]
pub struct ConfigGetCommand {
    pub name: String,
    pub key: String,
}

#[derive(Args, Debug)]
pub struct ConfigDeleteCommand {
    pub name: String,
}
