'use strict'

let u = require('./utils')

let checksum = 0
let divisibleSum = 0

u.readInputAsLines(function (line) {
  let row = line.split('\t')

  // Part 1
  let min = 99999
  let max = 0
  for (let value of row) {
    min = Math.min(min, parseInt(value))
    max = Math.max(max, parseInt(value))
  }
  checksum += max - min

  // Part 2
  let rowResult = -1
  for (let i in row) {
    for (let j in row) {
      if (i !== j) {
        let division = Math.max(row[i],row[j])/Math.min(row[i],row[j])
        if (Number.isInteger(division)) {
          rowResult = division
          break
        }
      }
    }
    if (rowResult !== -1) break
  }
  divisibleSum += rowResult
})

console.log("Checksum: " + checksum)
console.log("Divisible sum: " + divisibleSum)