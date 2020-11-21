'use strict'

let u = require('./utils')

let registers
let program = []
let pointer = 0, iteration = 0
u.readInputAsLines(function (line) {
  let [instr, i1, i2] = line.split(' ')
  program.push({
    line: line,
    instr: instr,
    i1: i1,
    i2: i2,
    i1numeric: !isNaN(i1),
    i2numeric: !isNaN(i2)
  })
})

let success = false
let i = 0
while (!success) {
  let out = run(i)
  console.log(i++, out.join(''))
  
  let expected = 0
  success = true
  for (let c of out) {
    if (c === expected) {
      expected = 1 - expected
    } else {
      success = false
      break
    }
  }
}

function run (initialA) {
  registers = {a: initialA, b: 0, c: 0, d: 0}

  pointer = 0
  iteration = 0
  let out = []
  while (pointer < program.length && iteration++ < 100000) {
   // printProgram()
    let lineInfo = program[pointer]
    let [instr, i1, i2] = [lineInfo.instr, lineInfo.i1, lineInfo.i2]
    switch (instr) {
      case 'cpy':
        if (!lineInfo.i2numeric) {
          if (lineInfo.i1numeric) registers[i2] = parseInt(i1)
          else registers[i2] = registers[i1]
        }
        pointer++
        break
      case 'mov':
        registers[i2] += registers[i1]
        registers[i1] = 0
        pointer++
        break
      case 'mul':
        registers[i2] *= registers[i1]
        registers[i1] = 0
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
      case 'ign':
        pointer++
        break
      case 'jnz':
        if (!lineInfo.i1numeric && registers[i1] !== 0 || lineInfo.i1numeric && parseInt(i1) !== 0) {
          if (lineInfo.i2numeric) {
            pointer += parseInt(i2)
          } else {
            pointer += registers[i2]
          }
        } else {
          pointer++
        }
        break
      case 'tgl':
        let instrIndex = pointer + ((lineInfo.i1numeric) ? parseInt(i1) : registers[i1])
        if (instrIndex < program.length) toggleInstruction(program[instrIndex])
        pointer++
        break
      case 'out':
        if (lineInfo.i1numeric) out.push(parseInt(i1))
        else out.push(registers[i1])
        pointer++
        break
    }
  }
  
  return out
}

function toggleInstruction(instruction) {
  switch (instruction.instr) {
    case 'tgl':
    case 'dec':
    case 'out':
      instruction.line = instruction.line.replace(/(tgl|dec|out)/g, 'inc')
      instruction.instr = 'inc'
      break
    case 'inc':
      instruction.line = instruction.line.replace('inc', 'dec')
      instruction.instr = 'dec'
      break
    case 'cpy':
      instruction.line = instruction.line.replace('cpy', 'jnz')
      instruction.instr = 'jnz'
      break
    case 'jnz':
      instruction.line = instruction.line.replace('jnz', 'cpy')
      instruction.instr = 'cpy'
      break
    default:
      throw new Error(JSON.stringify(instruction))
  }
}

function printProgram (force) {
  if (force || iteration % 10000 === 0) {
    u.clearScreen(100)
    console.log('t=' + iteration, 'registers=' + JSON.stringify(registers))
    console.log('----------------------------------------')
    program.forEach(function (lineInfo, i) {
      if (i == pointer) console.log(' > ' + lineInfo.line)
      else console.log('   ' + lineInfo.line)
    })
  }
}
