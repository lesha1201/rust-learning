use aggregator::{notify, summary_for, NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("lesha1201"),
        content: String::from("Hello, World!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        author: String::from("Alexey Ryabov"),
        content: String::from("Hello, World!"),
        headline: String::from("How to write \"Hello, World!\""),
        location: String::from("St. Petersburg"),
    };

    notify(&article);

    summary_for(&tweet);
}
