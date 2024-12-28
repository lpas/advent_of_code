using System.IO;
using System;
using System.Text.RegularExpressions;

var input = File.ReadAllLines("input").ToList();

var height = input.Count;
var width = input[0].Length;

int[,] directions = { { 1, 0 }, { -1, 0 }, { 0, 1 }, { 0, -1 }, { -1, -1 }, { -1, 1 }, { 1, 1 }, { 1, -1 } };
var count = 0;
for (var y = 0; y < height; y++) {
    for (var x = 0; x < width; x++) {
        if (input[y][x] == 'X') {
            for (var i = 0; i < directions.GetLength(0); i++) {
                if (check1(input, x, y, [directions[i, 0], directions[i, 1]])) {
                    count++;
                }
            }
        }
    }
}
Console.WriteLine(count);



var count2 = 0;
for (var y = 0; y < height; y++) {
    for (var x = 0; x < width; x++) {
        if (input[y][x] == 'A') {
            if (
                (check(input, x - 1, y - 1, 'M') && check(input, x + 1, y + 1, 'S')
                 || check(input, x - 1, y - 1, 'S') && check(input, x + 1, y + 1, 'M')
                )
                &&
                 (check(input, x + 1, y - 1, 'M') && check(input, x - 1, y + 1, 'S')
                 || check(input, x + 1, y - 1, 'S') && check(input, x - 1, y + 1, 'M')
                )
            ) {
                count2++;
            }


        }
    }
}
Console.WriteLine(count2);


private static bool check1(List<string> input, int x, int y, int[] dir) {
    var xmas = "XMAS";
    foreach (var c in xmas) {
        if (!check(input, x, y, c)) {
            return false;
        }
        x += dir[0];
        y += dir[1];
    }


    return true;

}

private static bool check(List<string> input, int x, int y, char c) {
    var height = input.Count;
    var width = input[0].Length;
    if (x < 0 || x >= width || y < 0 || y >= height) {
        return false;
    }
    return input[y][x] == c;
}









