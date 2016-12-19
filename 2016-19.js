'use strict'

let u = require('./utils')

const ELVES_COUNT = 20 // 3005290

// Pt. 1

let elves = u.createArray(ELVES_COUNT, 1)
let current, next = -1
while (1) {
	current = nextElf(next)
	next = nextElf(current)
	elves[current] += elves[next]
	elves[next] = 0
	if (elves[current] == ELVES_COUNT) break
}

function nextElf(start) {
	var index = start
	do {
		index = (index + 1) % ELVES_COUNT
	} while (elves[index] === 0)
	return index
}

console.log(current + 1)

// Pt. 2 (using a graph, takes hours to complete!)

if (false) {

	let remainingElves = ELVES_COUNT
	let lastElf = {id: ELVES_COUNT, gifts: 1, next: null}
	let currentElf = lastElf
	for (let i = ELVES_COUNT - 1; i > 0; i--) {
		let newElf = {id: i, gifts: 1, next: currentElf}
		currentElf = newElf
	}
	lastElf.next = currentElf

	let almostStolenElf, stolenElf
	while (currentElf.gifts < ELVES_COUNT) {
		if (remainingElves % 1000 === 0) console.log(remainingElves + " elves remain...")
		almostStolenElf = currentElf
		stolenElf = currentElf.next
		for (let i = 0; i < Math.floor(remainingElves/2) - 1; i++) {
			almostStolenElf = stolenElf
			stolenElf = stolenElf.next
		}
		currentElf.gifts += stolenElf.gifts
		almostStolenElf.next = stolenElf.next
		currentElf = currentElf.next
		remainingElves--
	}

	console.log(currentElf.id)

}

// Pt. 2 (using array + splice, barely twice faster!)

elves = []
for (let i = 1; i <= ELVES_COUNT; i++) {
	elves.push({id: i, gifts: 1})
}

let currentElfIndex = 0, currentElf, stolenElfIndex, stolenElf
while (elves.length > 1) {
	console.log(elves);
	if (elves.length % 1000 === 0) console.log(elves.length + " elves remain...")
	currentElf = elves[currentElfIndex]
	stolenElfIndex = (currentElfIndex + Math.floor(elves.length/2)) % elves.length
	currentElf.gifts += elves[stolenElfIndex].gifts
	elves.splice(stolenElfIndex, 1)
	currentElfIndex = ((stolenElfIndex < currentElfIndex) ? currentElfIndex : currentElfIndex + 1) % elves.length
}

console.log(currentElf.id)
