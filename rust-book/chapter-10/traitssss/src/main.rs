// traits are interfaces from java lol
trait Summary {
    fn summarize(&self) -> String;
    fn another_summarize(&self) -> String {
        String::from("Pending") // This is how we can give a default implementation
    }
}

struct NewsArticle {
    headline: String,
    _location: String,
    author: String,
    content: String 
}

struct Tweet {
    username: String,
    content: String,
    _reply: bool,
    _retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet by {}: {}", self.username, self.content)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}: {}", self.headline, self.author, self.content)
    }
}

fn news_aggregator(source : &impl Summary) {
    println!("There is a new news in the market!");
    println!("{}", source.summarize());
    println!("{}\n\n", source.another_summarize());
}

fn get_news<T: Summary>(news: &T) {
    println!("Breaking news!: {}", news.summarize());
}

// This works
fn mix_news(primary: &impl Summary, secondary: &impl Summary) {
    println!("Primary:{}\n\n\nSecondary:{}", primary.summarize(), secondary.summarize());
}

/*
// This won't
// This would give type error
// Because they can be same or different
// so by default Rust says NO
fn mix_news<T: Summary>(primary: &T, secondary: &T) {
    println!("Primary: {}\n\n\nSecondary: {}", primary.summarize(), secondary.summarize());
}
*/


fn main() {
    let tweet = Tweet {
        username: String::from("This is a username"),
        content: String::from("This is a content"),
        _reply: false,
        _retweet: false
    };

    news_aggregator(&tweet);
    get_news(&tweet);

    let news_article = NewsArticle {
        headline: String::from("This is a headline"),
        _location: String::from("This is a location"),
        author: String::from("This is a author"),
        content: String::from("This is the main content")
    };

    news_aggregator(&news_article);
    get_news(&news_article);


    mix_news(&tweet, &news_article);

    // Mutiple Trait implementation
    // fn notify(item: & (impl Summary + Display))
    // fn notify<T: Summary + Display>(item: &T)
}

// A more cleaner way to do it 
use std::fmt::{Display, Debug};
fn some_function<T, U>(_t: &T, _u: &U) -> i32 
where 
    T: Display + Clone, // The above import
    U: Clone + Debug    // is necessay in this case
{
    1
}

// Mix type returing of impl is not allowed
// i.e. if something happens -> NewsArticle
// else happens -> Tweet 
// This creates a confusion for the compiler so yeah
// We can do this but via box or something
// More advnaced traits will come
// TODO: Blanket trait
