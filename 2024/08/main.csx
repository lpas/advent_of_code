using System.IO;
using System;
using System.Text.RegularExpressions;

var input = File.ReadAllLines("input").ToList();


var height = input.Count;
var width = input[0].Length;

var signals = new Dictionary<char, List<(int x, int y)>>();

for (var y = 0; y < height; y++) {
    for (var x = 0; x < width; x++) {
        var c = input[y][x];
        if (c != '.') {
            if (!signals.ContainsKey(c)) {
                signals[c] = new List<(int, int)>();
            }
            signals[c].Add((x, y));
        }
    }
}

var values = new HashSet<(int x, int y)>();

foreach (var signal in signals) {
    var value = signal.Value;
    for (var i = 0; i < value.Count; i++) {
        for (var j = i + 1; j < signal.Value.Count; j++) {

            var v1 = value[i];
            var v2 = value[j];
            values.Add(v1);
            values.Add(v2);
            (int x, int y) d = (v1.x - v2.x, v1.y - v2.y);


            (int x, int y) t = (v1.x + d.x, v1.y + d.y);
            while (t.x >= 0 && t.y >= 0 && t.x < width && t.y < height) {
                values.Add(t);
                t.x += d.x;
                t.y += d.y;
            }
            t = (v2.x - d.x, v2.y - d.y);
            while (t.x >= 0 && t.y >= 0 && t.x < width && t.y < height) {
                values.Add(t);
                t.x -= d.x;
                t.y -= d.y;
            }

        }
    }
}

// 0,0 : 5,0 : 6,2 : 9,3 : 2,4 : 3,6 : 4,8
foreach (var v in values) {
    // Console.Write(v);
}

Console.WriteLine(values.Count);
