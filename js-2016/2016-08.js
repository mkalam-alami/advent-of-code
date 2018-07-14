var u = require('../utils')

var screen = u.createGrid(50, 6, ' ')

u.readInputAsLines(function (line) {
  var a = line.split(/[ =]/g)
  if (a[0] == 'rect') {
    var coords = a[1].split('x')
    for (var i = 0; i < parseInt(coords[0]); i++) {
      for (var j = 0; j < parseInt(coords[1]); j++) {
        screen[i][j] = '*'
      }
    }
  } else if (a[0] == 'rotate') {
    if (a[1] == 'row') {
      rotateScreen(parseInt(a[3]), false, parseInt(a[5]))
    } else {
      rotateScreen(false, parseInt(a[3]), parseInt(a[5]))
    }
  }
})

function rotateScreen (row, col, amount) {
  var fromI = (row !== false) ? 0 : col, toI = (row !== false) ? screen.length - 1 : col
  var fromJ = (col !== false) ? 0 : row, toJ = (col !== false) ? screen[0].length - 1 : row
  var di = (row !== false) ? 1 : 0
  var dj = (col !== false) ? 1 : 0

  for (var n = 0; n < amount; n++) {
    var buffer = false
    for (var i = fromI; i <= toI; i++) {
      for (var j = fromJ; j <= toJ; j++) {
        if (buffer === false) {
          buffer = screen[i][j]
        } else {
          var otherBuffer = screen[i][j]
          screen[i][j] = buffer
          buffer = otherBuffer
        }
      }
    }
    screen[fromI][fromJ] = buffer
  }
}

for (var j = 0; j < screen[0].length; j++) {
  var str = ''
  for (var i = 0; i < screen.length; i++) {
    str += screen[i][j]
  }
  console.log(str)
}

console.log(u.countGrid(screen, '*'))
