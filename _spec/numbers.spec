negative extend number {
  range = "(-inf,0)"
}

range extend string {
  regex match {
    pattern = "(?:\(|\[)(?:-inf|\d+|\d+\.\d+)(?:, ?)(?:inf|\d+|\d+\.\d+)(?:\)|\])"
  }
}
