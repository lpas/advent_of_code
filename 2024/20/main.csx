using System.IO;
using System;
using System.Text.RegularExpressions;
using System.Linq;
using System.Data;
using System.Collections.Concurrent;
using System.Threading;

var input = File.ReadAllLines("input").ToList();


var height = input.Count();
var width = input[0].Length;

var start = GetPos('S');
var end = GetPos('E');
(int x, int y)[] dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

Dictionary<(int x, int y), ((int x, int y), int steps)?> GetCameFrom((int x, int y) pos) {
    var frontier = new Queue<((int x, int y), int steps)>();
    frontier.Enqueue((pos, 1));
    var cameFrom = new Dictionary<(int x, int y), ((int x, int y), int steps)?>();
    cameFrom[pos] = null;
    while (frontier.Count() > 0) {
        var (curr, steps) = frontier.Dequeue();
        foreach (var dir in dirs) {
            (int x, int y) next = (curr.x + dir.x, curr.y + dir.y);
            if (next.x < 0 || next.y < 0 || next.x > width || next.y > height)
                continue;
            if (input[next.y][next.x] == '#')
                continue;
            if (cameFrom.ContainsKey(next))
                continue;
            frontier.Enqueue((next, steps + 1));
            cameFrom[next] = (curr, steps);
        }
    }
    return cameFrom;
}
var fromStart = GetCameFrom(start);
var fromEnd = GetCameFrom(end);

var pos = start;
var steps = 0;
while (true) {
    if (fromEnd[pos] == null) break;
    (pos, _) = (((int width, int height), int steps))fromEnd[pos];
    steps++;
}
Console.WriteLine(steps);
var defaultSteps = steps;

Console.WriteLine(GetCheats(2).Where(item => item.Key >= 100).Select(item => item.Value).Sum());
Console.WriteLine(GetCheats(20).Where(item => item.Key >= 100).Select(item => item.Value).Sum());
// foreach (var cheat in cheats.OrderBy(item => item.Key)) {
//     Console.WriteLine($"There are {cheat.Value} cheats that save {cheat.Key} picoseconds.");
// }
// Console.WriteLine(count);

Dictionary<int, int> GetCheats(int moves) {
    var cheats = new Dictionary<int, int>();
    var pos = start;
    var steps = 0;
    while (true) {
        if (fromEnd[pos] == null) break;

        var posSet = new Queue<((int x, int y), int steps)>();
        var dict = new Dictionary<((int x, int y), (int x, int y)), int>();
        posSet.Enqueue((pos, 0));
        var visited = new HashSet<(int x, int y)>();
        while (posSet.Count > 0) { // find positions from curr that are max x moves away
            var (c, currSteps) = posSet.Dequeue();
            if (currSteps >= moves) continue;
            if (visited.Contains(c)) continue;
            visited.Add(c);
            var nextSteps = currSteps + 1;
            foreach (var dir in dirs) {
                var next = (c.x + dir.x, c.y + dir.y);

                if (!ValidPos(next)) continue;
                posSet.Enqueue((next, nextSteps));
                if (!dict.ContainsKey((pos, next))) {
                    dict[(pos, next)] = nextSteps;
                } else {
                    if (nextSteps < dict[(pos, next)]) {
                        dict[(pos, next)] = nextSteps;
                    }
                }
            }
        }

        foreach (var d in dict) {
            if (fromEnd.ContainsKey(d.Key.Item2)) {
                var s = steps + (fromEnd[d.Key.Item2]?.steps ?? 0) + d.Value;
                if (s < defaultSteps) {
                    var savedSteps = defaultSteps - s;
                    if (cheats.ContainsKey(savedSteps)) {
                        cheats[savedSteps]++;
                    } else {
                        cheats[savedSteps] = 1;
                    }
                }
            }
        }
        steps++;
        (pos, _) = (((int width, int height), int steps))fromEnd[pos];
    }
    return cheats;
}

bool ValidPos((int x, int y) pos) {
    if (pos.x < 0 || pos.y < 0 || pos.x > width || pos.y > height)
        return false;
    return true;
}

(int x, int y) GetPos(char c) {
    for (var y = 0; y < height; y++) {
        for (var x = 0; x < width; x++) {
            if (input[y][x] == c) return (x, y);
        }
    }
    throw new Exception();
}