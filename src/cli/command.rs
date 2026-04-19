use super::*;

#[derive(Args, Debug, Default)]
pub struct CommandDocsCommand {
    /// Command path like `auth machine connect`
    pub path: Vec<String>,
}
