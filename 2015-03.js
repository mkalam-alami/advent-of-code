var utils = require('./utils');


var santa = {i:0,j:0}, robot = {i:0,j:0}, noRobot = {i:0,j:0}, index = 0;
var visited = {};
var visited2 = {};

function visit(grid, i, j) {
	if (!grid[i]) grid[i] = {};
	grid[i][j] = true;
}

function count(grid) {
	var total = 0;
	for (var i in grid) {
		var line = grid[i];
		for (var j in line) {
			total++;
		}
	}
	return total;
}

function countRobot() {
	return 
}

visit(visited, santa.i, santa.j);
visit(visited2, santa.i, santa.j);

utils.readInputAsChars(function(c) {
	var santa2;
	if (index++ % 2 == 0) {
		santa2 = noRobot;
	}
	else {
		santa2 = robot;
	}

	switch (c) {
		case 'v': santa.j++; santa2.j++; break;
		case '<': santa.i--; santa2.i--; break;
		case '>': santa.i++; santa2.i++; break;
		case '^': santa.j--; santa2.j--;
	}
	visit(visited, santa.i, santa.j);
	visit(visited2, santa2.i, santa2.j);

});

console.log(count(visited), count(visited2));