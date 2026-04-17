use super::*;

#[derive(Args, Debug)]
#[command(after_long_help = APP_AFTER_HELP)]
pub struct AppCommand {
    pub app_name: String,
    pub path: Option<String>,
}
