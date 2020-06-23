use lnrust::lib::list;
#[test]
pub fn test_list_display() {
    let v = list::List::new(vec![1, 2, 3]);
    println!("{}", v);
}
