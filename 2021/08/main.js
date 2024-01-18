const utils = require('../utils/utils')
const input = utils.readData('input')


const data = input.map((item) =>
    item
        .split('|')
        .map((item) => item.trim()
            .split(' ')
            .map((item) => Array.from(item).sort())
        ))


// part 1
// let count = 0
// data
//     .map((item) => item[1])
//     .map((line) => line
//         .map((item) => item.length)
//         .forEach((length) => {
//             if (length === 2 || length == 4 || length == 3 || length == 7) {
//                 count++
//             }
//         })
//     )
// console.log(count)

// 0:      1:      2:      3:      4:
// aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
// ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
// gggg    ....    gggg    gggg    ....

//  5:      6:      7:      8:      9:
// aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
// dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
// gggg    gggg    ....    gggg    gggg


// length = 2 -> 1
// length = 4 -> 4
// length = 3 -> 7
// length = 7 -> 8
// length = 5 & all segs from 1 -> 3
// length = 6 & all segs from 3 -> 9
// segs from 9 - 3 -> b
// length = 5 with b -> 5
// length = 5 -> 2
// segs from 1 - 5 -> f
// segs from 1 - f -> c
// length = 6 & !c -> 6
// last is 0

function getMapping(items) {
    items = items.sort((a, b) => a.length - b.length)

    const mapping = {
        0: undefined,
        1: items[0],
        2: undefined,
        3: undefined,
        4: items[2],
        5: undefined,
        6: undefined,
        7: items[1],
        8: items[9],
        9: undefined
    }
    const length5 = [items[3], items[4], items[5]]
    const length6 = [items[6], items[7], items[8]]

    mapping[3] = length5.splice(length5.findIndex((item) => item.filter((i) => mapping[1].includes(i)).length == mapping[1].length), 1)[0]
    mapping[9] = length6.splice(length6.findIndex((item) => item.filter((i) => mapping[3].includes(i)).length == mapping[3].length), 1)[0]
    const b = mapping[9].find((i) => !mapping[3].includes(i))
    mapping[5] = length5.splice(length5.findIndex((item) => item.includes(b)), 1)[0]
    mapping[2] = length5.pop()
    const f = mapping[1].find((i) => mapping[5].includes(i))
    const c = mapping[1].find((item) => item !== f)
    mapping[6] = length6.splice(length6.findIndex((item) => !item.includes(c)), 1)[0]
    mapping[0] = length6.pop()

    return Object.fromEntries(
        Object.entries(mapping).map(([num, letters]) => ([letters.join(''), num]))
    )
}

const result = data.map(([pattern, values]) => {
    const mapping = getMapping(pattern)
    return +values.map((item) => mapping[item.join('')]).join('')
}).reduce((a, b) => a + b)
console.log(result)
