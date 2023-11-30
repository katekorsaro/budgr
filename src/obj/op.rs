#[derive(PartialEq, Debug)]
pub struct Op {
    pub id: u32,
    pub date: u32,
    pub desc: String,
    pub val: i32,
}

impl Op {
    pub fn new() -> Op {
        Op {
            id: 0u32,
            date: 0u32,
            desc: String::from(""),
            val: 0i32,
        }
    }

    pub fn from(str_val: &str) -> Op {
        let tokens: Vec<&str> = str_val.split('|').collect();

        Op {
            id: tokens[0].parse().unwrap(),
            date: tokens[1].parse().unwrap(),
            desc: tokens[2].to_string(),
            val: tokens[3].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn op_from() {
        let op = super::Op::from("1|20230101|Description for operation 001|+1000");

        assert_eq!(op.id, 1);
        assert_eq!(op.date, 20230101);
        assert_eq!(op.desc, String::from("Description for operation 001"));
        assert_eq!(op.val, 1000);

        let op = super::Op::from("1|20230101|Description for operation 001|-1000");

        assert_eq!(op.id, 1);
        assert_eq!(op.date, 20230101);
        assert_eq!(op.desc, String::from("Description for operation 001"));
        assert_eq!(op.val, -1000);
    }
}
