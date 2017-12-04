'use strict'

// Source: https://twitter.com/_sorceress/status/935256907408867328

let u = require('./utils')

let valid = 0
let valid2 = 0

u.readInputAsLines(function (line) {
  let words = line.split(' ')

  // 1st star
  let isValid = true
  for (let i = 0; i < words.length; i++) {
    if (words.indexOf(words[i]) !== i) {
      isValid = false
      break 
    }
  }
  if (isValid) valid++

  // 2nd star
  let sortedWords = []
  for (let word of words) {
    sortedWords.push(word.split("").sort().join(""))
  }
  isValid = true
  for (let i = 0; i < sortedWords.length; i++) {
    if (sortedWords.indexOf(sortedWords[i]) !== i) {
      isValid = false
      break 
    }
  }
  if (isValid) valid2++
})

console.log("Valid lines:", valid, ",", valid2)