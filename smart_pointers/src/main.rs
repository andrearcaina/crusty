use crate::List::{Cons, Nil};
use crate::ListRc::{Cons as ConsRc, Nil as NilRc};
use std::ops::Deref;
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

// defining our own smart pointer
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
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

    // if we try to assert the value of a box, we get a compile-time error:
    // assert_eq!(b, 5); will not compile
    //
    // this is because we cannot dereference a box directly - we need to use the `*` operator to get the value inside the box
    assert_eq!(*b, 5); // if nothing prints and program runs, the assertion passes and the program continues

    let x = 5;
    let y = Box::new(x);

    assert_eq!(x, 5); // check if the value of x is 5
    assert_eq!(*y, 5); // check if the value inside the box is 5

    let z = MyBox::new(x);
    println!("{:?}", z);
    assert_eq!(*z, 5); // check if the value inside the custom box is 5 (works because we implemented Deref for MyBox)

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // deref coercion: &MyBox<String> -> &str
    hello(&*m); // explicit deref to String, then coerced to &str
    hello(&(*m)[..]); // explicit deref + slicing to &str

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

    // c and d are dropped when they go out of scope
    // can't manually drop a box or custom smart pointer
    // c.drop() won't compile
    // drop(c); can be used to drop a box or custom smart pointer, but it's not recommended
    //
    drop(c);
    drop(d);

    println!("CustomSmartPointers dropped before the end of main");

    // sharing data across smart pointers using Rc (reference counted smart pointer)
    // basically a Box is a smart pointer that stores data on the heap and is dropped when it goes out of scope
    // Rc is similar, but it keeps track of how many references to the data exist and only drops the data when there are no more references
    // Rc also shares ownership of the data, so multiple Rc pointers can point to the same data

    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ConsRc(3, Rc::clone(&a)); // basically b shares ownership of the data with a and theres a total of 2 references (with Rc)
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ConsRc(4, Rc::clone(&a)); // c shares ownership of the data with a and theres a total of 3 references (with Rc)
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // once c goes out of scope, the total references drop to 2
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
