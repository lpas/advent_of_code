using System.IO;
using System;
using System.Text.RegularExpressions;
using System.Linq;
using System.Threading;

var input = File.ReadAllLines("input").ToList();
long count = 0;
// for (var i = 0; i < input.Count; i += 4) {
//     var A = input[i].Split(':')[1].Split(',')
//         .Select(item => long.Parse(item.Split('+')[1]))
//         .ToList();
//     var B = input[i + 1].Split(':')[1].Split(',')
//         .Select(item => long.Parse(item.Split('+')[1]))
//         .ToList();
//     var price = input[i + 2].Split(':')[1].Split(',')
//         .Select(item => long.Parse(item.Split('=')[1]))
//         .ToList();

//     int? small = null;
//     for (var a = 1; a < 100; a++) {
//         for (var b = 1; b < 100; b++) {
//             if (A[0] * a + B[0] * b == price[0] && A[1] * a + B[1] * b == price[1]) {
//                 var cost = a * 3 + b;
//                 if (small == null || small > cost) {
//                     small = cost;
//                 }
//             }
//         }
//     }

//     count += small ?? 0;
//     // Console.WriteLine($"{A[0]} {A[1]}, {price[0]} {small}");
// }

// Console.WriteLine(count);



record struct Vec2(long x, long y);
long Det(Vec2 a, Vec2 b) => a.x * b.y - a.y * b.x;


// var count = 0;
for (var i = 0; i < input.Count; i += 4) {
    var A = input[i].Split(':')[1].Split(',')
        .Select(item => long.Parse(item.Split('+')[1]))
        .ToList();
    var B = input[i + 1].Split(':')[1].Split(',')
        .Select(item => long.Parse(item.Split('+')[1]))
        .ToList();
    var price = input[i + 2].Split(':')[1].Split(',')
        .Select(item => long.Parse(item.Split('=')[1]))
        .ToList();
    // todo!!
    var a = new Vec2(A[0], A[1]);
    var b = new Vec2(B[0], B[1]);
    var p = new Vec2(price[0] + 10000000000000, price[1] + 10000000000000);
    var m = Det(p, b) / Det(a, b);
    var n = Det(a, p) / Det(a, b);

    if (m >= 0 && n >= 0 && a.x * m + b.x * n == p.x && a.y * m + b.y * n == p.y) {
        count += 3 * m + n;
    }

    // count += small ?? 0;
    // Console.WriteLine($"{A[0]} {A[1]}, {price[0]} {small}");
}

Console.WriteLine(count);
