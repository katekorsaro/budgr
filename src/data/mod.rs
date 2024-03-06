mod operation;
mod read_data;
mod status;

pub use operation::*;
pub use read_data::*;
pub use status::*;

#[test]
fn parse_data() {
    let input: String = String::from(
        "1|active|2024-01-01 10:55:35|2024-01-02 12:55:35|20240101|Note|10000|bank|purpose|goal",
    );
    let operation: Operation = Operation::from_string(&input).unwrap();
    assert_eq!(operation.id, 1);
    assert_eq!(operation.creation_date, String::from("2024-01-01 10:55:35"));
    assert_eq!(
        operation.modification_date,
        Some(String::from("2024-01-02 12:55:35"))
    );
    assert_eq!(operation.date, 20240101);
    assert_eq!(operation.note, "Note".to_string());
    assert_eq!(operation.amount, 10000);
    assert_eq!(operation.account, Some(String::from("bank")));
    assert_eq!(operation.purpose, Some(String::from("purpose")));
    assert_eq!(operation.goal, Some(String::from("goal")));
}
