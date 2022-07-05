declare foo {
  bar {
    attribute = value
  }
}

declare foo {
  declare bar {
    attribute = value
  }
}

foo { 
  bar {
    attribute = value
  }
}

foo { 
  declare bar {
    attribute = value
  }
}
