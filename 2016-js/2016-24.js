'use strict'

let u = require('./utils')

//let map = u.createGrid(11, 5)
let map = u.createGrid(177, 45)
let goals = []

let j = 0
u.readInputAsLines(function (line) {
  for (var i = 0; i < line.length; i++) {
    map[i][j] = line[i]
    if (!isNaN(parseInt(line[i]))) {
      goals[parseInt(line[i])] = {i:i,j:j}
    }
  }
  j++
})

console.log("Goals locations:")
console.log(goals)

const DIRECTIONS = [{i:1,j:0}, {i:-1,j:0}, {i:0,j:1}, {i:0,j:-1}]
let goalsGraph = u.createGrid(goals.length, goals.length)

for (let start = 0; start < goalsGraph.length; start++) {
  for (let end = 0; end <= start; end++) {
    goalsGraph[end][start] = goalsGraph[start][end] = shortestPathLength(goals[start], goals[end])
  }
}

function shortestPathLength (start, end) {
  let next = [start]
  let visited = [start]
  let finalNode = (start.i === end.i && start.j === end.j) ? start : null
  while (finalNode === null && next.length > 0) {
    let newNext = []
    next.forEach(function (node) {
      DIRECTIONS.forEach(function (direction) {
        let candidate = {i: node.i + direction.i, j: node.j + direction.j, parent: node}
        if (areCoordsValid(candidate) && map[candidate.i][candidate.j] !== '#'
            && !nodeListContains(visited, candidate)) {
          visited.push(candidate)
          newNext.push(candidate)
          if (candidate.i === end.i && candidate.j === end.j) {
            finalNode = candidate
          }
        }
      })
    })
    next = newNext
  }
  
  if (finalNode !== null) {
    let node = finalNode
    let length = 0
    while (node.parent) {
      length++
      node = node.parent
    }
    return length
  } else {
    return false
  }
}

function areCoordsValid (node) {
 return node.i >= 0 && node.i < map.length && node.j >= 0 && node.j < map[0].length 
}

function nodeListContains (list, node) {
  for (let listNode of list) {
    if (listNode.i === node.i && listNode.j === node.j) {
      return true
    }
  }
  return false
}


console.log("\nCost matrix:")
console.log(goalsGraph)


let pt2 = true
let nodes = [{current:0, visited:[0], totalCost: 0}]
let best = null
while (nodes.length > 0) {
  let bestIndex = 0
  for (let i = 1; i < nodes.length; i++) {
    if (nodes[i].totalCost < nodes[bestIndex].totalCost) {
      bestIndex = i
    }
  }
  
  let visiting = nodes.splice(bestIndex, 1)[0]
  if (visiting.visited.length === goals.length + (pt2 ? 1 : 0)) {
    best = visiting
    break
  }
  
  for (let goalIndex = 0; goalIndex < goals.length; goalIndex++) {
    if (visiting.visited.indexOf(goalIndex) === -1
        || pt2 && visiting.visited.length === goals.length && goalIndex === 0) {
      let newVisited = visiting.visited.slice(0)
      newVisited.push(goalIndex)
      nodes.push({
        current: goalIndex,
        visited: newVisited,
        totalCost: visiting.totalCost + goalsGraph[visiting.current][goalIndex]
      })
    }
  }
}

console.log("\nBest path:")
console.log(best)