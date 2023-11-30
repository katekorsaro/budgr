use budgr::list;

fn main() {
    let ops = list("./data/db.csv");

    if ops.is_ok() {
        let ops = ops.unwrap();

        ops.iter().for_each(|x| println!("{x:?}"));
    }
}
