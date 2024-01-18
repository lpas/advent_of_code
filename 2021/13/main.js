const utils = require('../utils/utils')
const input = utils.readData('input')

const folds = input.splice(input.findIndex((item) => item[0] === 'f'))
const data = new Set(input);

// [folds[0]] // step1
folds
    .forEach((fold) => {
        const foldSplit = fold.split('=')
        const foldType = foldSplit[0].at(-1)
        const foldNum = +foldSplit[1];

        [...data].forEach((item) => {
            let [x, y] = item.split(',').map((item) => +item)

            if (foldType === 'y') {
                if (y < foldNum) return
                y = foldNum + foldNum - y
            } else {
                if (x < foldNum) return
                x = foldNum + foldNum - x
            }


            data.delete(item)
            data.add(`${x},${y}`)
        })
    })

console.log(data.size)


let maxX = 0
let maxY = 0
data.forEach((item) => {
    const [x, y] = item.split(',').map((item) => +item)
    if (maxX < x) maxX = x
    if (maxY < y) maxY = y
})


console.log({ maxX, maxY })

for (let y = 0; y <= maxY; y++) {
    let s = ''
    for (let x = 0; x <= maxX; x++) {
        s += data.has(`${x},${y}`) ? '#' : '.'
    }
    console.log(s)
}

// JPZCUAUR