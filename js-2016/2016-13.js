'use strict'

let u = require('../utils')

const SIZE = 51
const GOAL_X = 31, GOAL_Y = 39
const GOAL_H = h(GOAL_X, GOAL_Y)
const LOCATIONS_TEST = 50

let visitedLocations = [h(1, 1)]
let visitedWalls = []
let next = [{x: 1, y: 1}]
let goal = null
let turns = 1

const NEIGHBOR_OFFSETS = [{x: 1, y: 0}, {x: -1, y: 0}, {x: 0, y: 1}, {x: 0, y: -1}]

while (goal === null && next.length > 0) {
  let current = next
  next = []
  current.forEach(function (node) {
    for (var offset of NEIGHBOR_OFFSETS) {
      var neighbor = {x: node.x + offset.x, y: node.y + offset.y, parent: node}
      var neighborHash = h(neighbor.x, neighbor.y)
      if (neighbor.x >= 0 && neighbor.x <= SIZE
					&& neighbor.y >= 0 && neighbor.y <= SIZE
					&& visitedLocations.indexOf(neighborHash) === -1
					&& visitedWalls.indexOf(neighborHash) === -1) {
        if (neighborHash === GOAL_H) {
          goal = neighbor
          break
        } else if (!isWall(neighbor.x, neighbor.y)) {
          next.push(neighbor)
          visitedLocations.push(neighborHash)
        } else {
          visitedWalls.push(neighborHash)
        }
      }
    }
  })
  if (turns === LOCATIONS_TEST) {
    console.log(visitedLocations)
    console.log('Visited in ' + LOCATIONS_TEST + ' turns: ' + visitedLocations.length)
  }
  turns++
}

let path = []
if (goal !== null) {
  while (goal.parent) {
    path.push(h(goal.x, goal.y))
    goal = goal.parent
  }
  console.log('Goal reached in: ' + path.length)
}

printRoom()

function printRoom () {
  for (let y = 0; y < SIZE; y++) {
    var line = ''
    for (let x = 0; x < SIZE; x++) {
      if (x === GOAL_X && y === GOAL_Y) line += '¤'
      else if (path.indexOf(h(x, y)) !== -1) line += '.'
      else line += isWall(x, y) ? 'O' : ' '
    }
    console.log(line)
  }
}

function isWall (x, y) {
  let binary = (x * x + 3 * x + 2 * x * y + y + y * y + 1358).toString(2)
  let ones = 0
  for (let c of binary) {
    if (c === '1') ones++
  }
  return ones % 2 == 1
}

function h (x, y) {
  return 'x' + x + 'y' + y
}
