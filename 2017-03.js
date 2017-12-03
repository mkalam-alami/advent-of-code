'use strict'

let u = require('./utils')

const INPUT = 361527
const DIRECTIONS = [
  {dx: 1, dy: 0},
  {dx: 0, dy: -1},
  {dx: -1, dy: 0},
  {dx: 0, dy: 1}
]

let checksum = 0
let divisibleSum = 0

function nextDirection(dir) {
  let index = DIRECTIONS.indexOf(dir)
  return DIRECTIONS[(index + 1) % 4]
}

// First star

function spiralDistance(n) {
  let coords = {x: 0, y: 0}
  let direction = DIRECTIONS[0]
  let maxDistance = 1
  let maxDistanceUsedOnce = false

  n--
  while (n > 0) {
    if (n > maxDistance) {
      coords.x += direction.dx * maxDistance
      coords.y += direction.dy * maxDistance
      n -= maxDistance

      direction = nextDirection(direction)
      if (!maxDistanceUsedOnce) {
        maxDistanceUsedOnce = true
      } else {
        maxDistance++
        maxDistanceUsedOnce = false
      }
    } else {
      coords.x += direction.dx * n
      coords.y += direction.dy * n
      break;
    }
  }

  console.log("Spiral distance of " + n + ": " + (Math.abs(coords.x) + Math.abs(coords.y)))
}


spiralDistance(1)
spiralDistance(12)
spiralDistance(23)
spiralDistance(1024)
spiralDistance(INPUT)

console.log("============")

// Second star

let grid = u.createGrid(100, 100, 0)
function get(coords) { return grid[coords.x + 50][coords.y + 50] }
function set(coords, value) { grid[coords.x + 50][coords.y + 50] = value }

function sumNeighbors(coords) {
  let sum = 0
  for (let x = coords.x - 1; x <= coords.x + 1; x++) {
    for (let y = coords.y - 1; y <= coords.y + 1; y++) {
      sum += get({x,y})
    }
  }
  return sum
}

function countNeighbors(coords) {
  let count = 0
  for (let x = coords.x - 1; x <= coords.x + 1; x++) {
    for (let y = coords.y - 1; y <= coords.y + 1; y++) {
      if (get({x,y}) !== 0) count++
    }
  }
  return count
}

let value = 1
let direction = DIRECTIONS[0]
let coords = {x: 0, y: 0}
set(coords, 1)

while (value < INPUT) {
  console.log(value, coords)

  coords.x += direction.dx
  coords.y += direction.dy
  if (countNeighbors(coords) < 3) {
    direction = nextDirection(direction)
  }

  value = sumNeighbors(coords)
  set(coords, value)
}

console.log("First above " + INPUT + ": " + value)