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

circle {
  center = point
  radius = number
  circumference = 2 * pi * radius
  area = pi * radius * radius
}

polygon {
  N = number
  sides extends list {
    length = N
    each extends line {
      next_side = sides[(index + 1) % N]
      b = next_side.a
    }
  }
}

triangle extends polygon {
  N = 3
}

equilateral_triangle extends triangle {
  sides each {
    length = sides[0].length
  }
}

acute_triangle extends triangle {
  angles each {
    is_acute = value > 0 && value < 90
  }
}

isosceles_triangle extends triangle {
  is_isosceles = (sides[0].length == sides[1].length) || (sides[1].length == sides[2].length) || (sides[0].length == sides[2].length)
}

rectangle extends polygon {
  N = 4
  sides each {
    opposite_side = sides[(index + 2) % sides.length]
    length = opposite_side.length
  }
}

square extends rectangle {
  sides each {
    length = sides[0].length
  }
}
