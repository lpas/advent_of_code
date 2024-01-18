const utils = require('../utils/utils')
const input = utils.readData('input')



const nodes = []
const getNode = (name) => {
    let node = nodes.find((item) => item.name === name)
    if (!node) {
        node = {
            name,
            big: name.toUpperCase() === name,
            links: [],
        }
        nodes.push(node)
    }
    return node
}

input.forEach((item) => {
    const [name1, name2] = item.split('-')
    const node1 = getNode(name1)
    const node2 = getNode(name2)
    node1.links.push(node2)
    node2.links.push(node1)
})

const start = getNode('start')
const end = getNode('end')

const paths = []

function ttr1(node, visited, visitedSmallTwice) {
    const n = [...visited, node]
    if (node === end) {
        paths.push(n)
        return
    }
    node.links.forEach((link) => {
        if (link === start) {
            return
        }

        if (link.big) {
            ttr1(link, n, visitedSmallTwice)
            return
        }
        const wasThere = visited.includes(link)
        if (wasThere && visitedSmallTwice) {
            return
        }
        ttr1(link, n, visitedSmallTwice || wasThere)
    })
}

// ttr1(start, [], true) // part1
ttr1(start, [], false)
console.log(paths.length)

// console.log(paths.map((item) => item.map((i) => i.name).join()))

