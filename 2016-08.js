var u = require('./utils');

// WIP

var screen = u.createGrid(50, 6, 0);

u.mockInput('rect 1x1\nrotate row y=0 by 10');

u.readInputAsLines(function(line) {
	var a = line.split(/[ =]/g);
	if (a[0] == 'rect') {
		var coords = a[1].split('x');
		for (var i = 0; i < parseInt(coords[0]); i++) {
			for (var j = 0; j < parseInt(coords[1]); j++) {
				screen[i][j] = 1;
			}
		}
	}
	else if (a[0] == 'rotate') {
		if (a[1] == 'row') {
			rotateScreen(parseInt(a[3]), false, parseInt(a[5]));
		}
		else {
			rotateScreen(false, parseInt(a[3]), parseInt(a[5]));
		}
	}
	
});

function rotateScreen(row, col, amount) {
	var fromI = (row !== false) ? 0 : col, toI = (row !== false) ? screen.length-1 : col;
	var fromJ = (col !== false) ? 0 : row, toJ = (col !== false) ? screen[0].length-1 : row;
	var di = (row !== false) ? 1 : 0;
	var dj = (col !== false) ? 1 : 0;
	//console.log(fromI, fromJ, toI, toJ, "/",di, dj)
		
	for (var n = 0; n < amount; n++) {
		for (var i = toI; i >= fromI; i--) {
			for (var j = toJ; j >= fromJ; j--) {
				var newI = (i + di) % screen.length;
				var newJ = (j + dj) % screen[0].length;
				console.log(i,j,"<>",newI,newJ);
				var buffer = screen[newI][newJ];
				screen[newI][newJ] = screen[i][j];
				screen[i][j] = buffer;
			}	
		}
	}
}

console.log(screen, u.countGrid(screen, 1));