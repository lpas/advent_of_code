const utils = require('../utils/utils')
const input = utils.readData('input')

function checkBoard(board) {
    for (let i = 0; i < 5; i++) {
        for (let j = 0; j < 5; j++) {
            if (board[i][j] !== true) {
                break
            }
            if (j === 4) return true
        }
    }
    for (let i = 0; i < 5; i++) {
        for (let j = 0; j < 5; j++) {
            if (board[j][i] !== true) {
                break
            }
            if (j === 4) return true
        }
    }
    return false
}

function markNumber(board, number) {
    for (let rowIndex in board) {
        for (let colIndex in board[rowIndex]) {
            if (board[rowIndex][colIndex] === number) {
                board[rowIndex][colIndex] = true
            }
        }
    }
}


function getWinningCombo(boards, numbers) {
    for (let number of numbers) {
        for (i = 0; i < boards.length; i++) {
            const board = boards[i]
            markNumber(board, number)
            if (checkBoard(board)) {
                // return [board, number] // part 1
                if (boards.length === 1) {
                    return [board, number]
                }
                boards.splice(i, 1)
                i--
            }
        }
    }
}


const numbers = input.shift().split(',').map((item) => +item)
const boards = []
while (input.length) {
    const board = []
    for (let i = 0; i < 5; i++) {
        board.push(input.shift().split(' ').filter((item) => item !== '').map((item) => +item))
    }
    boards.push(board)
}
const [board, lastNumber] = getWinningCombo(boards, numbers)
const sum = (a, b) => a + b
const score = board
    .map((row) => row
        .map((item) => item === true ? 0 : item)
        .reduce(sum))
    .reduce(sum) * lastNumber


// console.log(boards, board, lastNumber)
console.log(score)


