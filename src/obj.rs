#[derive(PartialEq, Debug)]
pub struct Op {
    pub date: u32,
    pub desc: String,
    pub val: i32
}

impl Op {
    pub fn new () -> Op {
        Op {
            date: 0u32,
            desc: String::from(""),
            val: 0i32
        }
    }

    pub fn from (str_val: &str) -> Op {
        let tokens:Vec<&str> = str_val.split('|').collect();

        let mut op = Op::new();
        op.date = String::from(tokens[0]).parse().unwrap();
        op.desc = String::from(tokens[1]);
        op.val = String::from(tokens[2]).parse().unwrap();

        op
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn op_from () {
        let op = super::Op::from("20230101|Description for operation 001|+1000");

        assert_eq!(op.date, 20230101);
        assert_eq!(op.desc, String::from("Description for operation 001"));
        assert_eq!(op.val, 1000);

        let op = super::Op::from("20230101|Description for operation 001|-1000");

        assert_eq!(op.date, 20230101);
        assert_eq!(op.desc, String::from("Description for operation 001"));
        assert_eq!(op.val, -1000);
    }
}
