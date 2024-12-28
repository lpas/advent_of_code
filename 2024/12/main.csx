using System.IO;
using System;
using System.Text.RegularExpressions;
using System.Linq;

var input = File.ReadAllLines("input").ToList();

var height = input.Count;
var width = input[0].Length;


var visitedAll = new HashSet<(int x, int y)>();
(int x, int y)[] dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
var count = 0;
var count2 = 0;
for (var y = 0; y < height; y++) {
    for (var x = 0; x < width; x++) {
        if (!visitedAll.Contains((x, y))) {
            var visited = new HashSet<(int x, int y)>();
            var queue = new Queue<(int x, int y)>();
            queue.Enqueue((x, y));
            var v = input[y][x];
            var perimeter = 0;
            while (queue.Count > 0) {
                var curr = queue.Dequeue();
                if (visited.Contains(curr)) continue;
                visited.Add(curr);
                foreach (var dir in dirs) {
                    (int x, int y) next = (curr.x + dir.x, curr.y + dir.y);
                    if (next.x < 0 || next.y < 0
                    || next.x >= width || next.y >= height
                    || input[next.y][next.x] != v) {
                        perimeter++;
                        continue;
                    }
                    queue.Enqueue(next);
                }
            }
            count += visited.Count * perimeter;
            var sides = GetSides(input, visited, v);
            // Console.WriteLine($"{v} {visited.Count} {sides}");
            count2 += visited.Count * sides;



            // Console.WriteLine($"{v} {visited.Count} {perimeter}");
            visitedAll.UnionWith(visited);
        }
    }
}
Console.WriteLine(count);
Console.WriteLine(count2);

int GetSides(List<string> input, HashSet<(int x, int y)> visited, char value) {
    int minX = int.MaxValue, minY = int.MaxValue;
    int maxX = int.MinValue, maxY = int.MinValue;
    foreach (var item in visited) {
        if (minX > item.x) minX = item.x;
        if (minY > item.y) minY = item.y;
        if (maxX < item.x) maxX = item.x;
        if (maxY < item.y) maxY = item.y;
    }

    // Console.WriteLine($"{minX},{minY}  {maxX} {maxY}");
    // Console.WriteLine(value);
    var last = new List<(int, bool)>();
    var count = 0;
    // sweep left to right
    for (var y = minY; y <= maxY; y++) {
        var curr = new List<(int, bool)>();
        bool? lastInside = null;
        for (var x = minX; x <= maxX; x++) {
            var inside = input[y][x] == value && visited.Contains((x, y));
            if ((lastInside == null && inside) || (lastInside != null && inside != lastInside)) {
                curr.Add((x, inside));
                if (!last.Contains((x, inside))) {
                    count++;
                }
            }
            lastInside = inside;
        }
        if (lastInside == true) {
            curr.Add((maxX + 1, true));
            if (!last.Contains((maxX + 1, true))) {
                count++;
            }
        }
        last = curr;
    }
    // sweep top to bottom
    // Console.WriteLine(count);
    last = [];
    for (var x = minX; x <= maxX; x++) {
        var curr = new List<(int, bool)>();
        bool? lastInside = null;
        for (var y = minY; y <= maxY; y++) {
            var inside = input[y][x] == value && visited.Contains((x, y));
            if ((lastInside == null && inside) || (lastInside != null && inside != lastInside)) {
                curr.Add((y, inside));
                if (!last.Contains((y, inside))) {
                    count++;
                }
            }
            lastInside = inside;
        }
        if (lastInside == true) {
            curr.Add((maxY + 1, true));
            if (!last.Contains((maxY + 1, true))) {
                count++;
            }
        }
        last = curr;
    }
    // Console.WriteLine(count);
    return count;
}


