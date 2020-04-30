pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.to_string()
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: &dyn Summary) {
    println!("Notify: Breaking news! {}", item.summarize());
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("Largest number is: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let tweet = Tweet {
        username: String::from("user Filip"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penquins"),
        location: String::from("North pole"),
        author: String::from("Amundsen"),
        content: String::from("Wrecked ship"),
    };

    let article2 = NewsArticle {
        headline: String::from("Penquins"),
        location: String::from("North pole"),
        author: String::from("Amundsen"),
        content: String::from("Wrecked ship"),
    };

    println!("New article available! {}", article.summarize());

    notify(&article2);

    let mut v: Vec<Box<&dyn Summary>> = Vec::new();

    v.push(Box::new(&tweet));
    v.push(Box::new(&article));

    for (count, sum) in v.iter().enumerate() {
        println!("{} {}", count, sum.summarize());
        // notify(sum);
    }

    let mut vec: Vec<&dyn Summary> = Vec::new();
    vec.push(&tweet);
    vec.push(&article);

    for (count, sum) in vec.iter().enumerate() {
        println!("{}: {}", count, sum.summarize());
        // notify(sum);
    }
}
