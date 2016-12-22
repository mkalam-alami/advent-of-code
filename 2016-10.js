'use strict'

let u = require('./utils')

var graph = {}

u.readInputAsLines(function (line) {
  if (line.indexOf('value') == 0) {
    var [, value,,,, targetBot] = line.split(' ')
    init('bot' + targetBot)
    assign(graph['bot' + targetBot], parseInt(value))
  } else {
    var [, sourceBot,,,, lowType, lowId,,,, highType, highId] = line.split(' ')
    init('bot' + sourceBot)
    graph['bot' + sourceBot].low = lowType + lowId
    graph['bot' + sourceBot].high = highType + highId
  }
})

function init (key) {
  if (!graph[key]) {
    graph[key] = {
      low: false,
      lowValue: false,
      high: false,
      highValue: false,
      resolved: false
    }
  }
}

function assign (bot, value) {
  if (!bot.lowValue) { bot.lowValue = value; return false } else if (bot.lowValue < value) bot.highValue = value
  else { bot.highValue = bot.lowValue; bot.lowValue = value }
}

var progress = true
while (progress) {
  progress = false
  for (var key in graph) {
    var bot = graph[key]
    if (bot.lowValue && bot.highValue && !bot.resolved) {
      init(bot.low)
      init(bot.high)
      assign(graph[bot.low], bot.lowValue)
      assign(graph[bot.high], bot.highValue)
      progress = bot.resolved = true
      if (bot.lowValue == 17 && bot.highValue == 61) {
        console.log(key, bot)
      }
    }
  }
}

console.log(graph['output0'].lowValue * graph['output1'].lowValue * graph['output2'].lowValue)
