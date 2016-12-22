var u = require('./utils')

var grid1 = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
var grid2 = [[0, 0, 1, 0, 0], [0, 2, 3, 4, 0], [5, 6, 7, 8, 9], [0, 'A', 'B', 'C', 0], [0, 0, 'D', 0, 0]]
var pos1 = {i: 1, j: 1}, result1 = '', pos2 = {i: 0, j: 2}, result2 = ''

function safeMove (pos, di, dj, grid) {
  var newI = Math.min(grid.length - 1, Math.max(pos.i + di, 0))
  var newJ = Math.min(grid.length - 1, Math.max(pos.j + dj, 0))
  return (grid[newJ][newI] !== 0) ? {i: newI, j: newJ} : pos
}

u.readInputAsLines(function (line) {
  for (var n = 0; n < line.length; n++) {
    switch (line[n]) {
      case 'L': pos1 = safeMove(pos1, -1, 0, grid1); pos2 = safeMove(pos2, -1, 0, grid2); break
      case 'R': pos1 = safeMove(pos1, 1, 0, grid1); pos2 = safeMove(pos2, 1, 0, grid2); break
      case 'U': pos1 = safeMove(pos1, 0, -1, grid1); pos2 = safeMove(pos2, 0, -1, grid2); break
      case 'D': pos1 = safeMove(pos1, 0, 1, grid1); pos2 = safeMove(pos2, 0, 1, grid2); break
    }
  }
  result1 = result1 + grid1[pos1.j][pos1.i]
  result2 = result2 + grid2[pos2.j][pos2.i]
})

console.log(result1, result2)
