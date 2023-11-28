mod eng;
mod obj;

fn main () {
    let ops = eng::list("./data/db.csv");

    match ops {
        Err(ref msg) =>  println!("{msg:?}"),
        Ok(ref ops) =>  print_ops(ops),
    }

    println!("{ops:?}");
}

fn print_ops (ops: &Vec<obj::Op>) {
    let ops = ops.iter()
        .filter(|x| x.date <= 20230110)
        .map(|x| x.date);

    let ops = ops
        .rev()
        .take(3);

    ops
        .for_each(|x| println!("{x}"));
}
