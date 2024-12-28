using System.IO;

var input = File.ReadAllLines("input").ToList();
var list1 = new List<int>();
var list2 = new List<int>();

foreach (var line in input) {
    var s = line.Split(' ', StringSplitOptions.RemoveEmptyEntries);
    list1.Add(int.Parse(s[0]));
    list2.Add(int.Parse(s[1]));
}
// task 1
list1.Sort();
list2.Sort();

int count = 0;

for (var i = 0; i < list1.Count; i++) {
    count += Math.Abs(list1[i] - list2[i]);
}

Console.WriteLine(count);

// task 2
int count2 = 0;
foreach (var item in list1) {
    int c = 0;
    foreach (var item2 in list2) {
        if (item == item2) c++;
    }
    count2 += item * c;
}


Console.WriteLine(count2);

// task 2 with linq
int count3 = list1
    .Select(item => list2.FindAll(f => f == item).Count * item)
    .Aggregate<int>((sum, c) => sum + c);
Console.WriteLine(count3);