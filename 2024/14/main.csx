using System.IO;
using System;
using System.Text.RegularExpressions;
using System.Linq;
using System.Threading;
using System.Security.Cryptography;
using System.Reflection.Metadata.Ecma335;
using System.ComponentModel;
using System.Data;

var input = File.ReadAllLines("input").ToList();

record struct Vec2(int x, int y) {
    public static Vec2 operator +(Vec2 a, Vec2 b)
     => new Vec2(a.x + b.x, a.y + b.y);
};
record struct Robot(Vec2 pos, Vec2 dir);

var robots = new List<Robot>();

foreach (var line in input) {
    // Console.WriteLine(line);
    var nums = Regex.Matches(line, @"-?\d+")
        .Select(m => int.Parse(m.Value)).ToArray();
    var robot = new Robot(new Vec2(nums[0], nums[1]), new Vec2(nums[2], nums[3]));
    robots.Add(robot);
}

var width = 11;
width = 101;
var height = 7;
height = 103;
var SECONDS = 100;

var counts = new int[2, 2]{
    {0,0},
    {0,0},
};

for (var i = 0; i < robots.Count; i++) {
    var robot = Move(robots[i], width, height, 100);
    // robots[i] = robot;
    if (robot.pos.x == width / 2) continue;
    if (robot.pos.y == height / 2) continue;
    var xC = robot.pos.x > width / 2 ? 0 : 1;
    var yC = robot.pos.y > height / 2 ? 0 : 1;
    counts[xC, yC]++;
}

var sum = 1;
foreach (var c in counts)
    sum *= c;

Console.WriteLine(sum);

// part2 output print to file and check manual
using (var outfile = new StreamWriter("./File.txt")) {
    var s = 0;
    while (s < 10000) {
        for (var i = 0; i < robots.Count; i++) {
            robots[i] = Move(robots[i], width, height, 1);
        }
        s++;
        // Console.WriteLine(s);
        outfile.WriteLine(s);
        outfile.Write(GetField(robots, width, height));
    }

}

Robot Move(Robot robot, int width, int height, int seconds) {
    for (var s = 0; s < seconds; s++) {
        var (x, y) = robot.pos + robot.dir;
        if (y < 0) {
            y += height;
        } else if (y >= height) {
            y %= height;
        }
        if (x < 0) {
            x += width;
        } else if (x >= width) {
            x %= width;
        }
        robot.pos = new Vec2(x, y);
    }
    return robot;
}

string GetField(List<Robot> robots, int width, int height) {
    var sb = new StringBuilder();
    var hs = new HashSet<(int x, int y)>();
    foreach (var r in robots) {
        hs.Add((r.pos.x, r.pos.y));
    }

    for (var y = 0; y < height; y++) {
        for (var x = 0; x < width; x++) {
            // var counts = robots
            // .Where(robot => robot.pos.x == x && robot.pos.y == y)
            // .Count();
            sb.Append(hs.Contains((x, y)) ? '#' : ' ');
            // Console.Write(counts > 0 ? counts.ToString() : '.');
        }
        sb.AppendLine();
        // Console.Write('\n');
    }
    return sb.ToString();
}
