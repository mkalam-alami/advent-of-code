var u = require('./utils')

u.readInput(function (line) {
  line = line.replace(/[ \r\n]/g, '')
  part1(line)
  part2(line)
})

function part1 (line) {
  var result = 0
  var c
  for (var i = 0; i < line.length; i++) {
    c = line[i]
    if (c == '(') {
      result += expand(line, i).length
      i = getMarkerEndIndex(line, i) + getMarkerInfo(line, i).size
    } else {
      result++
    }
  }
  console.log(result)
}

function part2 (line) {
  console.log(walk(line, 1))
}

function walk (string, multiplier) {
  var result = 0
  for (var i = 0; i < string.length; i++) {
    c = string[i]
    if (c == '(') {
      var markerEndIndex = getMarkerEndIndex(string, i)
      var markerInfo = getMarkerInfo(string, i)
      var newMultiplier = multiplier * markerInfo.multiplier
      result += walk(string.slice(markerEndIndex + 1, markerEndIndex + 1 + markerInfo.size), newMultiplier)
      i = markerEndIndex + markerInfo.size
    } else {
      result += multiplier
    }
  }
  return result
}

function getMarkerEndIndex (line, i) {
  return line.indexOf(')', i)
}

function expand (line, i) {
  var markerEndIndex = getMarkerEndIndex(line, i)
  var markerInfo = getMarkerInfo(line, i)
  var data = line.slice(markerEndIndex + 1, markerEndIndex + 1 + markerInfo.size)
  var dupedString = ''
  for (var n = 0; n < markerInfo.multiplier; n++) {
    dupedString += data
  }
  return dupedString
}

function getMarkerInfo (line, i) {
  var markerString = line.slice(i + 1, getMarkerEndIndex(line, i))
  var markerArray = markerString.split('x')
  return {
    size: parseInt(markerArray[0]),
    multiplier: parseInt(markerArray[1])
  }
}
