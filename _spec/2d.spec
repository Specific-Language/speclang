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

shape {
  sides extend list {
    each extend line {
      _next = sides[(index + 1) % sides.length]
      b = _next.a
    }
  }
}

triangle extend shape {
  sides extend list {
    length = 3
  }
}

rectangle extend shape {
  sides {
    each {
      _opposite = sides[(index + 2) % sides.length]
      length = _opposite.length
    }
    length = 4
  }
}

square extend rectangle {
  sides {
    each {
      length = sides[0].length
    }
  }
}
