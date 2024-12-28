using System.IO;
using System;
using System.Text.RegularExpressions;
using System.Linq;

var input = File.ReadAllLines("input").ToList();

List<List<byte>> map = input.Select(line =>
    line.ToCharArray().Select((i) => byte.Parse(i.ToString())).ToList()
).ToList();

var width = map[0].Count;
var height = map.Count;

var dirs = new List<(int x, int y)> { (0, 1), (1, 0), (0, -1), (-1, 0) };
var count = 0;
var count2 = 0;
for (var y = 0; y < height; y++) {
    for (var x = 0; x < width; x++) {
        if (map[y][x] == 0) {
            var queue = new Queue<(int x, int y)>();
            var heads = new HashSet<(int x, int y)>();
            queue.Enqueue((x, y));
            while (queue.Count > 0) {
                var c = queue.Dequeue();
                if (map[c.y][c.x] == 9) {
                    heads.Add(c);
                    count2++;
                    continue;
                }
                foreach (var (dx, dy) in dirs) {
                    var nx = c.x + dx;
                    var ny = c.y + dy;
                    if (nx < 0 || ny < 0 || nx >= width || ny >= height) {
                        continue;
                    }
                    if (map[ny][nx] == map[c.y][c.x] + 1) {
                        queue.Enqueue((nx, ny));
                    }
                }
            }
            count += heads.Count;
        }
    }
}

Console.WriteLine(count);
Console.WriteLine(count2);