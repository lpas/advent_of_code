using System.IO;
using System;
using System.Text.RegularExpressions;

var input = File.ReadAllLines("input").ToList();

long c = 0;
foreach (var line in input) {
    var s = line.Split(':');
    var result = long.Parse(s[0]);
    var remaining = s[1].Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).ToList();

    if (test(result, remaining)) {
        c += result;
    }
}
Console.WriteLine(c);

bool test(long r, List<long> values) {
    if (values.Count == 1) {
        return r == values[0];
    }
    var v1 = values[0];
    values.RemoveAt(0);
    var l1 = new List<long>(values);
    l1[0] += v1;
    var t = test(r, l1);
    if (t == true) return true;
    var l2 = new List<long>(values);
    l2[0] *= v1;
    t = test(r, l2);
    if (t == true) return true;
    // part2
    var l3 = new List<long>(values);
    l3[0] = long.Parse(v1.ToString() + l3[0].ToString());
    t = test(r, l3);
    if (t == true) return true;
    return false;


}
