var u = require('./utils');

var SIZE = 8;

var freqs = [];
for (var i = 0; i < SIZE; i++) {
	freqs.push({});
}

//u.mockInput('eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar');

u.readInputAsLines(function(line) {	
	for (var i = 0; i < SIZE; i++) {
		var c = line.charAt(i);
		if (!freqs[i][c]) {
			freqs[i][c] = 0;
		}
		freqs[i][c]++;
	}
});

var result = '', result2 = '';
freqs.forEach(function(map) {
	var keep, keep2;
	for (var key in map) {
		if (!keep || map[key] > map[keep] && !!key) {
			keep = key;
		}
		if (!keep2 || map[key] < map[keep2] && !!key) {
			keep2 = key;
		}
	}
	result += keep;
	result2 += keep2;
});

console.log(result, result2);
