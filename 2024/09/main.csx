using System.IO;
using System;
using System.Text.RegularExpressions;

var input = File.ReadAllLines("input").ToList();
var line = input[0];

var id = 0;
var temp = new List<int?>();
var file = true;
for (var i = 0; i < line.Length; i++) {
    var length = int.Parse(line[i].ToString());
    if (file) {
        temp.AddRange(Enumerable.Repeat<int?>(id++, length));
    } else {
        temp.AddRange(Enumerable.Repeat<int?>(null, length));
    }
    file = !file;
}

for (var i = 0; i < temp.Count; i++) {
    var t = temp[i];
    if (t == null) {
        int? item;
        int index;
        do {
            index = temp.Count - 1;
            item = temp[index];
            temp.RemoveAt(index);
        } while (item == null && index > i);
        if (item == null) {
            break;
        }
        temp[i] = item;
        t = item;
    }
}
long count = 0;
for (var i = 0; i < temp.Count; i++) {
    if (temp[i] == null) Console.WriteLine("WOW");
    count += i * temp[i] ?? 0;
}

Console.WriteLine(count);


file = true;
id = 0;
var ttr1 = new List<(int length, int? value, bool moved)>();
for (var i = 0; i < line.Length; i++) {
    var length = int.Parse(line[i].ToString());
    ttr1.Add((length, file ? id++ : null, false));
    file = !file;
}


for (var i = ttr1.Count - 1; i > 0; i--) {
    var item = ttr1[i];
    if (item.moved || item.value == null) continue;
    for (var j = 0; j <= i; j++) {
        if (ttr1[j].value == null && ttr1[j].length >= item.length) {
            if (ttr1[j].length == item.length) {
                ttr1[j] = (length: item.length, value: item.value, moved: true);
            } else {
                ttr1[j] = (length: ttr1[j].length - item.length, value: null, moved: false);
                ttr1.Insert(j, (length: item.length, value: item.value, moved: true));
                i++;
            }
            ttr1[i] = (length: item.length, value: null, moved: false);
            break;
        }
    }
}

long count2 = 0;
id = 0;
for (var i = 0; i < ttr1.Count; i++) {
    for (var j = 0; j < ttr1[i].length; j++) {
        count2 += (ttr1[i].value ?? 0) * id++;
    }
}
Console.WriteLine(count2);



