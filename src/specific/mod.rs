use std::str::FromStr;
use std::{fmt, str};
use std::collections::BTreeMap;
use hcl::Value;
use indexmap::IndexMap;
use self::builder::{
    Builder, 
    value::Specific
};

pub mod builder;

pub struct Context {
    pub tree: BTreeMap<String, Specific>
}

impl Context {
    pub fn new() -> Self {
        Self {
            tree: BTreeMap::new()
        }
    }

    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn collect_prefix(&self, prefix: &str, sep: char) -> Vec<(&String, &Specific)> {
        let mut end_bound = prefix.to_string();
        end_bound.push(sep);
        if let Some(last_char) = end_bound.pop() {
            end_bound.push((last_char as u8 + 1) as char);
        } else {
            end_bound.push('\0');
        }
        self.tree.range(prefix.to_string()..end_bound).collect()
    }
}

impl FromStr for Context {
    type Err = String; // todo : specificerror enum

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parsed_value: Value = hcl::from_str(input)
            .map_err(|err| err.to_string())?;
        
        println!("{:?}", parsed_value);
        
        let parsed_obj = parsed_value
            .as_object()
            .ok_or("Expected parsed value to be a Value::Object".to_string())?;
        
        Ok(Self::from(parsed_obj))
    }
}

impl From<&IndexMap<String, Value>> for Context {
    fn from(input: &IndexMap<String, Value>) -> Self {
        Context::builder()
            .apply_object("", input)
            .build()
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("tree", &self.tree.iter().collect::<Vec<_>>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::specific::builder::value::{Expression, Operator, Binary};

    use super::*;

    #[test]
    fn test_basic() {
        let input = r#"
            a = 1
            b = 2
            c = a + b
        "#;
        let specific = Context::from_str(input).unwrap();
        println!("{:#?}", specific);
        assert_eq!(
            specific.tree.get("a").unwrap(),
            &Specific::Literal(Value::Number(1.into()))
        );
        assert_eq!(
            specific.tree.get("b").unwrap(),
            &Specific::Literal(Value::Number(2.into()))
        );
        assert_eq!(
            specific.tree.get("c").unwrap(),
            &Specific::Expression(Box::new(Expression {
                left: Specific::Reference("a".to_string()),
                op: Operator::Binary(Binary::Add),
                right: Specific::Reference("b".to_string()),
            }))
        );
    }

    #[test]
    fn test_2d() {
        let input = r#"
        point {
            x = number // shorthand for x number {}
            y = number
        }

        line {
            start = point
            end = point

            length = sqrt(pow(end.x - start.x, 2) + pow(end.y - start.y, 2))
            // since this isnt a Literal, and its not a Reference, it is a derived value
        
            from function {
                input = [start, end]
                output line {
                    start = from.input[0]
                    end = from.input[1]
                }
            }
        }
        
        circle {
            center = point
            radius = number
            area = MathPi * radius * radius
            circumference = 2 * MathPi * radius
        }
        
        triangle {
            points { 
                a = point
                b = point
                c = point
            }
            sides {
                ab = line.from(points.a, points.b)
                bc = line.from(points.b, points.c)
                ca = line.from(points.c, points.a)
            }
            perimeter = sides.ab.length + sides.bc.length + sides.ca.length
        }
        "#;
        let specific = Context::from_str(input).unwrap();
        println!("{:#?}", specific);
    }
    
    #[test]
    fn test_from() {
        let input = r#"
            bird {
                flying = true
                wings {
                    feathered = true
                    count = 2
                }
            }
            cat {
                meow = true
            }
            duck bird {
                quack = true
            }
            wuck duck {
                flying = false
            }
            bings "bird.wings" {
                count = 4
            }
            dird {
                name = string
                wuck {
                    wings = bings
                }
                cat {
                    meow = false
                }
            }
        "#;
        let specific = Context::from_str(input).unwrap();
        println!("{:#?}", specific);
    }

    #[test]
    fn test_weekly() {
        let input = r#"
        weekly_tracker_grid {
            rows = list(weekly_tracker_row)
            rows {
                count = 10
            }
        }
        
        weekly_tracker_row {
            mantra = string
            days {
                Sunday = boolean
                Monday = boolean
                Tuesday = boolean
                Wednesday = boolean
                Thursday = boolean
                Friday = boolean
                Saturday = boolean
            }
            // each specific is a snapshot in time. expressions should be evaluated as part of construction
            average = days.each[value ? 1 : 0].sum / days.length
        }
        "#;
        let specific = Context::from_str(input).unwrap();
        println!("{:#?}", specific);
    }

    #[test]
    fn test_coordinates() {
        let input = r#"
            latitude number {
                range = "[-90, 90]"
            }
            longitude number {
                range = "[-180, 180]"
            }
            coordinates { 
                // "any group that fulfills these subtraits is an instance of `coordinates`"
                latitude {}
                longitude {}
            }
            location {
                coordinates {}
                // impl region, language, laws, etc
            }
        "#;
        let specific = Context::from_str(input).unwrap();
        println!("{:#?}", specific);
    }

    #[test]
    fn test_salida() {
        let input = r#"
        // ibu = number // shorthand for `ibu { number {} }`

        // beer {
        //     brewery {}
        //     ibu {}
        // }

        // // todo : function calls
        // // beers = "list(beer)"

        // brewery {
        //     location {} // {} means "can be filled by any specific trait"
        //     beers {}
        // }

        // // specific traits are capitalized. proper nouns. can exist in real world
        // Reviresco {
        //     beer {
        //         brewery = TresLitros // assignment creates a specific "pin" in the trait. a "specific trait"
        //     }
        // }

        // need to diagram EVERYTHING out here. array vs object after hcl parse, etc
        TresLitros brewery {
            location = Salida
            beers = [Reviresco]
        }

        TresLitros live_music_venue {
            name string {}
            event_calendar {}
        }

        // Salida location {
        //     coordinates {
        //         latitude = 38.5342
        //         longitude = -105.9980
        //     }
        // }
        "#;
        let specific = Context::from_str(input).unwrap();
        println!("{:#?}", specific);
    }
}
