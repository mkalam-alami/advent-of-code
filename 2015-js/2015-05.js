var utils = require('./utils')

var total = 0, totalPt2 = 0

utils.readInputAsLines(function (line) {
	// PART 1
  var vowels = line.match(/[aeiou]/g)
  if (vowels && vowels.length >= 3) {
    var c = null, ok = false
    for (var i = 0; i < line.length; i++) {
      if (line[i] == c) {
        ok = true
        break
      } else {
        c = line[i]
      }
    }

    if (ok) {
      ['ab', 'cd', 'pq', 'xy'].forEach(function (bad) {
        if (ok && line.indexOf(bad) != -1) {
          ok = false
        }
      })

      if (ok) {
        total++
        console.log(line + ': YES')
      } else {
        console.log(line + ': NO (blacklist)')
      }
    } else {
      console.log(line + ': NO (doubles)')
    }
  } else {
    console.log(line + ': NO (vowels)')
  }

	// PART 2
  if (twoLetters(line) && repeatsWithGap(line)) {
    totalPt2++
  }
})

function twoLetters (line) {
  var found = false, token
  for (var i = 0; i < line.length - 1; i++) {
    token = line.slice(i, i + 2)
    if (line.indexOf(token, i + 2) != -1) {
      found = true
      break
    }
  }
  return found
}

function repeatsWithGap (line) {
  var found = false, token
  for (var i = 0; i < line.length - 2; i++) {
    token = line.charAt(i)
    if (line.charAt(i + 2) == token) {
      found = true
      break
    }
  }
  return found
}

console.log('TOTALS: ', total, totalPt2)
