use crate::define;
use crate::context;
use serde_json::Value;
use serde_json::Number;

#[test]
fn success() {
  let input = r#"
  claw {}
  claws = [claw,claw,claw,claw,claw]
  // claws list {
  //   type = claw
  //   N = 5
  // }
  paw {
    claws {}
  }
  // cat {
  //   paws {
  //     front = [paw, paw]
  //     back = [paw, paw]
  //   }
  // }
  // George cat {
  //   age = 15
  //   temperament = temperaments.wellbehaved
  //   hungry = (time.unixref.hrs - lastfed.hrs) > 4
  //   paws-front-paw0-injured = true
  // }
  "#;
  let mut context = context::ContextNode::new();
  define(&mut context, "test", input);
  println!("trie {}", context);
  assert_eq!(
    context.get("test.George.cat.age").unwrap().value, 
    Some(Value::Number(Number::from(15)))
  );
  assert_eq!(
    context.get("test.George.cat.temperament").unwrap().value, 
    Some(Value::String("${temperaments.wellbehaved}".to_owned()))
  );
  // assert_eq!(
  //   trie.get("cat.claws.sharp"), 
  //   Some(&Value::Bool(true))
  // );
  // assert_eq!(
  //   trie.get("cat.claws.retractable"), 
  //   Some(&Value::Bool(true))
  // );
  // assert_eq!(
  //   trie.get("cat.claws.claw.0.length.cm"), 
  //   Some(&Value::Number(Number::from_f64(1.25).unwrap()))
  // );
  // assert_eq!(
  //   trie.get("cat.claws.claw.1.paw"),
  //   Some(&Value::String("${paw.back.right}".to_owned()))
  // );
}
