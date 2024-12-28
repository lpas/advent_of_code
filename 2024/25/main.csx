var input = File.ReadAllLines("input").ToList();

var keys = new List<int[]>();
var locks = new List<int[]>();

var firstLine = true;
var curr = new int[5];
var isLock = true;
foreach (var line in input) {
    if (line == "") {
        firstLine = true;
        if (isLock) {
            locks.Add(curr);
        } else {
            for (var i = 0; i < 5; i++) {
                curr[i]--;
            }
            keys.Add(curr);
        }
        // Console.WriteLine(string.Join(',', curr));
        curr = new int[5];
        continue;
    }
    if (firstLine) {
        firstLine = false;
        isLock = line.All((item) => item == '#');
        continue;
    }
    for (var i = 0; i < 5; i++) {
        curr[i] += line[i] == '#' ? 1 : 0;
    }
}
if (isLock) {
    locks.Add(curr);
} else {
    for (var i = 0; i < 5; i++) {
        curr[i]--;
    }
    keys.Add(curr);
}


var count = 0;
foreach (var key in keys) {
    foreach (var l in locks) {
        if (key.Zip(l).All(item => item.First + item.Second <= 5)) {
            count++;
        }
    }
}

Console.WriteLine(count);





