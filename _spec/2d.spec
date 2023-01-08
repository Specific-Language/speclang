namespace = "2d"

point {
  x = number
  y = number
}

side extend list {
  each = point
  length = 2
}

shape {
  sides extend list {
    each = side
    # sides connect to form a shape
  }
  angles extend list {
    each = number
    length = sides.length
    # angles match sides
    # computed?
  }
}

rectangle extend shape {
  angles = [90, 90, 90, 90]
}

square extend rectangle {
  sides {
    # only 1 length value within list (all side lengths equal)
  }
}
