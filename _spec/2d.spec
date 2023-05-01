namespace = "2d"

point {
  x = number
  y = number
}

line {
  a = point
  b = point
  length = distance(a, b)
}

rectangle {
  sides extend list {
    each extend line {
      origin = (index == 3) ? sides[index - 3] : sides[index + 1]
      opposite = (index % 2 == 0) ? sides[index + 1] : sides[index - 1]
    }
    length = 4
  }
  constraints {
    connected_sides = all(sides, side => side.a.x == side.origin.b.x && side.a.y == side.origin.b.y)
    equal_length_opposite = all(sides, side => side.length == side.opposite.length)
  }
}

square extend rectangle {
  constraints {
    equal_length_all_sides = all(sides, side => side.length == sides[0].length)
  }
}

triangle {
  sides extend list {
    each extend line {
      origin = (index == 2) ? sides[index - 2] : sides[index + 1]
    }
    length = 3
  }
  constraints {
    connected_sides = all(sides, side => side.a.x == side.origin.b.x && side.a.y == side.origin.b.y)
  }
}
