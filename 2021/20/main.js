const utils = require('../utils/utils')
const input = utils.readData('input')


const enchanc = input.shift().split('').map((item) => item === '#' ? 1 : 0)

let fill = 0
let img = input.map((line) => line.split('').map((item) => item === '#' ? 1 : 0))


function getOut(x, y) {
    let s = ''
    for (let i = y - 1; i <= y + 1; i++) {
        for (let j = x - 1; j <= x + 1; j++) {
            s += img[i]?.[j] ?? fill
        }
    }
    const pos = parseInt(s, 2)
    return enchanc[pos]
}

function print() {
    console.log(img.map(
        (line) => line.map((item) => item == 0 ? '.' : '#').join('')).join('\n'))
}

const loops = 50


for (let loop = 0; loop < loops; loop++) {
    let width = img[0].length

    img.push(Array(width).fill(fill))
    img.push(Array(width).fill(fill))
    img.unshift(Array(width).fill(fill))
    img.unshift(Array(width).fill(fill))

    img.forEach((line) => {
        line.push(fill)
        line.push(fill)
        line.unshift(fill)
        line.unshift(fill)
    })

    img = img.map((line, y) => line.map((_, x) => getOut(x, y)))
    fill = enchanc[parseInt(('' + fill).repeat(9), 2)]
    // print()
}

const litPixels = img.map((line) => line.reduce((a, b) => a + b)).reduce((a, b) => a + b)
console.log(litPixels)