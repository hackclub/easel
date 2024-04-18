export class EaselError extends Error {
  constructor(msg) {
    super()
    this.message = msg
    this.value = msg
  }
}

class Canvas {
  constructor(rows = 64, cols = 64) {
    this.rows = rows
    this.cols = cols
    this.grid = []
    for (let i = 0; i < cols * rows; i++) {
      this.grid.push({ r: 0, g: 0, b: 0 })
    }
  }

  fill(x, y, color) {}

  erase(x, y) {
    let index = y * this.cols
  }
}

export default {
  Canvas: new Canvas(),
  Color: members => {
    let instance = {}
    for (let key of Object.keys(members)) {
      if (!['r', 'g', 'b'].includes(key))
        throw new Error(`Unexpected member ${key}`)
      instance[key] = members[key]
    }
    return instance
  },
  ink: args => console.log(...args),
  random: range => {
    const [min, max] = range
    return Math.random() * (max - min + 1) + min
  },
  round: number => Math.round(number)
}
