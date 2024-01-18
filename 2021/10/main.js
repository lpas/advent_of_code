const utils = require('../utils/utils')
const input = utils.readData('input')

const data = input.map((item) => item.split(''))

const mapping = new Map([
    ['(', ')'],
    ['[', ']'],
    ['{', '}'],
    ['<', '>'],
])

const pointsMapPart1 = {
    ')': 3,
    ']': 57,
    '}': 1197,
    '>': 25137,
}

const broken = data.map((line) => {
    const stack = []
    return line.find((item) => {
        if (mapping.has(item)) { // open
            stack.push(item)
        } else { // close
            const last = stack.pop()
            if (mapping.get(last) !== item) {
                return true
            }
        }
    })
})

const points = broken
    .map((item) => pointsMapPart1[item] ?? 0)
    .reduce((a, b) => a + b)
console.log({ points })


// part 2
const pointsMapPart2 = {
    ')': 1,
    ']': 2,
    '}': 3,
    '>': 4,
}

const incomplete = data.map((line) => {
    const stack = []
    const corrupt = line.find((item) => {
        if (mapping.has(item)) { // open
            stack.push(item)
        } else { // close
            const last = stack.pop()
            if (mapping.get(last) !== item) {
                return true
            }
        }
    })
    if (corrupt) {
        return null
    }
    return stack
        .reverse()
        .map((item) => pointsMapPart2[mapping.get(item)])
        .reduce((sum, value) => sum * 5 + value, 0)

})
    .filter((item) => item !== null)
    .sort((a, b) => a - b)

console.log(incomplete[(incomplete.length - 1) / 2])
