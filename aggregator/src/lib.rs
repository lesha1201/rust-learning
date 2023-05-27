use std::fmt;

//==============================================================================
// Summary
//==============================================================================

pub trait Summary {
    fn summarize(&self) -> String;
}

//==============================================================================
// NewsArticle
//==============================================================================

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//==============================================================================
// Tweet
//==============================================================================

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tweet by @{}", self.username)
    }
}

//==============================================================================
// Utils
//==============================================================================

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn summary_for(item: &(impl Summary + fmt::Display)) {
    println!("Summary for {}: {}", item, item.summarize());
}
