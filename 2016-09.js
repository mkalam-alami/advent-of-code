var u = require('./utils');

u.readInput(function(line) {
	line = line.replace(/[ \r\n]/g, '');
	console.log(process(line, false));
	console.log(process(line, true));
});

function process(line, part2) {
	var result = 0, resultString = '';;
	var c;
	for (var i = 0; i < line.length; i++) {
		c = line[i];
		if (c == '(') {
			if (part2) {
				resultString += expand(line, i, part2);
			}
			else {
				result += expand(line, i, part2).length;
			}
			i = line.indexOf(')', i) + getMarkerInfo(line, i)[0];
		}
		else {
			result++;
		}
	}
	return part2 ? resultString : result;
}

function getMarkerInfo(line, i) {
	var nextDataI = line.indexOf(')', i);
	var marker = line.slice(i + 1, nextDataI);
	var markerInfo = marker.split('x');
	return markerInfo.map(function(s) { return parseInt(s); });
}

function expand(line, i, recursive) {
	var nextDataI = line.indexOf(')', i);
	var markerInfo = getMarkerInfo(line, i);
	var data = line.slice(nextDataI + 1, nextDataI + 1 + parseInt(markerInfo[0]));
	if (recursive && data.indexOf('(') != -1) {
		data = process(data, true, true);
	}
	var dupedString = '';
	for (var n = 0; n < markerInfo[1]; n++) {
		dupedString += data;
	}
	return dupedString;
}