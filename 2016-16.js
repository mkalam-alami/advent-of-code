'use strict'
let u = require('./utils')

let str = '11100010111110100'

function fill(size, str) {
	while (str.length < size) {
		str = str + '0' + dragonify(str)
	}
	return str.slice(0, size)
}

function dragonify(str) {
	let array = Array.from(str).reverse()
	let result = ''
	for (let c of array) {
		result += c == '1' ? '0' : '1'
	}
	return result
}

function checksum(str) {
	while (str.length % 2 === 0) {
		let nextStr = ''
		for (let i = 0; i < str.length; i += 2) {
			nextStr += (str[i] === str[i+1]) ? '1' : '0'
		}
		str = nextStr
	}
	return str
}

console.log('--- tests')
console.log(fill(3, '1'))
console.log(fill(3, '0'))
console.log(fill(11, '11111'))
console.log(fill(25, '111100001010'))
console.log(checksum('110010110100'))
console.log(checksum(fill(20, '10000')))

console.log('--- pt. 1')
console.log(checksum(fill(272, '11100010111110100')))
console.log('--- pt. 2')
console.log(checksum(fill(35651584, '11100010111110100')))