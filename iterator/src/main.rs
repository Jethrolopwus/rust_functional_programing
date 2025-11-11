struct Employee {
    name: String,
    salary: u16,
}

struct EmployeeRecords {
    employee_db: Vec<Employee>,
}

impl Iterator for EmployeeRecords {
    type Item = String;
    
    // fn next(&mut self) -> Option<Self::Item> {
    //     if !self.employee_db.is_empty() {
    //         let result = self.employee_db[0].name.clone();
    //         self.employee_db.remove(0);
    //         Some(result)
    //     } else {
    //         None
    //     }
    // }

    fn next(&mut self) -> Option<Self:: Item> {
        if self.employee_db.len() != 0 {
            let result = self.employee_db[0].name.clone();
            self.employee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}


//Intoiterator //

struct Book {
    title: String,
    author: String,
    genre: String,
}

// struct BookIterator {
//     properties: Vec<String>,
// }

// impl Iterator for BookIterator {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         if !self.properties.is_empty() {
//             Some(self.properties.remove(0))
//         } else {
//             None
//         }
//     }
// }

    impl IntoIterator for Book {
        type Item = String;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
        vec![self.title, self.author, self.genre].into_iter()
        }
    }

fn main() {
    let emp_1 = Employee {
        name: String::from("John"),
        salary: 40_000,
    };
    
    let emp_2 = Employee {
        name: String::from("Jethro"),
        salary: 30_000,
    };
    
    let mut emp_db = EmployeeRecords {
        employee_db: vec![emp_1, emp_2],
    };
    
    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());

    //using for loop

    for employee in emp_db {
        println!("{employee}");
    }



    // using intoiterator in main


    let book = Book {
        title: String::from("Digital Image Processing"),
        author: String::from("Gonzale"),
        genre: String::from("Science Book"),
    };
    let mut book_iterator = book.into_iter();

    // println!("{:?} ", book_iterator.next());
    // println!("{:?} ", book_iterator.next());
    // println!("{:?} ", book_iterator.next());

    for book_info in book_iterator{
        println!("{book_info} ");
    }
}