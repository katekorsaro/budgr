#[derive(PartialEq, Debug)]
pub struct Op {
    date: u16,
    desc: String,
    val: i32
}

impl Op {
    pub fn new () -> Op {
        Op {
            date: 0u16,
            desc: String::from(""),
            val: 0i32
        }
    }

    pub fn from (str_val: &str) -> Op {
        let mut op = Op::new();
        op.desc = String::from(str_val);

        op
    }
}
