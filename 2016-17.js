'use strict'
let u = require('./utils')
let md5 = require('js-md5')

const DIRECTIONS = [{i:0,j:-1,c:'U'}, {i:0,j:1,c:'D'}, {i:-1,j:0,c:'L'}, {i:1,j:0,c:'R'}]
const OPEN_DOORS = Array.from('bcdef')

run(4, 'ihgpwlah', false)
run(4, 'ihgpwlah', true)
run(4, 'kglvqrro', false)
run(4, 'kglvqrro', true)
run(4, 'ulqzkmiv', false)
run(4, 'ulqzkmiv', true)
run(4, 'vwbaicqe', false)
run(4, 'vwbaicqe', true)

function run(roomSize, input, longest) {
  let it = 0
  let goal = {i:roomSize-1, j: roomSize-1}
  let nodes = [{i:0,j:0,path:'',visited:[0]}]
  setNodeScore(nodes[0], goal)
  let result = null
  let longestPathLength = null
  
  while (result === null && nodes.length > 0) {
    /*if (longest && it++ % 10000 === 0) {
      console.log(it, nodes.length, longestPathLength)
    }*/
    
    let bestNode = nodes[0]
    for (let node of nodes) {
      if (node.score < bestNode.score) bestNode = node
    }
    nodes.splice(nodes.indexOf(bestNode), 1)
    if (bestNode.i === goal.i && bestNode.j === goal.j && !longest) {
      result = bestNode
    }
    
    let doorsHash = md5(input + bestNode.path)
    for (let directionIndex in DIRECTIONS) {
      if (OPEN_DOORS.indexOf(doorsHash.charAt(directionIndex)) !== -1) {
        let direction = DIRECTIONS[directionIndex]
        let candidate = {i: bestNode.i + direction.i, j: bestNode.j + direction.j}
        if (candidate.i >= 0 && candidate.i < roomSize && candidate.j >= 0 && candidate.j < roomSize) {
          if (!longest || candidate.i !== goal.i || candidate.j !== goal.j) {
            let candidateHash = candidate.j * roomSize + candidate.i
            candidate.path = bestNode.path + direction.c
            candidate.visited = bestNode.visited.slice(0)
            candidate.visited.push(candidateHash)
            setNodeScore(candidate, goal, longest)
            nodes.push(candidate)
          } else if (longest) {
            longestPathLength = Math.max(longestPathLength, bestNode.path.length + 1)
          }
        }
      }
    }
  }
  
  if (longest) {
    if (longestPathLength === null) longestPathLength = -1
    console.log(`${input}: longest is ${longestPathLength} long`)
  } else {
    if (result === null) result = {path: 'not found!'}
    console.log(`${input}: ${result.path}`)
  }
}

function setNodeScore(node, goal, longest) {
  node.score = node.path.length + Math.abs(goal.i - node.i) + Math.abs(goal.j - node.j)
}