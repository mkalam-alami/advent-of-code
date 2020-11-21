'use strict'

let u = require('./utils')

//let input = '0 2 7 0'
let input = '4 10 4 1 8 4 9 14 5 1 14 15 0 15 3 5'
  .split(' ').map(s => parseInt(s))

function choose() {
  let max = 0
  for (let i = 0; i < input.length; i++) {
    if (input[i] > input[max]) {
      max = i
    }
  }
  return max
}

function reallocate(i) {
  let blocks = input[i]
  input[i] = 0
  let index = i
  while (blocks > 0) {
    index = (index + 1) % input.length
    input[index]++
    blocks--
  }
}

let iterations = 0
let history = {}
function seenBefore() {
  let hash = input.join('|')
  if (!history[hash]) {
    history[hash] = iterations
    return false
  } else {
    return history[hash]
  }
}

while (!seenBefore()) {
  let i = choose()
  reallocate(i)
  iterations++
}

console.log("Iterations: " + iterations)
console.log("Looped after: " + (iterations - seenBefore()))