use crate::define;
use crate::types;
use serde_json::Value;
use serde_json::Number;

#[test]
fn success() {
  let input = r#"cat {
    name = "George"
    age = 15
    temperament = temperaments-wellbehaved
    hungry = (time-unixref-hrs - lastfed-hrs) > 4
    claws {
      sharp = true
      retractable = true
      claw {
        paw = paw-front-left
        position = 3
        length { cm = 1.25 }
      }
      claw {
        paw = paw-back-right
        position = 1
        length { cm = 2 }
      }
    }
  }
  "#;
  let mut trie = types::Trie::new();
  define(&mut trie, input);
  assert_eq!(
    trie.get("root-cat-name"), 
    Some(&Value::String("George".to_owned()))
  );
  assert_eq!(
    trie.get("root-cat-age"), 
    Some(&Value::Number(Number::from(15)))
  );
  assert_eq!(
    trie.get("root-cat-temperament"), 
    Some(&Value::String("${temperaments-wellbehaved}".to_owned()))
  );
  assert_eq!(
    trie.get("root-cat-claws-sharp"), 
    Some(&Value::Bool(true))
  );
  assert_eq!(
    trie.get("root-cat-claws-retractable"), 
    Some(&Value::Bool(true))
  );
  assert_eq!(
    trie.get("root-cat-claws-claw-0-length-cm"), 
    Some(&Value::Number(Number::from_f64(1.25).unwrap()))
  );
  assert_eq!(
    trie.get("root-cat-claws-claw-1-paw"),
    Some(&Value::String("${paw-back-right}".to_owned()))
  );
}
