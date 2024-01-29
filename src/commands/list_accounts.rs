use crate::commands::filter_data::filter_data;
use crate::data::read_data;
use crate::Budgr;
use crate::Config;
use std::collections::HashSet;

pub fn list_accounts(config: &Config, args: &Budgr) {
  let data = read_data(config);
  let data = filter_data(data, args);
  let mut accounts: HashSet<String> = HashSet::new();
  data.into_iter().for_each(|operation| {
    accounts.insert(operation.account.clone().unwrap_or_default());
  });
  let mut accounts = accounts.iter().collect::<Vec<_>>();
  accounts.sort();
  accounts
    .iter()
    .filter(|account| !account.is_empty())
    .for_each(|account| println!("{account}"));
}
