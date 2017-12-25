'use strict'

// Source: https://twitter.com/_sorceress/status/935256907408867328

let u = require('./utils')

function printState(program) {
  u.clearScreen()
  console.log(iterations + " iterations\n=======================")
  let s = ''
  let i = 0
  for (let i = 0; i < program.length; i++) {
    let instr = program[i]
    if (i === position) {
      s += '[' + instr + ']'
    } else {
      s += ' ' + instr + ' '
    }
  }
  console.log(s)
  u.pause(1000)
}

let program = []
let program2 = []
u.readInputAsLines(function (line) {
  program.push(parseInt(line))
  program2.push(parseInt(line))
})


// Part 1

let position = 0
let iterations = 0

while (position < program.length) {
 // printState()
  let jump = program[position]++
  position += jump
  iterations++
}

console.log("Part 1:", iterations)

// Part 2

position = 0
iterations = 0

while (position < program2.length) {
  //printState(program2)
  let jump = program2[position]
  if (program2[position] >= 3) {
    program2[position]--
  } else {
    program2[position]++
  }
  position += jump
  iterations++
}

console.log("Part 2:", iterations)