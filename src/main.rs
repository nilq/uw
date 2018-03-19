extern crate colored;

mod uw;

use uw::lexer::*;
use uw::source::*;

fn main() {
  let test = r#"
outer := 1

{
  inner := outer

  {
    innerer := inner

    {
      innest := innerer
    }
  }
}

outerer := outer
  "#;

  let source = Source::from("main.rs/testing.wu", test.lines().map(|x| x.into()).collect::<Vec<String>>());
  let mut lexer = Lexer::default(test.chars().collect(), &source);

  for token in lexer {
    println!("{:#?}", token)
  }
}
