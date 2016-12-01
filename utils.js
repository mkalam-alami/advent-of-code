var fs = require('fs');

var mock = null;

module.exports = {
	l: console.log,
	exit: exit,

	readInput: readInput,
	readInputAsLines: readInputAsLines,
	readInputAsChars: readInputAsChars,
	mockInput: mockInput,

	createArray: createArray,
	countArray: countArray,
	sumArray: sumArray,

	createGrid: createGrid,
	countGrid: countGrid,
	sumGrid: sumGrid,

	sortObjectByKeys: sortObjectByKeys
};

function exit(message, afterNCalls) {
	if (afterNCalls) {
		if (message) console.log(message);
		process.exit(0);
	}
}

function mockInput(input) {
	mock = input;
}

function readInput(callback) {
	if (mock) {
		callback(mock);
	}
	else {
		var fileContents = fs.readFileSync(_callerFileName() + '.txt');
		callback(fileContents.toString());
	}
}

function readInputAsLines(callback) {
	readInput(function(input) {
		input.split('\n').forEach(callback);
	});
}

function readInputAsChars(callback) {
	readInput(function(input) {
		for (var i = 0; i < input.length; i++) {
			callback(input[i]);
		}
	});
}

function _callerFileName() {
	var traceElemBits = (new Error()).stack.split('\n')[4].split(/[\\:.]/g);
	return traceElemBits[traceElemBits.length - 4];
}

function createArray(length, value) {
	return new Array(length).fill(value);
}

function createGrid(w, h, value) {
	var grid = [];
	for (var i = 0; i < w; i++) {
		grid[i] = createArray(h, value);
	}
	return grid;
}

function sumArray(array) {
	var total = 0;
	array.forEach(function(cell) {
		total += cell;
	})
	return total;
}

function sumGrid(grid) {
	var total = 0;
	grid.forEach(function(line) {
		total += sumArray(line);
	})
	return total;
}

function countArray(array, value) {
	var total = 0;
	array.forEach(function(cell) {
		if (cell === value) {
			total++;
		}
	})
	return total;
}

function countGrid(grid, value) {
	var total = 0;
	grid.forEach(function(line) {
		total += countArray(line, value);
	})
	return total;
}

function sortObjectByKeys(obj) {
	// http://stackoverflow.com/a/29622653
    return Object.keys(obj).sort().reduce(function (result, key) {
        result[key] = obj[key];
        return result;
    }, {});
}