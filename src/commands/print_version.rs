pub fn print_version() {
  let version = env!("CARGO_PKG_VERSION");
  let name = env!("CARGO_PKG_NAME");
  println!("{name}: {version}");
}
