var u = require('./utils');

var result = 0, result2 = 0;
var sideRows = [];

u.readInputAsLines(function(line) {
	var sides = parseLine(line);
	result += triangleTest(sides[0], sides[1], sides[2]);

	sideRows.push(parseLine(line));
	if (sideRows.length == 3) {
		for (var i = 0; i < 3; i++) {
			result2 += triangleTest(sideRows[0][i], sideRows[1][i], sideRows[2][i]);
		}
		sideRows = [];
	}
});

function parseLine(line) {
	return line.split(/[ ]+/g).slice(1).map(function(i) { return parseInt(i); });
}

function triangleTest(n1, n2, n3) {
	var max = Math.max(n1, Math.max(n2, n3));
	var minSum = 0, ignored = false;
	[n1,n2,n3].forEach(function(n) {
		if (n != max || ignored) {
			minSum += n;
		}
		else {
			ignored = true;
		}
	});
	return minSum > max ? 1 : 0;
}

console.log(result, result2);