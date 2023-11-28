mod eng;
mod obj;

fn main () {
    let ops = eng::list("./data/db.csv");

    match ops {
        Err(_) => panic!(),
        Ok(ops) => ops.iter().for_each(|op| println!("{op:?}")),
    }
}
