
using Dict = System.Collections.Generic.Dictionary<(int, int, int, int), int>;

var input = File.ReadAllLines("input").ToList();

long Step(long num) {
    num = ((num * 64) ^ num) % 16777216;
    num = ((num / 32) ^ num) % 16777216;
    num = ((num * 2048) ^ num) % 16777216;
    return num;
}

long Sim(long num, int steps) {
    for (var i = 0; i < steps; i++) {
        num = Step(num);
    }
    return num;
}

// part1
long result = 0;
foreach (var line in input) {
    result += Sim(long.Parse(line), 2000);
}
Console.WriteLine(result);

// part2 
var dict = new Dict();
foreach (var line in input) {
    foreach (var d in GetDict(long.Parse(line))) {
        if (dict.ContainsKey(d.Key)) {
            dict[d.Key] += d.Value;
        } else {
            dict[d.Key] = d.Value;
        }
    }
}
Console.WriteLine(dict.Values.Max());

Dict GetDict(long num) {
    long? last = null;
    var queue = new Queue<int>();
    var dict = new Dict();
    for (var i = 0; i < 2000; i++) {
        num = Step(num);
        var oneDigit = (int)(num % 10);
        var diff = oneDigit - last;
        last = oneDigit;
        if (diff != null) {
            queue.Enqueue((int)diff);
            if (queue.Count() > 4) {
                queue.Dequeue();
            }
        }
        if (queue.Count() == 4) {
            var key = (queue.ElementAt(0), queue.ElementAt(1), queue.ElementAt(2), queue.ElementAt(3));
            if (!dict.ContainsKey(key)) {
                dict[key] = oneDigit;
            }
        }
    }
    return dict;
}