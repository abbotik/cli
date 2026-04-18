use super::*;

#[derive(Args, Debug, Default)]
#[command(after_long_help = DOCTOR_AFTER_HELP)]
pub struct DoctorCommand {}
