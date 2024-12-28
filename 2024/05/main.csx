using System.IO;
using System;
using System.Text.RegularExpressions;

var input = File.ReadAllLines("input").ToList();

var first = true;
var pairs = new Dictionary<string, List<string>>();
var count = 0;
var count2 = 0;
foreach (var line in input) {
    if (String.IsNullOrEmpty(line)) {
        first = false;
        continue;
    }


    if (first) {
        var pair = line.Split('|').ToList<string>();
        if (pairs.ContainsKey(pair[0])) {
            pairs[pair[0]].Add(pair[1]);
        } else {
            pairs[pair[0]] = new List<string> { pair[1] };
        }
    } else {
        var before = new List<string> { };
        var ok = true;
        var split = line.Split(',').ToList();
        foreach (var item in split) {
            if (pairs.ContainsKey(item)) {
                if (pairs[item].Intersect(before).Count() > 0) {
                    ok = false;
                    break;
                }
            }
            before.Add(item);
        }
        if (ok) {
            // part1
            var v = split[split.Count() / 2];
            count += int.Parse(v);
        } else {
            //part2
            for (var i = 0; i < split.Count(); i++) {
                if (pairs.ContainsKey(split[i])) {
                    var values = pairs[split[i]];
                    for (var j = 0; j < i; j++) {
                        if (values.Contains(split[j])) {
                            var v = split[i];
                            split.RemoveAt(i);
                            split.Insert(j, v);
                            i = 0; // dirty start again we moved items
                        }
                    }
                };
            }
            count2 += int.Parse(split[split.Count() / 2]);
        }


    }

}
Console.WriteLine(count);
Console.WriteLine(count2);




