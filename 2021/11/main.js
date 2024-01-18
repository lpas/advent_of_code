const utils = require('../utils/utils')
const input = utils.readData('input').map((item) => item.split('').map((item) => +item))

// console.log(input)




const print = (input) => {
    console.log(input.map((item) => item.join('') + '\n').join(''))
}

const steps = 1000
let flashes = 0
for (let step = 1; step <= steps; step++) {

    const flashing = Array(10).fill().map(() => Array(10).fill(false))
    const next = []
    for (let y = 0; y < 10; y++) {
        for (let x = 0; x < 10; x++) {
            if (++input[y][x] > 9) {
                next.push([y, x])
                flashing[y][x] = true
            }
        }
    }

    const inc = (y, x) => {
        if (x < 0 || x >= 10 || y < 0 || y >= 10) return
        input[y][x]++
        if (input[y][x] > 9 && !flashing[y][x]) {
            next.push([y, x])
            flashing[y][x] = true
        }
    }

    while (next.length) {
        const [y, x] = next.pop()
        flashes++
        inc(y - 1, x)
        inc(y - 1, x + 1)
        inc(y, x + 1)
        inc(y + 1, x + 1)
        inc(y + 1, x)
        inc(y + 1, x - 1)
        inc(y, x - 1)
        inc(y - 1, x - 1)
    }

    for (let y = 0; y < 10; y++) {
        for (let x = 0; x < 10; x++) {
            if (input[y][x] > 9) {
                input[y][x] = 0
            }
        }
    }


    // step2
    if (input.every((item) => item.every((item) => item === 0))) {
        console.log({ step })
        break;
    }


}

console.log({ flashes })