var utils = require('./utils')

var grid = utils.createGrid(1000, 1000, false)
var grid2 = utils.createGrid(1000, 1000, false)

// utils.mockInput('turn on 0,0 through 3,3')

utils.readInputAsLines(function (line) {
	// e.g. "turn on 957,736 through 977,890"
  line = line.replace('toggle', 'x toggle')
  var splitLine = line.split(/[ ,]/g)
  var instruction = splitLine[1]
  var from = {x: parseInt(splitLine[2]), y: parseInt(splitLine[3])}
  var to = {x: parseInt(splitLine[5]), y: parseInt(splitLine[6])}

  for (var i = from.x; i <= to.x; i++) {
    for (var j = from.y; j <= to.y; j++) {
      switch (instruction) {
        case 'on': grid[i][j] = true; grid2[i][j]++; break
        case 'off': grid[i][j] = false; if (grid2[i][j] > 0) grid2[i][j]--; break
        case 'toggle': grid[i][j] = !grid[i][j]; grid2[i][j] += 2
      }
    }
  }
})

console.log(utils.countGrid(grid, true), utils.sumGrid(grid2))
