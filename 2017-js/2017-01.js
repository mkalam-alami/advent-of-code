'use strict'

let u = require('./utils')

let sum = 3
let sum2 = 0

let prev = -1

u.readInput(function (line) {
  let offset = line.length / 2
  let i = 0

  for (let c of line) {
    if (prev === c) sum += parseInt(c)
    prev = c

    let opposite = line[(i + offset) % line.length]
    if (prev === opposite) sum2 += parseInt(c)
    i++
  }
})

console.log("Sum: " + sum)
console.log("Sum2: " + sum2)