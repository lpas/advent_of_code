const fs = require('fs');
const data = fs.readFileSync(__dirname + '/input').toString()

const input = data
    .split('\n')
    .map((item) => item.trim('\r'))
    .filter((input) => input.trim() !== '')


let position = 0
let depth = 0
let aim = 0

for (item of input) {
    let [command, value] = item.split(' ')
    value = +value
    switch (command) {
        case 'forward':
            position += value
            depth += aim * value
            break
        case 'down':
            // depth += value
            aim += value
            break
        case 'up':
            // depth -= value
            aim -= value
            break
    }
    // console.log(position, depth, aim)
}

console.log(position, depth, position * depth)