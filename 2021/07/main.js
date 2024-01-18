const utils = require('../utils/utils')
const input = utils.readData('input')

const data = input[0].split(',').map((item) => +item)
const min = data.reduce((a, b) => a < b ? a : b)
const max = data.reduce((a, b) => a > b ? a : b)

let minFuel = undefined

console.log(min, max)

for (let i = min; i <= max; i++) {
    const sum = data
        .map((item) => {
            const n = Math.abs(i - item)
            // return n // part1
            return ((n ** 2) + n) / 2

        })
        .reduce((a, b) => a + b)
    if (minFuel === undefined || minFuel > sum) {
        minFuel = sum
    }
}

console.log(minFuel)
