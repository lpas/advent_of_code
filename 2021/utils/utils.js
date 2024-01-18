const fs = require('fs');

const readData = (name) => {
    return data = fs.readFileSync(name).toString()
        .split('\n')
        .map((item) => item.trim('\r'))
        .filter((input) => input.trim() !== '')
}

module.exports = {
    readData
}