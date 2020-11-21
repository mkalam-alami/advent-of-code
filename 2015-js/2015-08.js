var u = require('./utils')

var total = 0
var total2 = 0

u.readInputAsLines(function (line) {
  mem = line.slice(1, line.length - 1)
  total += line.length - eval(line).length
  
  let subtotal2 = 2
  for (let c of line) {
	  if (c === '"' || c === '\\') subtotal2++
  }
  console.log(line + " => " + subtotal2)
  total2 += subtotal2
})

console.log("--------------")
console.log(total)
console.log(total2)
