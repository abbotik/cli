use super::*;

#[derive(Args, Debug)]
pub struct McpCommand {
    #[command(subcommand)]
    pub command: McpSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum McpSubcommand {
    /// List tools exposed by Abbotik's MCP endpoint
    List(McpListCommand),
    /// Call one MCP tool with JSON arguments
    Call(McpCallCommand),
}

#[derive(Args, Debug, Default)]
pub struct McpListCommand {}

#[derive(Args, Debug)]
pub struct McpCallCommand {
    pub tool: String,

    /// Tool arguments as inline JSON, @<path>, or - for stdin
    #[arg(long = "arguments")]
    pub arguments: Option<String>,
}
