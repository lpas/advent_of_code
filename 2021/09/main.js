const utils = require('../utils/utils')
const input = utils.readData(__dirname + '/input')

const data = input.map((item) => item.split('').map((item) => +item))

const points = []

let count = 0
data.forEach((line, y) => {
    line.forEach((point, x) => {
        const xm1 = data[y][x - 1]
        const y1 = data[y + 1]?.[x]
        const x1 = data[y][x + 1]
        const ym1 = data[y - 1]?.[x]

        if ((point < xm1 || xm1 === undefined) &&
            (point < y1 || y1 === undefined) &&
            (point < x1 || x1 === undefined) &&
            (point < ym1 || ym1 === undefined)
        ) {
            count += point + 1
            points.push([y, x])
        }
    })
    return
})
console.log(count)



const basins = points.map((point) => {
    const toCheck = [point]
    const inList = new Set([`${point[0]}_${point[1]}`])
    while (toCheck.length) {
        const [y, x] = toCheck.pop();
        [
            [y, x - 1],
            [y + 1, x],
            [y, x + 1],
            [y - 1, x]
        ].forEach(([y, x]) => {
            const item = data[y]?.[x]
            const key = `${y}_${x}`
            if (item !== undefined && item !== 9 && !inList.has(key)) {
                toCheck.push([y, x])
                inList.add(key)
            }
        })
    }
    return inList.size
}).sort((a, b) => b - a)

console.log(basins[0] * basins[1] * basins[2])
