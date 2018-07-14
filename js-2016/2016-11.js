'use strict'

// WORK IN PROGRESS

let u = require('../utils')

// Floors setup

let floors = {
  1: [ gen('pol'), gen('thu'), chip('thu'), gen('pro'), gen('rut'), chip('rut'), gen('cob'), chip('cob') ],
  2: [ chip('pol'), chip('pro') ],
  3: [ ],
  4: [ ]
}

function chip(mat) {
  return {type: 'chip', mat: mat}
}
function gen(mat) {
  return {type: 'gen', mat: mat}
}

// Logic

let currentFloor = 1
let currentGoal = {task: null /*grab|drop*/, location: 0, item: null}
let currentItems = []
let step = 0

while (floors[4].length < 10) {
  printFloors()

  step++;
}

function nextGoal() {
  if (currentFloor < 4) {
    if ()
    if (floors[currentFloor].length > 0) {

    }
  }
  else if (currentItems.length == 2) {
    return dropGoal(currentItems[0]);
  }
}

function grabGoal(item) {
  return {task: 'grab', item: item}
}

function dropGoal(item) {
  return {task: 'drop', item: item}
}

// Output

function printFloors() {
  u.clearScreen()
  for (let floor in floors) {
    let str = ((currentFloor === parseInt(floor)) ? ' > ' : '   ') + floor + ' | '
    floors[floor].forEach(function(item) {
      str += '[' + ((item.type === 'chip') ? 'C' : 'G') + '-' + item.mat + '] '
    })
    console.log(str)
  }
}

console.log(step)