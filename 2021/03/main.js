const utils = require('../utils/utils')
const input = utils.readData('input')

const arr = Array(input[0].length).fill(0)

for (const item of input) {
    for (const index in item) {
        arr[index] += item[index] === '1' ? 1 : -1
    }
}

const gamma = parseInt(arr.map((item) => item > 0 ? '1' : '0').join(''), 2)
const epsilon = parseInt(arr.map((item) => item < 0 ? '1' : '0').join(''), 2)
console.log(gamma, epsilon, gamma * epsilon)

// part 2

function getValue(func) {
    let data = input
    let index = 0
    while (data.length > 1) {
        let count = 0
        for (const item of data) {
            count += item[index] === '1' ? 1 : -1
        }
        const num = func(count)
        data = data.filter((item) => item[index] === num)
        // console.log(index, count, num, data)
        index++
    }
    return parseInt(data[0], 2)
}


const oxygenGeneratorRating = getValue((count) => count >= 0 ? '1' : '0')
const co2ScrubberRating = getValue((count) => count < 0 ? '1' : '0')

console.log(oxygenGeneratorRating, co2ScrubberRating, oxygenGeneratorRating * co2ScrubberRating)