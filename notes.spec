// everything is an object by default 
// some objects adapt to primitives. fill with language features
// example : string : { value, get, set, length }

list primitive {
  // -> magic -> JS array
}

object primitive {
  // -> magic -> JS object
}

hcl2 {
  block {
    name string {}
    value {}
  }
  attribute {
    name string {}
    value hcl2.expression {}
  }
  expression primitive {
    // anything after the =
  }
}

specification hcl2.block {
  value.entries each constraint {}
}

constraint {
  value {}
  pattern {
    test = value
  }
}

pattern {
  test speclang.expression {}
  match speclang.function {
    input = test
    output = true
  }
}
