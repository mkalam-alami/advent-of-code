'use strict'

let u = require('../utils')

const ROWS = 400000

let previousLine = '.^..^....^....^^.^^.^.^^.^.....^.^..^...^^^^^^.^^^^.^.^^^^^^^.^^^^^..^.^^^.^^..^.^^.^....^.^...^^.^.'
let result = log(previousLine, 0)

for (let i = 1; i < ROWS; i++) {
  let currentLine = ''
  for (let j = 0; j < previousLine.length; j++) {
    let source = read(previousLine, j - 1) + read(previousLine, j) + read(previousLine, j + 1)
    switch (source) {
      case '^^.':
      case '.^^':
      case '^..':
      case '..^':
        currentLine += '^'
        break
      default:
        currentLine += '.'
    }
  }
  let newSafe = u.countArray(Array.from(currentLine), '.')
  result += log(currentLine, i)
  previousLine = currentLine
}

console.log('total: ' + result)

function log (line, i) {
  let safe = u.countArray(Array.from(line), '.')
  console.log(i + ': ' + line + ' (' + safe + ')')
  return safe
}

function read (line, i) {
  if (i < 0 || i >= line.length) return '.'
  else return line[i]
}
