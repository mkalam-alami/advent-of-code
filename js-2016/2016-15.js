'use strict'

let u = require('../utils')

/* let input = `Disc #1 has 5 positions; at time=0, it is at position 4.
			 Disc #2 has 2 positions; at time=0, it is at position 1.` */

let input = `Disc #1 has 5 positions; at time=0, it is at position 2.
			 Disc #2 has 13 positions; at time=0, it is at position 7.
			 Disc #3 has 17 positions; at time=0, it is at position 10.
			 Disc #4 has 3 positions; at time=0, it is at position 2.
			 Disc #5 has 19 positions; at time=0, it is at position 9.
			 Disc #6 has 7 positions; at time=0, it is at position 0.`

input += '\nDisc #7 has 11 positions; at time=0, it is at position 0.' // pt. 2

input = input.split('\n').map(function (s) { return s.trim() })

let discs = []
input.forEach(function (line) {
  let [, id,, count,,,,,,,, init] = line.split(/[ #]+/g).map(function (s) { return parseInt(s) })
  discs.push({id, count, init})
})

let t = 0
while (true) {
  if (t % 10000 === 0) printDiscs()
  let aligned = true
  for (let i = 0; i < discs.length; i++) {
    aligned = isAligned(discs[i], t)
    if (!aligned) break
  }
  if (aligned) {
    printDiscs()
    console.log('capsule get!')
    break
  }
  t++
}

function printDiscs () {
  u.clearScreen(10)
  console.log(`time=${t}`)
  for (let i = 0; i < discs.length; i++) {
    console.log(`${discs[i].id}: ${isAligned(discs[i], t)}`)
  }
}

function isAligned (disc, t) {
  return (t + disc.init + disc.id) % disc.count === 0
}
