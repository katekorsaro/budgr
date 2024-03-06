use crate::commands::set_operation_status::set_operation_status;
use crate::config::Config;
use crate::data::Status;
use crate::Budgr;

pub fn delete_operations(config: &Config, args: &Budgr) {
    set_operation_status(config, args, Status::Deleted);
}
