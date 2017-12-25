'use strict'

let u = require('./utils')

let result = 0

let nodes = {}

u.readInputAsLines(function (line) {
  let [ name, weight, info ] = line.split(/[\(\)]/g)
  name = name.trim()
  let children = []
  if (info) {
    children = info.replace(/ -> /g, '').split(', ')
  }
  nodes[name] = {
    name,
    weight: parseInt(weight),
    children,
    parents: []
  }
})

u.each(nodes, (name, node) => {
  for (let child of node.children) {
    nodes[child].parents.push(name)
  }
})

// Part 1

let root = null
u.each(nodes, (name, node) => {
  if (node.parents.length === 0) {
    if (root === null) root = node
    else console.error('ERROR')
  }
})

console.log("Root: " + root.name)

// Part 2

function totalWeight(name) {
  let node = nodes[name]
  let result = node.weight
  for (let child of nodes[name].children) {
    result += totalWeight(child)
  }
  return result
}

function verify(name) {
  let weights = []
  let invalid = false
  for (let child of nodes[name].children) {
    let newWeight = totalWeight(child)
    if (weights.length > 0 && !weights.includes(newWeight)) {
      invalid = true
    }
    weights.push(newWeight)
  }
  if (invalid) {
    console.log(nodes[name], weights)
  }
}

for (let name in nodes) {
  verify(name)
}

console.log(nodes['eionkb'])