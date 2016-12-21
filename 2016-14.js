let u = require('./utils')
let md5 = require('js-md5')

let hashes = []
let keysFound = 0
let i = 0
let result = null

let stretchedMode = true // pt. 2

while (result === null) {
	let hash = md5('cuanljph' + i++)
	if (stretchedMode) {
		for (let i = 0; i < 2016; i++) {
			hash = md5(hash)
		}
	}
	
	hashes.push(hash)
	
	if (i > 1000) {
		let candidateIndex = i - 1001
		let candidate = hashes[candidateIndex]
		let token = findRepeating(candidate, 3)
		if (token) {
			for (let j = candidateIndex + 1; j <= candidateIndex + 1000; j++) {
				if (findRepeating(hashes[j], 5, token)) {
					keysFound++
					console.log(`key ${keysFound} is ${candidateIndex}: ${candidate} with token ${token}`)
					if (keysFound === 64) {
						result = candidateIndex
					}
					break
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
			if (++foundCount === size) {
				return foundToken
			}
		} else {
			foundToken = (token === undefined || token === c) ? c : null
			foundCount = 1
		}
	}
	return false
}

console.log(result)