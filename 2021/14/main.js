const utils = require('../utils/utils')
const input = utils.readData('input')

let template = input.shift()
const rules = new Map(input.map((item) => item.split(' -> ')))

const steps = 40

let pairs = Object.fromEntries(Array.from(rules.keys()).map((item) => [item, 0]))
let count = Object.fromEntries(Array.from(new Set(rules.values())).map((item) => [item, 0]))
for (let i = 0; i < template.length - 1; i++) {
    count[template[i]]++
    const pair = template.slice(i, i + 2)
    pairs[pair]++
}
count[template.at(-1)]++

for (let step = 1; step <= steps; step++) {
    Object.entries(pairs).forEach(([key, num]) => {
        const rule = rules.get(key)
        const a = key[0] + rule
        const b = rule + key[1]
        count[rule] += num
        pairs[a] += num
        pairs[b] += num
        pairs[key] -= num
    })
}

const numbers = Object.values(count).sort((a, b) => a - b)
console.log(numbers.at(-1) - numbers[0])
