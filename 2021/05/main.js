const utils = require('../utils/utils')
const input = utils.readData('input')


const data = input.map((row) => row
    .split('->')
    .map((item) => item.trim().split(',')
        .map((item) => +item)))



const map = new Map()

function incItem(x, y) {
    key = `${x}_${y}`
    if (map.has(key)) {
        map.set(key, map.get(key) + 1)
    } else {
        map.set(key, 1)
    }
}



for (const line of data) {
    let [[x1, y1], [x2, y2]] = line
    // console.log(x1, y1, x2, y2)
    if (x1 !== x2 && y1 !== y2) {
        // continue // part1
        const length = Math.abs(x1 - x2)
        for (let i = 0; i <= length; i++) {
            incItem(x1, y1)
            x1 += x1 > x2 ? -1 : 1
            y1 += y1 > y2 ? -1 : 1
        }
    } else {
        if (x1 === x2) {
            if (y1 > y2) {
                [y1, y2] = [y2, y1]
            }
            for (; y1 <= y2; y1++) {
                incItem(x1, y1)
            }

        } else {
            if (x1 > x2) {
                [x1, x2] = [x2, x1]
            }
            for (; x1 <= x2; x1++) {
                incItem(x1, y1)
            }
        }
    }


}

const pointsCount = Array.from(map.values()).filter((item) => item >= 2).length

console.log(pointsCount)
