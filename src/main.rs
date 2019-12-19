use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Type {
    Int(Option<i32>),
    Text(Option<String>),
}

#[derive(Clone, Debug)]
struct Table {
    name:     String,
    order:    Vec<String>,
    column:   HashMap<String, Type>,
    max_lens: HashMap<String, usize>,
    data:     Vec<HashMap<String, Type>>,
}

impl Table {
    fn new(name: &str, col: Vec<(&str, Type)>) -> Self {
        let mut order    = Vec::new();
        let mut column   = HashMap::new();
        let mut max_lens = HashMap::new();
        for c in col {
            order.push(c.0.to_owned());
            column.insert(c.0.to_owned(), c.1);
            max_lens.insert(c.0.to_owned(), c.0.len());
        }
        Table { name: name.to_owned(),
                order,
                column,
                max_lens,
                data: Vec::new(),
        }
    }

    fn insert(&mut self, data: Vec<(&str, Type)>) {
        let mut hashmap = HashMap::new();
        for d in data {
            if let Some(_) = &self.column.get(d.0) {
                let len = match &d.1 {
                    Type::Int(Some(i))  => i32_len(*i),
                    Type::Text(Some(t)) => t.len(),
                    _ => 4, // null
                };
                if self.max_lens[d.0] < len {
                    self.max_lens.insert(d.0.to_owned(), len);
                }
                hashmap.insert(d.0.to_owned(), d.1);
            } else {
                panic!(format!("Unknown column \"{}\" in \"{}\"", d.0, self.name));
            }
        }

        self.data.push(hashmap);
    }

    fn select(&self, cols: Vec<&str>) -> Table {
        let mut new_t = self.clone();
        new_t.column.clear();
        new_t.order.clear();
        for col in &cols {
            new_t.column.insert(col.to_owned().to_string(), self.column[col.clone()].clone());
        }
        new_t.order = cols.iter().map(|&c| c.to_string()).collect();
        new_t
    }

    fn less_than(&self, col: &str, num: i32) -> Table {
        let mut new_t = self.clone();
        new_t.data.clear();
        for d in &self.data {
            if let Type::Int(i) = d[col] {
                match i {
                    Some(n) => {
                        if n < num {
                            new_t.data.push(d.clone());
                        }
                    },
                    None => (),
                }
            }
        }
        new_t
    }


    fn display(&self) {
        let line = || {
            print!(" +");
            for key in self.order.iter() {
                print!("-{}-+", "-".repeat(self.max_lens[key]));
            }
            println!();
        };

        line();
        print!(" |");
        for key in self.order.iter() {
            print!(" {}{} |", key, " ".repeat(self.max_lens[key] - key.len()));
        }
        println!();
        line();

        for d in &self.data {
            for key in self.order.iter() {
                print!(" | ");
                let mut len: usize = 0;
                if let Some(v) = d.get(key) {
                    len = match v {
                        Type::Int(Some(i))  => {
                            print!("{}", i);
                            i32_len(*i)
                        },
                        Type::Text(Some(t)) => {
                            print!("{}", t);
                            t.len()
                        },
                        _ => {
                            print!("null");
                            4
                        },
                    };
                }
                print!("{}", " ".repeat(self.max_lens[key] - len));
            }
            println!(" |");
        }

        line();
    }
}

fn i32_len(i: i32) -> usize {
    let mut len: usize = 0;
    let mut i = if i < 0 {
        len += 1;
        i.abs()
    } else {
        i.abs()
    };

    while i > 0 {
        i /= 10;
        len += 1;
    }

    len
}

fn main() {
    let mut table1 = Table::new( "table1",
        vec![ (("id",    Type::Int(None))),
              (("name",  Type::Text(None))),
              (("price", Type::Int(None))), ]);

    table1.insert(vec![("id",    Type::Int(Some(1))),
                       ("name",  Type::Text(Some("apple".to_owned()))),
                       ("price", Type::Int(Some(50)))]);

    table1.insert(vec![("id",    Type::Int(Some(2))),
                       ("name",  Type::Text(Some("banana".to_owned()))),
                       ("price", Type::Int(Some(100)))]);

    table1.insert(vec![("id",    Type::Int(Some(3))),
                       ("name",  Type::Text(Some("citrus".to_owned()))),
                       ("price", Type::Int(None))]);

    println!("\n====[ ALL ]====");
    table1.display();

    println!("\n====[ SELECT ]====");
    table1.select(vec!["id"]).display();
    table1.select(vec!["name", "price"]).display();

    println!("\n====[ WHERE < ]====");
    table1.less_than("id", 10).display();
    table1.less_than("price", 250).display();
}

