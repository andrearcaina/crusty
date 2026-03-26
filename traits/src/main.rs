// a trait in Rust is like an interface in other languages. basically it defines a set of methods
pub trait Summary {
    fn summarize(&self) -> String;

    // traits can also have "default" implementations for some or all of their methods
    fn summarize_default(&self) -> String {
        String::from("random default method that can be overridden")
    }
}

// this is a struct that implements the Summary trait
#[derive(Debug)] // this also makes NewsArticle implement the Debug trait, which allows us to print it out easily (in debug format)
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// this is an implementation of the Summary trait for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

// this is a struct that also implements the Summary trait
#[derive(Debug)]
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// this is an implementation of the Summary trait for Tweet
impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Number {
    fn return_number(&self) -> u8 {
        0
    }
}

pub struct NumberStruct {
    number: u8,
}

impl Number for NumberStruct {}

pub struct NumberStruct1 {
    number: u8,
}

impl Number for NumberStruct1 {
    fn return_number(&self) -> u8 {
        self.number
    }
}

fn main() {
    let news = NewsArticle {
        headline: String::from("something news"),
        location: String::from("Toronto"),
        author: String::from("Andre"),
        content: String::from("NewsNewsNewsNewsNewsNews"),
    };

    println!("{:?}", news);
    println!("{:?}", news.summarize());
    println!("{:?}", news.summarize_default());

    let social = SocialPost {
        username: String::from("andre"),
        content: String::from("SocialSocialSocialSocialSocialSocial"),
        reply: false,
        retweet: false,
    };

    println!("{:?}", social);
    println!("{:?}", social.summarize());
    println!("{:?}", social.summarize_default());

    let number = NumberStruct { number: 42 };
    println!("{:?}", number.return_number()); // will return 0 since we didn't override return_number for NumberStruct

    let number = NumberStruct1 { number: 42 };
    println!("{:?}", number.return_number()); // will return 42 since we overrided return_number for NumberStruct1

    // both effectively do the same thing, the difference is in the syntax
    print_number(&number);
    print_number_with_bound(&number);

    // the following below won't work since there isn't a struct that implements both number and summary trait
    // print_number_with_multiple_bounds(&number);

    // impl Trait syntax
    print_with_impl(&number, &social);

    // above but with separate bounds
    print_with_separate_bounds(&number, &social);

    // above but with where clauses
    print_with_where(&number, &social);

    // impl Trait syntax for returning a trait
    // return_number_trait returns a struct that implements the Number trait
    let number = return_number_trait();
    println!("{:?}", number.return_number());
}

// using traits/interfaces as parameters
pub fn print_number(number: &impl Number) {
    println!("{:?}", number.return_number());
}

// trait bound syntax
// this is the most verbose way to write a trait bound (effectively does the same thing as the above function)
pub fn print_number_with_bound<T: Number>(number: &T) {
    println!("{:?}", number.return_number());
}

// if there are multiple trait bounds, you can use the `+` syntax
// this works if number has both Number and Summary traits implemented
pub fn print_number_with_multiple_bounds<T: Number + Summary>(number: &T) {
    println!("{:?}", number.return_number());
    println!("{:?}", number.summarize());
}

// u can also do the above like this
pub fn print_number_with_multiple_bounds_impl(number: &(impl Number + Summary)) {
    println!("{:?}", number.return_number());
    println!("{:?}", number.summarize());
}

// if there are different params with different traits, we can use impl Trait syntax
pub fn print_with_impl(number: &impl Number, summary: &impl Summary) {
    println!("{:?}", number.return_number());
    println!("{:?}", summary.summarize());
}

// the above can be rewritten like this
pub fn print_with_separate_bounds<T: Number, U: Summary>(number: &T, summary: &U) {
    println!("{:?}", number.return_number());
    println!("{:?}", summary.summarize());
}

// the above can also be rewritten with where clauses (effectively does the same thing as the above function)
pub fn print_with_where<T, U>(number: &T, summary: &U)
where
    T: Number,
    U: Summary,
{
    println!("{:?}", number.return_number());
    println!("{:?}", summary.summarize());
}

// u can also return types that implement traits
pub fn return_number_trait() -> impl Number {
    NumberStruct1 { number: 67 }
}
