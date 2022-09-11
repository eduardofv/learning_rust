pub trait Summary {
    
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("No summary")
    }

}

pub fn notify_summary(item: &impl Summary) {
//pub fn notify_summary<T: Summary>(item: &T) {
    println!("Notificacion: {}", item.summarize());
}

pub struct NewsArticle {
    pub headline: String
}

impl Summary for NewsArticle {

    fn summarize_author(&self) -> String {
        format!("No author yet")
    }

    fn summarize(&self) -> String {
        let mut num = 6;
        let len = self.headline.chars().count();
        if len < num {
            num = len;
        }

        self.headline.get(..num).unwrap_or("").to_string() + "..."
    }
}
