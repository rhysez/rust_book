// Traits allow us to define a set of methods that can be used across different types.
// In this example, we want to define shared behaviour between NewsArticle and Tweet.
// Hint: Remember that there are built-in types in Rust that share traits, like the Copy trait for example!

// We create a new public trait named Summary.
// We do not need to provide a function body for summarize().
// However, if we do provide a function body, this will be treated as the method's default behaviour.
// In this scenario, any type that implements the Summary trait will gain access to the summarize method.

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

// Here we're not providing an implementation for summarize() in NewsArticle.
// If we were to call NewsArticle.summarize(), it would use the default behaviour (return "Read more...").
impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

// We implement summarize for the Tweet type.
// In this case, summarize is handled differently, because it is a tweet.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet: Tweet = Tweet {
        username: String::from("rhysez"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false
    };

    let article: NewsArticle = NewsArticle {
        author: String::from("Rhys"),
        headline: String::from("Big Headline!"),
        content: String::from("Some news article content")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("News Article summary: {}", article.summarize());
}
