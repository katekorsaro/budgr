mod eng;
mod obj;

fn main () {
    let ops = eng::list("./data/db.csv");

    match ops {
        Err(_) => panic!(),
        Ok(ops) => {
            for op in ops {
                println!("{op:?}");
            }
        }
    }
}
