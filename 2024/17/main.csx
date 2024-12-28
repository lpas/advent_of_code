using System.IO;
using System;
using System.Text.RegularExpressions;
using System.Linq;
using System.Data;
using System.Diagnostics.Metrics;


var input = File.ReadAllLines("input").ToList();

var A = long.Parse(input[0].Split(':')[1]);
var B = long.Parse(input[1].Split(':')[1]);
var C = long.Parse(input[2].Split(':')[1]);

var program = input[4].Split(':')[1].Split(',').Select(int.Parse).ToList();

var output = RunProgram(A, B, C, program);
Console.WriteLine(string.Join(',', output));

List<int> RunProgram(long A, long B, long C, List<int> program) {
    var output = new List<int>();
    var pointer = 0;

    while (true) {
        if (pointer >= program.Count()) break;

        var opcode = program[pointer];
        var operand = program[pointer + 1];

        long V = operand;
        if (V > 3 && V < 7) {
            V = V switch {
                4 => A,
                5 => B,
                6 => C,
                _ => throw new Exception(),
            };
        }


        switch (opcode) {
            case 0: // adv
                A = (int)(A / Math.Pow(2, V));
                break;
            case 1: //bxl
                B = B ^ operand;
                break;
            case 2: // bst
                B = BitConverter.GetBytes(V % 8)[0] & ((1 << 3) - 1);
                break;
            case 3: //jnz
                if (A != 0) {
                    pointer = (int)operand;
                    continue;
                }
                break;
            case 4: // bcx
                B = B ^ C;
                break;
            case 5: // out
                output.Add((int)(V % 8));
                break;
            case 6: //bdv
                B = (int)(A / Math.Pow(2, V));
                break;
            case 7: // cdv
                C = (int)(A / Math.Pow(2, V));
                break;
        }
        pointer += 2;
    }
    return output;
}

// hand crafted code from my input program
List<int> Run(long A) {
    var output = new List<int>();
    while (true) {
        long B = BitConverter.GetBytes(A % 8)[0] & ((1 << 3) - 1) ^ 3;
        B = B ^ 5 ^ (long)(A / Math.Pow(2, B));
        A /= 8;
        output.Add((int)(B % 8));
        if (A == 0) break;
    }
    return output;
}
// A depends only on multiples of 8
var curr = new List<long>() { 0 };
for (var i = program.Count() - 1; i >= 0; i--) {
    var ops = new List<long>();
    foreach (var x in curr) {
        for (var j = 0; j < 8; j++) {
            long v = x * 8 + j;
            if (Run(v).SequenceEqual(program[i..])) {
                ops.Add(v);
            }
        }
    }
    curr = ops;
}

Console.WriteLine(curr.Min());
