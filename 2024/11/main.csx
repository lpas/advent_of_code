using System.IO;
using System;
using System.Text.RegularExpressions;
using System.Linq;

var input = File.ReadAllLines("input").ToList();
var line = input[0];

var numbers = line.Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).ToList();

// foreach (var n in numbers) {
//     Console.Write($"{n} ");
// }

var numbs = new Dictionary<long, long>();
foreach (var num in numbers) {
    numbs.Add(num, 1);
}

const int BLINKS = 75;
for (var b = 1; b <= BLINKS; b++) {
    var temp = new Dictionary<long, long>();
    foreach (var num in numbs) {
        if (num.Key == 0) {
            temp.TryGetValue(1, out var currentCount);
            temp[1] = currentCount + num.Value;
        } else {
            var s = num.Key.ToString();
            var l = s.Length;
            if (l % 2 == 0) {
                var v1 = long.Parse(s.Substring(0, l / 2));
                var v2 = long.Parse(s.Substring(l / 2));
                temp.TryGetValue(v1, out var currentCount);
                temp[v1] = currentCount + num.Value;
                temp.TryGetValue(v2, out currentCount);
                temp[v2] = currentCount + num.Value;
            } else {
                var v1 = num.Key * 2024;
                temp.TryGetValue(v1, out var currentCount);
                temp[v1] = currentCount + num.Value;
            }
        }
    }
    numbs = temp;
}

long count = 0;
foreach (var n in numbs) {
    count += n.Value;
}

Console.WriteLine(count);

// const int BLINKS = 25;
// for (var b = 1; b <= BLINKS; b++) {
//     for (var i = 0; i < numbers.Count; i++) {
//         if (numbers[i] == 0) {
//             numbers[i] = 1;
//         } else {
//             var s = numbers[i].ToString();
//             var l = s.Length;
//             if (l % 2 == 0) {
//                 numbers[i] = long.Parse(s.Substring(0, l / 2));
//                 numbers.Insert(i + 1, long.Parse(s.Substring(l / 2)));
//                 i++;
//             } else {
//                 numbers[i] *= 2024;
//             }
//         }
//     }
//     // Console.WriteLine(b);
//     // Console.Write('\n');
//     // foreach (var n in numbers) {
//     //     Console.Write($"{n} ");
//     // }
// }

// Console.WriteLine(numbers.Count);