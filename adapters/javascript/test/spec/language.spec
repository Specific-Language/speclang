// fancy things
// language hooks
// declare namespaces
// generics?!

// rust
declare rust-parser {}
declare rust-lib {
  assert {
    input rust-parser-string {}
    expected rust-parser-string {
      default = true
    }
    evaluate {
      input = "assert!(${input});"
      output = expected
    }
  }
  assert-equal value1 value2 {
    value1 rust-parser-string {}
    value2 rust-parser-string {}
    assert {
      input = "${value1 = value2}"
    }
  }
}

// speclang
declare speclang-lib {
  evaluate {
    input speclang-eval-string {}
    output speclang-parser-type {}
  }
}

declare math {
  percentage number {
    minimum = 0
    maximum = 100
  }
}

declare knowledge {
  fact {}
  confidence {
    fact {}
    percentage {}
  }
  known confidence {
    value = 100
  }
  unknown confidence {
    rust-lib-assert {
      input = "${confidence-percentage < 100}"
    }
  }
}
