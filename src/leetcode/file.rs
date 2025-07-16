


  use std::fs::File;
  use std::io::{self, Read};

  pub trait Summany {
      fn summarize(&self) -> String;
  }

  struct NewsArticle {
    pub handline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }

  impl Summany for NewsArticle {
      fn summarize(&self) -> String {
          format!("{}, by {} ({})", self.handline, self.author, self.location)
      }
  }

  pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
  }

  impl Summany for Tweet {
      fn summarize(&self) -> String {
          format!("{}: {}", self.username, self.content)
      }
  }

  fn returns_summarizable(swicth: bool) -> impl Summany {
    Tweet {
      username: String::from("horse_ebooks"),
      content: String::from("of course, as you probably already know, people",),
      reply: false,
      retweet: false,
    }
  }

  #[cfg(test)]
  mod tests {
    fn it_works() {
      let result = 2 + 2;
      assert_eq!(result, 4);
    }
  }


  pub fn open_file () -> Result<String, io::Error> {
    let mut greeting_file_result = File::open("hello.txt")?;

    let mut username = String::new();

    greeting_file_result.read_to_string(&mut username)?;
    Ok(username)

    // let _greeting_file = match greeting_file_result {
    //   Ok(file)=>file,
    //   Err(error)=> panic!("Problem opening the file: {:?}", error),
    // };
  }
