use super::*;

#[derive(Args, Debug, Default)]
pub struct UpdateCommand {
    /// Show published release versions without installing anything
    #[arg(long, conflicts_with = "version")]
    pub version_list: bool,

    /// Install a specific published release version like v1.7.1
    #[arg(long)]
    pub version: Option<String>,
}
