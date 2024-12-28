using System.IO;
using System;
using System.Text.RegularExpressions;
using System.Linq;
using System.Data;
using System.Diagnostics.Metrics;
using System.CodeDom.Compiler;


var input = File.ReadAllLines("input").ToList();

var width = 6;
var height = 6;

var nanoseconds = 12;

width = 70;
height = 70;
nanoseconds = 1024;


var grid = new bool[width + 1, height + 1];


for (var i = 0; i < nanoseconds; i++) {
    var curr = input[i].Split(',').Select(int.Parse).ToList();
    grid[curr[0], curr[1]] = true;

}

var start = (0, 0);
var end = (width, height);

// part1
Console.WriteLine(GetSteps());

// part2 
for (var i = nanoseconds; i < input.Count(); i++) {
    var curr = input[i].Split(',').Select(int.Parse).ToList();
    grid[curr[0], curr[1]] = true;
    var steps = GetSteps();
    if (steps == 0) {
        Console.WriteLine($"{curr[0]},{curr[1]}");
        break;
    }
}

// Dictionary<(int x, int y), (int x, int y)> 
int GetSteps() {
    var frontier = new Queue<(int x, int y)>();
    frontier.Enqueue(start);
    var cameFrom = new Dictionary<(int x, int y), (int x, int y)?>();
    cameFrom[start] = null;
    (int x, int y)[] dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while (frontier.Count() > 0) {
        var curr = frontier.Dequeue();
        if (curr == end) break;
        foreach (var dir in dirs) {
            (int x, int y) next = (curr.x + dir.x, curr.y + dir.y);
            if (next.x < 0 || next.y < 0 || next.x > width || next.y > height)
                continue;
            if (grid[curr.x, curr.y])
                continue;
            if (cameFrom.ContainsKey(next))
                continue;
            frontier.Enqueue(next);
            cameFrom[next] = curr;
        }
    }

    var pos = end;
    var steps = 0;
    if (!cameFrom.ContainsKey(end)) return 0;


    while (true) {
        if (cameFrom[pos] == null) break;
        pos = ((int width, int height))cameFrom[pos];
        steps++;
    }
    return steps;
}




// for (var y = 0; y <= height; y++) {
//     for (var x = 0; x <= width; x++) {
//         Console.Write(grid[x, y] ? '#' : '.');
//     }
//     Console.WriteLine();
// }
