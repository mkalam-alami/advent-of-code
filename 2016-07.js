var u = require('./utils');

var result = 0, result2 = 0, hypernet, outerAbba, innerAbba, abas, babs;

u.readInputAsLines(function(line) {
	hypernet = false; outerAbba = false; innerAbba = false; abas = []; babs = [];
	
	for (var i = 0; i < line.length - 3; i++) {
		switch (line.charAt(i)) {
			case '[': hypernet = true; break;
			case ']': hypernet = false; break;
			default:
				// Pt. 1
				if (hypernet) {
					if (!innerAbba && isAbba(line, i)) {
						innerAbba = true;
					}
				}
				else if (!outerAbba && isAbba(line, i)) {
					outerAbba = true;
				}
				
				// Pt. 2
				if (hypernet) {
					babs.push(getBabId(line, i));
				}
				else {
					abas.push(getAbaId(line, i));
				}
		}
	}
	
	// Pt. 1
	if (outerAbba && !innerAbba) {
		result++;
	}
	
	// Pt. 2
	var found = false;
	babs.forEach(function(bab) {
		if (!found && bab && abas.indexOf(bab) != -1) {
			found = true;
		}
	});
	if (found) {
		result2++;
	}
});

function isAbba(line, i) {
	return line[i] == line[i+3] && line[i+1] == line[i+2] && line[i] != line[i+1];
}

function getAbaId(line, i) {
	return (line[i] == line[i+2]) ? line[i] + line[i+1] : null;
}

function getBabId(line, i) {
	return (line[i] == line[i+2]) ? line[i+1] + line[i] : null;
}


console.log(result, result2);