var u = require('./utils');

var result = 0;

u.readInputAsLines(function(line) {
    var tokens = line.split(/[-\[\]]/g);
    var str = '', id, hash;
    tokens.forEach(function(token, i) {
       if (i < tokens.length - 3) {
           str += token;
       }
       else if (i == tokens.length - 3) {
           id = parseInt(token);
       }
       else if (i == tokens.length - 2) {
           hash = token;
       }
    });
    
    var freqs = {}, keys = [];
    var c;
    for (var i = 0; i < str.length; i++) {
        c = str[i];
        if (!freqs[c]) {
            freqs[c] = 0;
            keys.push(c);
        }
        freqs[c]++;
    }
    
    keys = keys.sort(function (c1, c2) {
       return freqs[c2] - freqs[c1] + (c1.charCodeAt(0) - c2.charCodeAt(0)) / 1000; 
    });
    if (keys.slice(0,5).join('') == hash) {
        result += id;
    }
    
    var decrypted = '';
    for (var i = 0; i < str.length; i++) {
        decrypted += String.fromCharCode( (str[i].charCodeAt(0) - 97 + id) % 26 + 97);
    }
    if (decrypted.indexOf('northpole') != -1) {
        console.log(decrypted, id);
    }
    
});

console.log(result);