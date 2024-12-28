using System.IO;


var input = File.ReadAllLines("input").ToList();
var count = 0;
var count2 = 0;

foreach (var line in input) {
    var s = line.Split(' ', StringSplitOptions.RemoveEmptyEntries);
    var l = new List<string>(s).ConvertAll<int>(item => int.Parse(item));

    if (task1(l)) {
        count++;
    }
    if (task2(l)) {
        count2++;
    }

}


Console.WriteLine(count);
Console.WriteLine(count2);

private static bool task1(List<int> l) {
    var dec = l[1] < l[0];

    for (var i = 0; i < l.Count - 1; i++) {
        if (!((dec && l[i] > l[i + 1] || !dec && l[i] < l[i + 1])
            && Math.Abs(l[i] - l[i + 1]) <= 3)) {
            return false;
        }
    }
    return true;
}

private static bool task2(List<int> l) {
    if (task1(l)) return true;
    for (var i = 0; i < l.Count; i++) {
        var l2 = new List<int>(l);
        l2.RemoveAt(i);
        if (task1(l2)) return true;
    }

    return false;

}