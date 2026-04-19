use super::*;

#[derive(Args, Debug)]
pub struct AppCommand {
    pub app_name: String,
    pub path: Option<String>,
}
