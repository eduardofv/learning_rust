use trait1::{Summary, NewsArticle};

/*
impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("Es {}", self)
    }
}
*/

fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("an article")
    }
}

fn main() {
    let news = NewsArticle {
        headline: String::from("Noticias de hoy")
    };

    println!("News: {}", news.summarize());

    let other = returns_summarizable();

    println!("Other {}", other.summarize());

/*
    let entero = 10;
    println!("Entero: {}", entero.summarize());
*/
}
