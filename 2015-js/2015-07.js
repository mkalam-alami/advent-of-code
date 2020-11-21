var u = require('./utils')

var mem = {}

// u.mockInput("123 -> x\nx -> y")

// u.mockInput("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i");

u.readInputAsLines(function (line) {
  var s = line.split(' ')
  if (s.length == 3 && s[0].match(/[a-z]/g) == null) {
    mem[s[2]] = parseInt(s[0])
  } else {
    var reqs = []
    switch (s.length) {
      case 3: reqs.push(s[0]); break
      case 4: reqs.push(s[1]); break
      case 5:
        if (s[0].match(/[a-z]/g)) reqs.push(s[0])
        if (['AND', 'OR'].indexOf(s[1]) != -1 && s[2].match(/[a-z]/g)) reqs.push(s[2])
        break
      default: throw new Error(s.length)
    }
    mem[s[s.length - 1]] = {
      reqs: reqs,
      line: line
    }
  }
})

// while (typeof())

mem = u.sortObjectByKeys(mem)

var newReady
while (typeof mem['a'] !== 'number') {
  newReady = 0
  for (var key in mem) {
    if (typeof mem[key] !== 'number') {
      var o = mem[key]
      var ready = true
      o.reqs.forEach(function (req) {
        if (typeof mem[req] !== 'number') {
          ready = false
        }
      })
      if (ready) {
        console.log('Resolving ' + o.line)
        newReady++
        mem[key] = resolveLine(o.line)
      }
    }
  }
  if (newReady == 0) {
    u.exit('Stuck!')
  }
  u.l(newReady + ' resolved, next step...')
}

function resolveLine (line) {
  var s = line.split(' ')
  if (s.length == 3) {
		// set var
    return parseInt(mem[s[0]])
  } else if (s.length == 4) {
		// not
    return (~mem[s[1]]) & 0xFFFF
  } else {
    var first = (s[0].match(/[a-z]/g) == null) ? parseInt(s[0]) : mem[s[0]]
    var second = (s[2].match(/[a-z]/g) == null) ? parseInt(s[2]) : mem[s[2]]
    switch (s[1]) {
      case 'AND': return first & second
      case 'OR': return first | second
      case 'LSHIFT': return first << second
      case 'RSHIFT': return first >> second
      default: throw new Error(s[1])
    }
  }
}

console.log(mem['a'])
