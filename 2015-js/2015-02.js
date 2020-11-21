var utils = require('./utils')

var wrap = 0
var ribbons = 0

utils.readInputAsLines(function (line) {
  var current, min = -1
  var dims = line.split('x')
  for (var i = 0; i < 3; i++) dims[i] = parseInt(dims[i]);
  [[0, 1], [0, 2], [1, 2]].forEach(function (combination) {
    current = 2 * dims[combination[0]] * dims[combination[1]]
    wrap += current
    if (min == -1 || current < min) {
      min = current
    }
  })
  wrap += min / 2

  ribbons += 2 * (dims[0] + dims[1] + dims[2] - Math.max(dims[0], Math.max(dims[1], dims[2])))
  ribbons += dims[0] * dims[1] * dims[2]
})

console.log(wrap, ribbons)
