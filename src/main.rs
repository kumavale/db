use std::collections::HashMap;

#[derive(Debug)]
enum Type {
    Int(Option<i32>),
    Text(Option<String>),
}

#[derive(Debug)]
struct Table {
    column: Vec<(String, Type)>,
    pub data:   Vec<HashMap<String, Type>>,
}

impl Table {
    fn new(column: Vec<(String, Type)>) -> Self {
        let data = Vec::new();
        Table { column, data }
    }

    fn insert(&mut self, data: Vec<(&str, Type)>) {
        let mut hashmap = HashMap::new();
        for d in data {
            hashmap.insert(d.0.to_string(), d.1);
        }

        self.data.push(hashmap);
    }
}

fn main() {
    let mut t = Table::new( vec![
        (String::from("id"),    Type::Int(None)),
        (String::from("name"),  Type::Text(None)),
        (String::from("price"), Type::Int(None)),
    ]);

    t.insert(vec![("id",    Type::Int(Some(1))),
                  ("name",  Type::Text(Some("apple".to_owned()))),
                  ("price", Type::Int(Some(50)))]);

    t.insert(vec![("id",    Type::Int(Some(2))),
                  ("name",  Type::Text(Some("banana".to_owned()))),
                  ("price", Type::Int(Some(100)))]);

    t.insert(vec![("id",    Type::Int(Some(3))),
                  ("name",  Type::Text(Some("ctrus".to_owned()))),
                  ("price", Type::Int(None))]);


    println!("{:?}", t.data[0].get("name"));
    //println!("{:?}", t.data[0]["nama"]);

    println!("{:?}", t);
}
