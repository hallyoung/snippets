//1-1
// define trait
pub trait Summary{
    fn summarize(&self)->String;
}
pub struct Post{
    pub title:String,
    pub author:String,
    pub content:String,

}

pub struct Weibo{
    pub user_name:String,
    pub content:String,
}

impl Summary for Post{
    fn summarize(&self) -> String {
        format!("{}-{}",self.author,self.title)
    }
}

impl Summary for Weibo{
    fn summarize(&self) -> String {
        format!("{}-{}",self.user_name,self.content)
    }
}

fn main() {
    let post=Post{
        title: "中国人失去想象力了吗？".to_string(),
        author: "沧海一笑".to_string(),
        content: "从盘古开天、嫦娥奔月的浪漫想象，到“天宫”遨游、“嫦娥”落月的伟大实践，中国人从不缺乏想象力，更不断积蓄着把美好想象变为生动现实的创造力。".to_string(),
    };
    let weibo=Weibo{ user_name: "探花郎".to_string(), content: "我不喜欢戴手表，因为时间是一个沉甸甸的东西".to_string() };
    println!("Post:{}",post.summarize());
    println!("Weibo:{}",weibo.summarize());
}
