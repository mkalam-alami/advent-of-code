let fs = require('fs')
let childProcess = require('child_process')

let mock = null

module.exports = {
  l: console.log,
  exit: exit,

  readInput,
  readInputAsLines,
  readInputAsChars,
  mockInput,

  pause,
  clearScreen,

  createArray,
  countArray,
  sumArray,

  createGrid,
  countGrid,
  sumGrid,
  fillGrid,
  printGrid,

  sortObjectByKeys,

  setCharAt,

  each
}

function exit (message, afterNCalls) {
  if (afterNCalls) {
    if (message) console.log(message)
    process.exit(0)
  }
}

function mockInput (input) {
  mock = input
}

function readInput (callback, indirect) {
  if (mock) {
    callback(mock)
  } else {
    let fileContents = fs.readFileSync(_callerFileName(indirect ? 4 : 3) + '.txt')
    callback(fileContents.toString())
  }
}

function readInputAsLines (callback) {
  readInput(function (input) {
    input.split(/\r?\n/g).forEach(callback)
  }, true)
}

function readInputAsChars (callback) {
  readInput(function (input) {
    for (let i = 0; i < input.length; i++) {
      callback(input[i])
    }
  }, true)
}

function _callerFileName (stackOffset) {
  stackOffset = stackOffset || 4
  let traceElemBits = (new Error()).stack.split('\n')[stackOffset].split(/[\\:.]/g)
  return traceElemBits[traceElemBits.length - 4]
}

function pause(delay) {
  childProcess.spawnSync('sleep', [(delay !== undefined) ? (delay / 1000) : 0.1])
}

function clearScreen (delay) {
  pause(delay)
  process.stdout.write('\u001b[0J\u001b[1J\u001b[2J\u001b[0;0H\u001b[0;0W')
}

function createArray (length, value) {
  return new Array(length).fill(value)
}

function createGrid (w, h, value) {
  let grid = []
  for (let i = 0; i < w; i++) {
    grid[i] = createArray(h, value)
  }
  return grid
}

function sumArray (array) {
  let total = 0
  array.forEach(function (cell) {
    total += cell
  })
  return total
}

function sumGrid (grid) {
  let total = 0
  grid.forEach(function (line) {
    total += sumArray(line)
  })
  return total
}

function countArray (array, value) {
  let total = 0
  array.forEach(function (cell) {
    if (cell === value) {
      total++
    }
  })
  return total
}

function countGrid (grid, value) {
  let total = 0
  grid.forEach(function (line) {
    total += countArray(line, value)
  })
  return total
}

function fillGrid (grid, value) {
  for (let i = 0; i < grid.length; i++) {
    for (let j = 0; j < grid[0].length; j++) {
      grid[i][j] = value
    }
  }
  return grid
}

function printGrid(grid) {
  for (let j = 0; j < grid[0].length; j++) {
    let line = ''
    for (let i = 0; i < grid.length; i++) {
      line += grid[i][j]
    }
    console.log(line)
  }
}

function sortObjectByKeys (obj) {
  // http://stackoverflow.com/a/29622653
  return Object.keys(obj).sort().reduce(function (result, key) {
    result[key] = obj[key]
    return result
  }, {})
}

function setCharAt (string, i, c) {
  return string.slice(0, i) + c + string.slice(i + 1)
}

function each(o, callback) {
  for (let key in o) {
    let value = o[key]
    callback(key, value)
  }
}