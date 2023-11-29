mod eng;
mod obj;

fn main () {
    let ops = eng::list("./data/db.csv");

    match ops {
        Err(ref msg) =>  println!("{msg:?}"),
        Ok(ref ops) =>  print_ops(ops, Some(20230110), Some("New description...")),
    }
}

fn print_ops (ops: &[obj::Op], from_date:Option<u32>, desc:Option<&str>) {
    let ops = ops.iter();

    // filter
    let ops = ops.filter(|x| x.date >= from_date.unwrap_or(0));

    // map
    let ops = ops.map(|x| obj::Op {
        desc: String::from(desc.unwrap_or(&x.desc)),
        ..*x
    });

    // print
    ops.for_each(|x| println!("{x:?}"));
}
