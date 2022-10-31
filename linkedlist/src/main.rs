#[derive(Debug)]
struct List<T> {
    value: T,
    next: Box<Option<List<T>>>
}

fn insert<T: std::cmp::PartialOrd>(val: T, li: Option<List<T>>) -> Option<List<T>> {
    match li {
        None => Some(List{value: val, next: Box::new(None)}),
        Some(List{value, next}) if value > val => Some(List{value: val, next: Box::new(Some(List{value, next}))}),
        Some(List{value, next}) => Some(List{value, next: Box::new(insert(val, *next))}),
    }
}

fn remove<T: std::cmp::PartialEq>(val: T, li: Option<List<T>>) -> Option<List<T>> {
    match li {
        None => None,
        Some(List{value, next}) if value == val => *next,
        Some(List{value, next}) => Some(List{value, next: Box::new(remove(val, *next))})
    }
}

fn find<T: std::cmp::PartialEq>(val: T, li: &Option<List<T>>) -> bool {
    match li {
       None => false,
       Some(List{value, next: _}) if value == &val => true,
       Some(List{value: _, next}) => find(val, next)
    }
}

fn main() {
    {
        let li = insert(42, None);
        let li = insert(44, li);
        let li = insert(45, li);
        let li = insert(43, li);
        let li = insert(41, li);
        dbg!(&li);
        let li = remove(41, li);
        let li = remove(43, li);
        let li = remove(45, li);
        let li = remove(46, li);
        dbg!(&li);
    }
    {
        let li = insert("foo", None);
        let li = insert("zebra", li);
        let li = insert("zoo", li);
        let li = insert("mountain", li);
        let li = insert("", li);
        dbg!(&li);
        let li = remove("", li);
        let li = remove("mountain", li);
        let li = remove("zoo", li);
        let li = remove("tree", li);
        dbg!(&li);
        dbg!(find("foo", &li));
        dbg!(find("zoo", &li));
    }
}
