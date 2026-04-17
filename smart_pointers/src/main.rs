use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // a box is a smart pointer that allocates data on the heap and provides a pointer to it
    // it is used to create recursive data structures, because it allows us to have a pointer to the next element in the list, which is necessary for a linked list
    let b = Box::new(5);
    println!("b = {}", b);

    // an example of a recursive data structure is a linked list, which is a collection of nodes where each node contains a value and a pointer to the next node in the list
    // in lisp, the expression (1 . (2 . (3 . nil))) can be represented in rust as:
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // basically this is a linked list where the first node contains the value 1 and a pointer to the next node,
    // which contains the value 2 and a pointer to the next node, which contains the value 3 and a pointer to nil, which indicates the end of the list

    // in order to print the values in the list, we can use pattern matching to recursively traverse the list and print each value:
    fn print_list(list: &List) {
        match list {
            Cons(value, next) => {
                print!("{} -> ", value);
                print_list(next);
            }
            Nil => {
                println!("Nil");
            }
        }
    }

    print_list(&list);
}
