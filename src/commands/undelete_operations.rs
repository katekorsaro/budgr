use crate::data::Status;
use crate::Budgr;
use crate::Config;
use crate::commands::set_operation_status::set_operation_status;

pub fn undelete_operations(config: &Config, args: &Budgr) {
  set_operation_status(config, args, Status::Active);
}
