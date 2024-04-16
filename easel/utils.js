class Color {
  constructor(r, g, b) {
    this.r = r
    this.g = g
    this.b = b
  }
}

class Canvas {
  constructor(rows = 64, cols = 64) {
    this.rows = 64
    this.cols = 64
  }
}

export default {
  Canvas: new Canvas(),
  Color,
  ink: args => console.log(args)
}
