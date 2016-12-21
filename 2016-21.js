'use strict'
let u = require('./utils')

// example input
// u.mockInput('swap position 4 with position 0\nswap letter d with letter b\nreverse positions 0 through 4\nrotate left 1 step\nmove position 1 to position 4\nmove position 3 to position 0\nrotate based on position of letter b\nrotate based on position of letter d')

let ops = []
u.readInputAsLines(function (line) {
	ops.push(line)
})

console.log(run('abcdefgh', false)) // pt. 1 
console.log(run('fbgdceah', true)) // pt. 2

function run(password, reversed) {
	if (reversed) {
		for (let i = ops.length - 1; i >= 0; i--) {
			password = apply(ops[i], password, true)
		}
	} else {
		for (let i = 0; i < ops.length; i++) {
			password = apply(ops[i], password, false)
		}
	}
	return password
}

function apply(line, password, reversed) {
	if (line.indexOf('swap') === 0) {
		let [,swapType,i1,,,i2] = line.split(' ')
		if (swapType === 'position') {
			// swap position
			i1 = parseInt(i1), i2 = parseInt(i2)
			let buffer = password.charAt(i1)
			password = u.setCharAt(password, i1, password.charAt(i2))
			password = u.setCharAt(password, i2, buffer)
		} else {
			// swap letter
			password = password.replace(new RegExp(i1, 'g'), '?')
			password = password.replace(new RegExp(i2, 'g'), i1)
			password = password.replace(new RegExp('\\?', 'g'), i2)
		}
	} else if (line.indexOf('rotate') === 0) {
		let amount
		if (line.indexOf('rotate based') === 0) {
			// rotate based
			let c = line.split(' ')[6]
			amount = password.indexOf(c)
			if (amount >= 4) amount++
			amount++
		} else {
			// rotate left/right
			let [,rotateType,n] = line.split(' ')
			amount = parseInt(n) * ((rotateType == 'left') ? -1 : 1)
		}
		if (reversed) amount *= -1
		let passwordLength = password.length
		password = password.repeat(5)
		password = password.slice(passwordLength * 2 - amount, passwordLength * 3 - amount)
	} else if (line.indexOf('reverse') === 0) {
		// reverse
		let [,,from,,to] = line.split(' ').map(function(s) { return parseInt(s) })
		password = password.slice(0, from) + Array.from(password.slice(from, to + 1)).reverse().join('') + password.slice(to + 1)
	} else {
		// move
		let [,,from,,,to] = line.split(' ').map(function(s) { return parseInt(s) })
		let array = Array.from(password)
		let c = array.splice(from, 1)
		password = array.join('')
		password = password.slice(0, to) + c + password.slice(to)
	}
	//console.log(line + '  -->  ' + password)
	return password
}
