'use strict'

let u = require('./utils')

let registers = {a: 0, b: 0, c: 1, d: 0}
let program = []
u.readInputAsLines(function (line) {
	let [instr, i1, i2] = line.split(' ')
	program.push({
		line: line,
		instr: instr,
		i1: i1,
		i2: i2,
		i1numeric: !isNaN(i1)
	})
})

let pointer = 0, iteration = 0
while (pointer < program.length - 1) {
	printProgram(false);
	let lineInfo = program[pointer];
	let [instr, i1, i2] = [lineInfo.instr, lineInfo.i1, lineInfo.i2]
	switch (instr) {
		case 'cpy':
			if (lineInfo.i1numeric) registers[i2] = parseInt(i1)
			else registers[i2] = registers[i1]
			pointer++
			break
		case 'inc':
			registers[i1]++
			pointer++
			break
		case 'dec':
			registers[i1]--
			pointer++
			break
		case 'jnz':
			if (registers[i1] !== 0) pointer += parseInt(i2)
			else pointer++
	}
}

function printProgram(force) {
	if (force || iteration++ % 100000 === 0) {
		u.clearScreen(0)
		console.log(registers)
		console.log("--------")
		program.forEach(function(lineInfo, i) {
			if (i == pointer) console.log(" > " + lineInfo.line)
			else console.log("   " + lineInfo.line)
		})
	}
}

printProgram(true)