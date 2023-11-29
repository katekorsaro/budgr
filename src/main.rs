mod eng;
mod obj;

fn main () {
    let ops = eng::list("./data/db.csv");

    match ops {
        Err(ref msg) =>  println!("{msg:?}"),
        Ok(ref ops) =>  print_ops(ops, Some(20230110)),
    }
}

fn print_ops (ops: &[obj::Op], from_date:Option<u32>) {
    let ops = ops.iter();

    let ops = ops.filter(|x| x.date >= from_date.unwrap_or(0));

    ops.for_each(|x| println!("{x:?}"));
}
