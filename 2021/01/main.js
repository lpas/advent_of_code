const fs = require('fs');
const data = fs.readFileSync(__dirname + '/input').toString()

let input = data
    .split('\n')
    .map((item) => item.trim('\r'))
    .filter((input) => input.trim() !== '')
    .map((item) => +item)


// part two
input = input
    .map((item, index) => item + input[index + 1] + input[index + 2])
    .filter((item) => !isNaN(item))

const result = input
    .map((value, index) => index === 0 ? false : value > input[index - 1])
    .filter(Boolean)
    .length

console.log(result)

