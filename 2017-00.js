'use strict'

// Source: https://twitter.com/_sorceress/status/935256907408867328

let u = require('./utils')

let furthest = 0
let furthestPair = 0

u.readInput(function (line) {
	let instrs = line.split(', ')
	let map = []
	let i = 0
	let j = 0
	let as = []
	let bs = []
	
	let sources = null
	let targets = null
	
	for (let instr of instrs) {
		switch (instr) {
		case 'A':
			sources = as
			targets = bs
		case 'B':
			if (!map[i]) map[i] = []
			map[i][j] = instr

			furthest = Math.max(furthest, Math.abs(i) + Math.abs(j))
			
			sources = sources || bs
			targets = targets || as
			sources.push({i,j})
			for (let target of targets) {
				furthestPair = Math.max(furthestPair, Math.abs(i - target.i) + Math.abs(j - target.j))
			}
			sources = targets = null
			break
		case 'Up':
			j--
			break
		case 'Down':
			j++
			break
		case 'Left':
			i--
			break
		case 'Right':
			i++
			break
		}
	}
})

console.log("Furthest: " + furthest)
console.log("Furthest pair: " + furthestPair)