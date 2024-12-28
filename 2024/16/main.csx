global using Pos = (int x, int y);
global using Dir = (int x, int y);


using System.IO;
using System;
using System.Text.RegularExpressions;
using System.Linq;
using System.Data;
using System.Security.Cryptography;


var input = File.ReadAllLines("input").ToList();

var width = input[0].Count();
var height = input.Count;

var start = GetMapPos('S');
var end = GetMapPos('E');
var frontier = new PriorityQueue<(Pos, Dir), int>();
frontier.Enqueue((start, (1, 0)), 0);
var cost = new Dictionary<(Pos, Dir), int>();
cost.Add((start, (1, 0)), 0);

Pos[] dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

while (frontier.Count > 0) {
    var curr = frontier.Dequeue();
    foreach (var dir in dirs) {
        Pos next = (curr.Item1.x + dir.x, curr.Item1.y + dir.y);
        if (next.x < 0 || next.y < 0 || next.x >= width || next.y >= height)
            continue;
        if (input[next.y][next.x] == '#')
            continue;
        var newCost = cost[curr] + 1 + Cost(curr.Item2, dir);
        if (!cost.ContainsKey((next, dir)) || newCost < cost[(next, dir)]) {
            cost[(next, dir)] = newCost;
            frontier.Enqueue((next, dir), newCost);
        }
    }
}

int result = int.MaxValue;
Dir endDir = (1, 0);
foreach (var dir in dirs) {
    if (cost.ContainsKey((end, dir)) && cost[(end, dir)] < result) {
        result = cost[(end, dir)];
        endDir = dir;
    }
}

var currL = new List<(Pos, int)>() { (end, result) };
var places = new HashSet<Pos>();
// moving backward from end to start 
while (true) {
    var newL = new List<(Pos, int)>();
    // Console.WriteLine($"###{currL.Count()}");
    foreach (var (curr, smallest) in currL) {
        places.Add(curr);
        if (curr == start) continue;
        // Console.WriteLine($"######{curr}, {smallest}");
        var list = new List<(Pos, Dir)>();
        foreach (var dir in dirs) {
            if (cost.ContainsKey((curr, dir))) {
                // next spot needs to be smaller 
                // we check the modulo part to not get wrong loops where the cost is for a short time smaller
                // this only works because the map is not big enough for the real steps to reach 1k
                if (cost[(curr, dir)] <= smallest && cost[(curr, dir)] % 1000 <= smallest % 1000) {
                    newL.Add(((curr.x - dir.x, curr.y - dir.y), cost[(curr, dir)]));
                }
            }
        }
    }
    currL = newL;
    if (currL.Count() == 0) break;
}

Console.WriteLine(result);
Console.WriteLine(places.Count());

void PrintMap() {
    for (var y = 0; y < height; y++) {
        for (var x = 0; x < width; x++) {
            if (input[y][x] != '#') {
                Console.Write(places.Contains((x, y)) ? 'O' : input[y][x]);
            } else
                Console.Write(input[y][x]);
        }
        Console.WriteLine();
    }
}


int Cost(Dir dir1, Dir dir2) {
    if (dir1 == dir2) return 0;
    if (dir1.x != dir2.x && dir1.y != dir2.y) return 1000;
    return 2000;
}


Pos GetMapPos(char c) {
    for (var y = 0; y < height; y++) {
        for (var x = 0; x < width; x++) {
            if (input[y][x] == c) return (x, y);
        }
    }
    throw new Exception();
}



