use crate::commands::set_operation_status::set_operation_status;
use crate::data::Status;
use crate::Budgr;
use crate::Config;

pub fn delete_operations(config: &Config, args: &Budgr) {
  set_operation_status(config, args, Status::Deleted);
}
