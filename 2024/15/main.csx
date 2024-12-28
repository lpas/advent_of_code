using System.IO;
using System;
using System.Text.RegularExpressions;
using System.Linq;
using System.Data;
using System.Security.Cryptography;

var input = File.ReadAllLines("input").ToList();

var map = new List<List<char>>();
var moves = new List<string>();

foreach (var line in input) {
    if (line.Count() > 0 && line[0] == '#') {
        map.Add(line.ToCharArray().ToList());
    } else {
        if (line.Count() > 0) {
            moves.Add(line);
        }
    }
}



void Part1() {
    (int x, int y) pos = GetStart(map);
    foreach (var moveline in moves) {
        foreach (var move in moveline) {
            (int x, int y) dir = move switch {
                '^' => (0, -1),
                '>' => (1, 0),
                '<' => (-1, 0),
                'v' => (0, 1),
                _ => throw new Exception(),
            };
            var curr = pos;
            var stack = new Stack<(int x, int y)>();
            stack.Push(pos);
            while (true) {
                curr = (curr.x + dir.x, curr.y + dir.y);
                stack.Push(curr);
                if (map[curr.y][curr.x] == '.') {
                    break;
                } else if (map[curr.y][curr.x] == '#') {
                    stack.Clear(); // no free space 
                    break;
                }
            }
            while (stack.Count() > 1) {
                var v = stack.Pop();
                var p = stack.Peek();
                map[v.y][v.x] = map[p.y][p.x];
            }
            if (stack.Count() > 0) {
                var v = stack.Pop();
                map[v.y][v.x] = '.';
                pos = (pos.x + dir.x, pos.y + dir.y);
            }

        }
    }
    var count = 0;
    for (var y = 0; y < map.Count(); y++) {
        for (var x = 0; x < map[0].Count(); x++) {
            if (map[y][x] == 'O') {
                count += y * 100 + x;
            }
        }
    }
    Console.WriteLine(count);
}
void Part2() {
    var dMap = new Dictionary<char, (char, char)>(){
        {'#', ('#', '#')},
        {'O', ('[', ']')},
        {'.', ('.', '.')},
        {'@', ('@', '.')},
    };
    for (var y = 0; y < map.Count(); y++) {
        var line = new List<char>();
        foreach (var c in map[y]) {
            line.Add(dMap[c].Item1);
            line.Add(dMap[c].Item2);

        }
        map[y] = line;
    }
    (int x, int y) pos = GetStart(map);
    foreach (var moveline in moves) {
        foreach (var move in moveline) {
            (int x, int y) dir = move switch {
                '^' => (0, -1),
                '>' => (1, 0),
                '<' => (-1, 0),
                'v' => (0, 1),
                _ => throw new Exception(),
            };
            var curr = pos;

            if (move == '>' || move == '<') {
                var stack = new Stack<(int x, int y)>();
                stack.Push(pos);
                while (true) {
                    curr = (curr.x + dir.x, curr.y + dir.y);
                    stack.Push(curr);
                    if (map[curr.y][curr.x] == '.') {
                        break;
                    } else if (map[curr.y][curr.x] == '#') {
                        stack.Clear(); // no free space 
                        break;
                    }
                }
                while (stack.Count() > 1) {
                    var v = stack.Pop();
                    var p = stack.Peek();
                    map[v.y][v.x] = map[p.y][p.x];
                }
                if (stack.Count() > 0) {
                    var v = stack.Pop();
                    map[v.y][v.x] = '.';
                    pos = (pos.x + dir.x, pos.y + dir.y);
                }
            } else { // different for part2 as boxes up/down can move more boxes

                var (_, yDir) = dir;
                var y = pos.y;
                var stack = new Stack<(int, HashSet<int>)>();
                stack.Push((y, new HashSet<int>() { pos.x }));
                while (true) {
                    y += yDir;
                    var last = stack.Peek();
                    var xList = new HashSet<int>();
                    foreach (var x in last.Item2) {
                        if (map[y][x] == ']') {
                            xList.Add(x);
                            xList.Add(x - 1);
                        } else if (map[y][x] == '[') {
                            xList.Add(x);
                            xList.Add(x + 1);
                        } else if (map[y][x] == '#') {
                            xList.Clear();
                            stack.Clear();
                            break;
                        }
                    }
                    if (xList.Count() == 0) {
                        break;
                    }
                    stack.Push((y, xList));
                }
                if (stack.Count > 0) {
                    pos = (pos.x, pos.y + yDir);
                }
                while (stack.Count > 0) {
                    var (yY, xList) = stack.Pop();
                    foreach (var x in xList) {
                        var v = map[yY][x];
                        map[yY + yDir][x] = map[yY][x];
                        map[yY][x] = '.';
                    }
                }
            }
        }
    }

}


// Part1();
Part2();
var count = 0;
for (var y = 0; y < map.Count(); y++) {
    for (var x = 0; x < map[0].Count(); x++) {
        if (map[y][x] == '[') {
            count += y * 100 + x;
        }
    }
}
Console.WriteLine(count);
// PrintMap(map);



(int, int) GetStart(List<List<char>> map) {
    for (var y = 0; y < map.Count(); y++) {
        for (var x = 0; x < map[0].Count(); x++) {
            if (map[y][x] == '@') return (x, y);
        }
    }
    return (-1, -1);
}

void PrintMap(List<List<char>> map) {
    foreach (var line in map) {
        Console.WriteLine(new string(line.ToArray()));
    }
}