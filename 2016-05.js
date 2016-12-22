var md5 = require('js-md5')

var password = '', password2 = '????????', i = 0, c6 = 2, c7
while (password2.indexOf('?') != -1) {
  var hash = md5('wtnhxymk' + i++)
  if (hash.indexOf('00000') == 0) {
    c6 = hash.charAt(5)
    c7 = hash.charAt(6)
    if (password.length < 8) {
      password += c6
    }
    if (c6 < 8 && password2.charAt(c6) == '?') {
      password2 = password2.slice(0, c6) + c7 + password2.slice(parseInt(c6) + 1)
    }
    console.log(i, hash.slice(0, 7), '==>', password, password2)
  }
}
