using wire = (string v1, string gate, string v2, string result);
var input = File.ReadAllLines("input1").ToList();


var initialValues = new Dictionary<string, bool>();
var wires = new Queue<wire>();
var wireDict = new Dictionary<string, (string v1, string v2, string gate)>();
var gates = false;

var c = 0;
foreach (var line in input) {
    if (line == "") {
        gates = true;
        continue;
    }
    if (!gates) {
        var x = line.Split(":");
        initialValues[x[0]] = x[1].Trim().Equals("1");
    } else {
        var x = line.Split(' ');
        wires.Enqueue((x[0], x[1], x[2], x[4]));
        wireDict[x[4]] = (x[0], x[2], x[1]);
        // .dot graphviz
        // c++;
        // var t = x[1] + c.ToString();

        // Console.WriteLine($"{t} [shape=invtriangle, label=\"{x[1]}\"]");
        // Console.WriteLine($"{x[0]} ->  {t}");
        // Console.WriteLine($"{x[2]} ->  {t}");
        // Console.WriteLine($"{t} ->  {x[4]}");


    }
}

var values = new Dictionary<string, bool>(initialValues);




// var zeroValues = new Dictionary<string, bool>(initialValues);
// foreach (var k in zeroValues.Keys) {
//     zeroValues[k] = false;
// }

var zKeys = GetKeys('z');
var xKeys = GetKeys('x');
var yKeys = GetKeys('y');





Console.WriteLine(GetValue(zKeys));

// part2 errors solve manually with graphviz
// this prints out which z is wrong to find errors faster
// the graph should be 43 full adder
// for (var x = 1; x < 44; x++) {
//     foreach (var a in FF()) {
//         foreach (var b in FF()) { 
//             var should = (a.v1 && a.v2) ^ (b.v1 ^ b.v2); // reset everything to zero to make the carry easier 
//             values = new Dictionary<string, bool>(zeroValues);
//             values['x' + (x - 1).ToString("D2")] = a.v1;
//             values['y' + (x - 1).ToString("D2")] = a.v2;
//             values['x' + x.ToString("D2")] = b.v1;
//             values['y' + x.ToString("D2")] = b.v2;

//             var z = 'z' + x.ToString("D2");
//             var v = GetV(z);
//             var v0 = GetV('z' + (x - 1).ToString("D2"));
//             if (should != v) {
//                 Console.WriteLine(x.ToString("D2"));
//                 Console.WriteLine($"{a.v1 && a.v2} {b} {v} {should} ||| {v == should}");
//             }
//         }
//     }
// }
var l = new List<string>() { "z11", "rpv", "ctg", "rpb", "dmh", "z31", "z38", "dvq" };
l.Sort();
Console.WriteLine(string.Join(',', l));

bool GetV(string key) {
    // Console.WriteLine(key);
    if (!values.ContainsKey(key)) {
        var (v1, v2, gate) = wireDict[key];
        // Console.WriteLine($"{key} = {v1} {gate} {v2}");
        values[key] = gate switch {
            "AND" => GetV(v1) && GetV(v2),
            "OR" => GetV(v1) || GetV(v2),
            "XOR" => GetV(v1) ^ GetV(v2),
            _ => throw new Exception(),
        };
    }
    return values[key];
}

List<string> GetKeys(char c) {
    var keys = initialValues.Keys.Union(wireDict.Keys).Where(item => item[0] == c).ToList();
    keys.Sort();
    keys.Reverse();
    return keys;
}

UInt64 GetValue(List<string> keys) {
    var str = new string(keys.Select(item => GetV(item) ? '1' : '0').ToArray());
    return Convert.ToUInt64(str, 2);
}



bool[] TF = [false, true];
IEnumerable<(bool v1, bool v2)> FF() {
    foreach (var tf1 in TF) {
        foreach (var tf2 in TF) {
            yield return (tf1, tf2);
            // Console.WriteLine($"{tf1} {tf2}");
        }
    }
}