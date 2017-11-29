'use strict'

let u = require('../utils')

let ranges = []

u.readInputAsLines(function (line) {
  let [from, to] = line.split('-').map(function (s) { return parseInt(s) })
  ranges.push({from: from, to: to})
})

ranges = ranges.sort(function (a, b) {
  return a.from - b.from
})

let mergeOccurred
do {
  mergeOccurred = false
  let newRanges = []
  for (let i = 0; i < ranges.length; i++) {
    let range = ranges[i]
    if (!range.deleted) {
      for (let j = 1; j < ranges.length; j++) {
        let otherRange = ranges[j]
        if (range !== null && range !== otherRange && !otherRange.deleted && overlap(range, otherRange)) {
          range = {from: Math.min(range.from, otherRange.from), to: Math.max(range.to, otherRange.to)}
          otherRange.deleted = true
          mergeOccurred = true
        }
      }
      newRanges.push(range)
    }
  }
  ranges = newRanges
} while (mergeOccurred)

function overlap (range1, range2) {
  return range1.to + 1 >= range2.from && range1.from <= range2.to + 1
}

let allowed = 0
for (let i = 0; i < ranges.length - 1; i++) {
  allowed += ranges[i + 1].from - ranges[i].to - 1
}

console.log(ranges[0].to + 1, allowed)
