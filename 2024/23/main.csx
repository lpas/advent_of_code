

using System.Data;

var input = File.ReadAllLines("input").ToList();

var pcs = new HashSet<string>();
var connection = new Dictionary<string, HashSet<string>>();

foreach (var line in input) {
    var split = line.Split('-');
    var pc1 = split[0];
    var pc2 = split[1];
    pcs.Add(pc1);
    pcs.Add(pc2);

    if (!connection.ContainsKey(pc1)) connection[pc1] = new HashSet<string>();
    if (!connection.ContainsKey(pc2)) connection[pc2] = new HashSet<string>();
    connection[pc1].Add(pc2);
    connection[pc2].Add(pc1);
}

var set = new HashSet<(string, string, string)>();

foreach (var pc in pcs) {
    if (pc[0] != 't') continue;
    var q = from a in connection[pc]
            from b in connection[pc]
            where a != b
            && connection[a].Contains(pc) && connection[a].Contains(b)
            && connection[b].Contains(pc) && connection[b].Contains(a)
            select (pc, a, b);

    foreach (var (a, b, c) in q) {
        string[] x = [a, b, c];
        Array.Sort(x);
        var item = (x[0], x[1], x[2]);
        set.Add(item);
    }
}
Console.WriteLine(set.Count());

// part2 
var checkedStr = new HashSet<string>();
var longest = 2;
var bestStr = "00,00";
foreach (var pc in pcs) {
    var l = new List<string>(connection[pc]) { pc };
    l.Sort();
    var permutations = new List<List<string>>();
    // need only permutations larger than our current largest one
    for (var i = longest; i < l.Count(); i++) {
        foreach (var x in GetPermutations(l, i)) {
            permutations.Add(x.ToList());
        }
    }
    // check if permutation is fully connected
    foreach (var p in permutations) {
        var str = string.Join(",", p);
        if (checkedStr.Contains(str)) continue;
        checkedStr.Add(str);
        var t = p.All((a) => p.All(b => a == b || connection[a].Contains(b)));
        if (t && p.Count() > longest) {
            longest = p.Count();
            bestStr = str;
        }
    }
}
Console.WriteLine(bestStr);

IEnumerable<IEnumerable<T>> GetPermutations<T>(IEnumerable<T> items, int count) {
    int i = 0;
    foreach (var item in items) {
        if (count == 1)
            yield return new T[] { item };
        else {
            foreach (var result in GetPermutations(items.Skip(i + 1), count - 1))
                yield return new T[] { item }.Concat(result);
        }
        ++i;
    }
}