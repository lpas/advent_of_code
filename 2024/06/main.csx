using VD = System.Collections.Generic.HashSet<(int x, int y, (int, int) dir)>;

var input = File.ReadAllLines("input").ToList();
var width = input[0].Length;
var height = input.Count;
var (startX, startY) = GetStartPos(input);
var (_, visited) = Walk(startX, startY);
var v = new HashSet<(int, int)>(visited.Select(item => (item.x, item.y)));
Console.WriteLine(v.Count);

var count2 = 0;
for (var y = 0; y < height; y++) {
    for (var x = 0; x < width; x++) {
        if (input[y][x] == '.') {
            var (loop, _) = Walk(startX, startY, x, y);
            if (loop) {
                count2++;
            }
        }
    }
}
Console.WriteLine(count2);

(bool, VD) Walk(int x, int y, int xB = -1, int yB = -1) {
    var visitedDirections = new VD();
    var dirs = new List<(int x, int y)> { (0, -1), (1, 0), (0, 1), (-1, 0) };
    var currDir = 0;
    visitedDirections.Add((x, y, dirs[currDir]));
    while (true) {
        var nX = x + dirs[currDir].x;
        var nY = y + dirs[currDir].y;
        if (nX < 0 || nY < 0 || nX >= width || nY >= height) {
            return (false, visitedDirections);
        }
        if (input[nY][nX] == '#' || (nY == yB && nX == xB)) {
            currDir = (currDir + 1) % 4;
            continue;
        }
        if (visitedDirections.Contains((nX, nY, dirs[currDir]))) {
            return (true, visitedDirections);
        }
        x = nX;
        y = nY;
        visitedDirections.Add((x, y, dirs[currDir]));
    }
}

void Print(int xO, int yO) {
    for (var y = 0; y < width; y++) {
        for (var x = 0; x < height; x++) {
            Console.Write(xO == x && yO == y ? 'O' : input[y][x]);
        }
        Console.WriteLine();
    }
}

(int x, int y) GetStartPos(List<string> input) {
    for (var y = 0; y < width; y++) {
        for (var x = 0; x < height; x++) {
            if (input[y][x] == '^') {
                return (x, y);
            }
        }
    }
    throw new Exception();
}