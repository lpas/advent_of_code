const utils = require('../utils/utils')
const input = utils.readData('input')

const bucket = Array(9).fill(0)
input[0]
    .split(',')
    .map((item) => +item)
    .forEach((item) => {
        bucket[item]++
    })


const days = 256
for (let day = 1; day <= days; day++) {
    const zeroes = bucket.shift()
    bucket.push(zeroes)
    bucket[6] += zeroes
}
console.log(bucket.reduce((a, b) => a + b))