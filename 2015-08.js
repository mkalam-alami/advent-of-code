var u = require('./utils');

var total = 0;

u.readInputAsLines(function(line) {
	mem = line.slice(1, line.length -1);
	console.log(line);
	u.exit()
});

console.log(total);