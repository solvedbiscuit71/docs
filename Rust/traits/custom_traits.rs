struct Comment {
  author: String,
  content: String,
  votes: i32,
}

struct Reply {
  author: String,
  content: String,
  votes: i32,
  replying_to: String,
}

// Summary trait defines the summarize method for
// giving a short summary on the data.
trait Summary {
  fn summarize(&self) -> String;
}

// Implementation...
impl Summary for Comment {
  fn summarize(&self) -> String {
    format!(
      "
A comment made by @{},
'{}' 

which has an vote of {}
    ",
      self.author, self.content, self.votes
    )
  }
}

impl Summary for Reply {
  fn summarize(&self) -> String {
    format!(
      "
A reply made by @{} to @{},
'{}' 

which has an vote of {}
    ",
      self.author, self.replying_to, self.content, self.votes
    )
  }
}

fn main() {
  let commet = Comment {
    author: String::from("solvedbiscuit71"),
    content: String::from("This is awesome"),
    votes: 10,
  };

  println!("{}", commet.summarize());

  let reply = Reply {
    author: String::from("bugmaster"),
    content: String::from("Totally agreed."),
    votes: 5,
    replying_to: String::from("solvedbiscuit71"),
  };

  println!("{}", reply.summarize());
}
