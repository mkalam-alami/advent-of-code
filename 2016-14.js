let u = require('./utils')
let md5 = require('js-md5')

let hashes = []
let keysFound = 0
let i = 0
let result = null

while (result === null) {
	let hash = md5('cuanljph' + i++) // cuanljph
	hashes.push(hash)
	
	if (i >= 1000) {
		let candidateIndex = i - 1000
		let candidate = hashes[candidateIndex]
		let token = findRepeating(candidate, 3)
		if (token) {
			for (let j = candidateIndex + 1; j <= candidateIndex + 1000; j++) {
				if (findRepeating(hashes[j], 5, token)) {
					keysFound++
					if (keysFound === 64) {
						result = candidateIndex
						break
					}
				}
			}
		}
		
	}
}

function findRepeating(str, size, token) {
	let foundToken = null
	let foundCount
	for (let c of Array.from(str)) {
		if (c === foundToken) {
			if (++foundCount === size) return foundToken
		} else if (token === undefined || token === c) {
			foundToken = c
			foundCount = 1
		}
	}
	return false
}

console.log(result)