using System.IO;
using System;
using System.Text.RegularExpressions;
using System.Linq;
using System.Data;
using System.Collections.Concurrent;

var input = File.ReadAllLines("input").ToList();

var towels = input[0]
    .Split(',', StringSplitOptions.RemoveEmptyEntries)
    .Select(s => s.Trim())
    .ToList();

var memo = new Dictionary<string, long>();
var memo2 = new ConcurrentDictionary<string, long>();

var stream = input.Skip(2).Select(Find2);

Console.WriteLine(stream.Count(v => v > 0));
Console.WriteLine(stream.Sum());

long Find(string design) {
    if (design.Count() == 0) return 1;
    if (memo.ContainsKey(design)) return memo[design];
    memo[design] = 0;
    foreach (var towel in towels) {
        var length = towel.Count();
        if (length <= design.Count() && design[..length] == towel) {
            var c = Find(design[length..]);
            memo[design] += c;
        }
    }
    return memo[design];
}

// linq style to write the find
long Find2(string design) =>
    memo2.GetOrAdd(design, (design) =>
        design switch {
            "" => 1,
            _ => towels
                .Where(design.StartsWith)
                .Sum(towel => Find(design[towel.Length..]))
        }
    );