pub struct Op {
    pub id: u32,
    pub date: u32,
    pub desc: String,
    pub val: i32
}

impl Op {
    pub fn new () -> Op {
        Op {
            id: 0,
            date: 0,
            desc: String::new(),
            val: 0
        }
    }

    pub fn from (str_val:&str) -> Op {
        let tokens:Vec<&str> = str_val.split('|').collect();

        Op {
            id: tokens[0].parse().unwrap(),
            date: tokens[1].parse().unwrap(),
            desc: String::from(tokens[2]),
            val: tokens[3].parse().unwrap(),
        }
    }
}

impl Default for Op {
    fn default () -> Op {
        Op::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn op_new () {
        let op = super::Op::new();

        assert_eq!(op.id, 0);
        assert_eq!(op.date, 0);
        assert_eq!(op.desc, String::new());
        assert_eq!(op.val, 0);
    }

    #[test]
    fn op_from_string () {
        let str_val = "1|20230101|Description 001|-1000";

        let op = super::Op::from(str_val);

        assert_eq!(op.id, 1);
        assert_eq!(op.date, 20230101);
        assert_eq!(op.desc, String::from("Description 001"));
        assert_eq!(op.val, -1000);
    }
}
