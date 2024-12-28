using System.IO;
using System;
using System.Text.RegularExpressions;

var input = File.ReadAllLines("input").ToList();

// string pattern = @"mul\((\d{1,3}),(\d{1,3})\)";
string pattern = @"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)";

RegexOptions options = RegexOptions.Multiline;
var count = 0;
var count2 = 0;
var ok = true;
foreach (var line in input) {

    foreach (Match m in Regex.Matches(line, pattern, options)) {
        if (m.Value == "don't()") {
            ok = false;
        } else if (m.Value == "do()") {
            ok = true;
        } else {
            var v = Convert.ToInt32(m.Groups[1].ToString()) * int.Parse(m.Groups[2].ToString());
            count += v;
            if (ok) {
                count2 += v;
            }
        }
    }
}
Console.WriteLine(count);
Console.WriteLine(count2);
