var md5 = require('js-md5')

var base = 'bgvyzdsv'

var i = 0
while (true) {
  var is = i.toString()
  while (is.length < 6) {
    is = '0' + is
  }
  if (md5(base + is).indexOf('00000') == 0) {
    console.log(i)
    process.exit()
  }
  if (i % 1000 == 0) console.log(i + '...')
  i++
}
