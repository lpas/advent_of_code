const utils = require('../utils/utils')
const input = utils.readData('input')


const rules = {
    '0': '0000',
    '1': '0001',
    '2': '0010',
    '3': '0011',
    '4': '0100',
    '5': '0101',
    '6': '0110',
    '7': '0111',
    '8': '1000',
    '9': '1001',
    'A': '1010',
    'B': '1011',
    'C': '1100',
    'D': '1101',
    'E': '1110',
    'F': '1111',
}


console.log(input)

let transmission = input[0]
let pos = 0
function* transmissionGenerator() {
    while (transmission.length > pos) {
        yield rules[transmission[pos++]]
    }
}

const tg = transmissionGenerator();
let str = ''
function get(n) {
    while (str.length < n) {
        str += tg.next().value
    }
    const result = str.slice(0, n)
    str = str.slice(n)
    return result
}


const versions = []


function readPackage(depth = 0) {
    const version = parseInt(get(3), 2)
    const typeId = parseInt(get(3), 2)
    let count = 6
    let value = -1
    versions.push(version)
    // console.log({ version, typeId, depth })
    if (typeId === 4) {
        let literalValue = ''
        while (true) {
            count += 5
            const isLast = get(1) === '0'
            literalValue += get(4)
            if (isLast) {
                break
            }
        }
        value = parseInt(literalValue, 2)
    } else {
        const values = []
        const mode = get(1)
        count += 1
        let length = 0
        if (mode === '0') {
            const totalLength = parseInt(get(15), 2)
            count += 15
            while (totalLength > length) {
                const { value, count: usedLength } = readPackage(depth + 1)
                values.push(value)
                length += usedLength
            }
        } else {
            const numPackages = parseInt(get(11), 2)
            count += 11
            for (let i = 0; i < numPackages; i++) {
                const { value, count: usedLength } = readPackage(depth + 1)
                values.push(value)
                length += usedLength
            }
        }
        count += length
        switch (typeId) {
            case 0: value = values.reduce((a, b) => a + b); break
            case 1: value = values.reduce((a, b) => a * b, 1); break
            case 2: value = values.sort((a, b) => a - b)[0]; break
            case 3: value = values.sort((a, b) => b - a)[0]; break
            case 5: value = values[0] > values[1] ? 1 : 0; break
            case 6: value = values[0] < values[1] ? 1 : 0; break
            case 7: value = values[0] == values[1] ? 1 : 0; break
        }


    }
    return { value, count }
}



const { value, count } = readPackage()
// part 2
console.log(value)
// part 1
console.log(versions.reduce((a, b) => a + b))
console.log({ pos, strLength: transmission.length, str })





