// Traits allow us to define a set of methods that can be used across different types.
// In this example, we want to define share behaviour between NewsArticle and Tweet.

// We create a new public trait named Summary.
// Notice that we do not provide a body to our summarize...
// This is because we do not want to dictate how a given type will implement it.
// However, we do want to ensure that summarize takes a reference to self, and returns a String.
// In this scenario, any type that implements the Summary trait will gain access to the summarize method.
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

// Now we can implement the Summary trait for our chosen type: NewsArticle.
// All we have to do is provide the function body
// AKA, tell Rust what happens when NewsArticle calls summarize.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

// Then we do the same for Tweet
// In this case, summarize is handled slightly differently, because it is a tweet.
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
