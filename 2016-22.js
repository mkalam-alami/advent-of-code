'use strict'
let u = require('./utils')

// pt. 1

const W = 30
const H = 35

let nodeGrid = u.createGrid(W, H)
let nodeList = []

u.readInputAsLines(function (line) {
  let [, x, y, size, used, avail] = line.split(/[^0-9]+/g).map(s => parseInt(s))
  nodeGrid[x][y] = {x, y, size, used, avail, goal: (x === W - 1 && y === 0)}
  nodeList.push(nodeGrid[x][y])
})

let viablePairs = []
nodeList.forEach(function (a) {
  nodeList.forEach(function (b) {
    if (a !== b && a.used > 0 && a.used <= b.avail) viablePairs.push({a, b})
  })
})

// pt. 2

let emptyNode = nodeGrid[28][34]
let turn = 0

function repeat (n, f) {
  for (let i = 0; i < n; i++) f()
}

function move (dx, dy, n = 1) {
  repeat(n, function () {
    let nextNode = nodeGrid[emptyNode.x + dx][emptyNode.y + dy]
    if (emptyNode.used <= nextNode.size && nextNode.used <= emptyNode.size) {
      ;[emptyNode.used, nextNode.used] = [nextNode.used, emptyNode.used]
      ;[emptyNode.goal, nextNode.goal] = [nextNode.goal, emptyNode.goal]
    } else {
      throw new Error(`can't go from ${JSON.stringify(emptyNode)} to ${JSON.stringify(nextNode)}`)
    }
    emptyNode = nextNode
    turn++
    printGrid()
    if (nodeGrid[0][0].goal) process.exit()
  })
}

printGrid()
move(0, -1, 13)
move(-1, 0, 5)
move(0, -1, 2)
move(1, 0, 5)
move(0, -1, 19)
repeat(999, function () {
  move(1, 0)
  move(0, 1)
  move(-1, 0, 2)
  move(0, -1)
})

// Output

function printGrid () {
  const PAD = 4
  const spaces = n => new Array(n).fill(' ').join('')
  const leftPad = str => str + spaces(PAD - str.length)
  const centerPad = str => spaces(Math.ceil((PAD - str.length) / 2)) + str + spaces(Math.floor((PAD - str.length) / 2))

  u.clearScreen()

  let header = leftPad(`t${turn}`)
  for (let i = 0; i < W; i++) {
    header += centerPad(`x${i}`)
  }
  console.log(header)

  for (let j = 0; j < H; j++) {
    let line = leftPad(`y${j}`)
    for (let i = 0; i < W; i++) {
      let node = nodeGrid[i][j]
      if (node.used === 0) line += centerPad('O')
      else if (node.size > 100) line += centerPad('=')
      else if (node.goal) line += centerPad('$')
      else line += centerPad('.')
      // line += centerPad(`${Math.floor(node.used/10)}/${Math.floor(node.size/10)}${node.used === 0?'!': ''}${node.goal?'$': ''}`)
    }
    console.log(line)
  }
}

printGrid()
