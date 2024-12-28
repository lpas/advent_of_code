var input = File.ReadAllLines("input").ToList();

var numpad = new List<string>() { "789", "456", "123", "#0A" };
var dirpad = new List<string>() { "#^A", "<v>" };
var cache = new Dictionary<(string, int), long>();

Complexity(2);
Complexity(25);
void Complexity(int keypadsCount) {
    var items = input.Select(line => {
        var count =
            ToMoves(numpad, line).Select(item =>
                item.Select(item => RoboMove(item, keypadsCount)
            ).Min()
        ).Sum();

        return count * int.Parse(line[..3]);
    });

    Console.WriteLine(items.Sum());
}

long RoboMove(string item, int depth) {
    if (cache.ContainsKey((item, depth))) return cache[(item, depth)];
    var r = ToMoves(dirpad, item)
        .Select((item) =>
            (depth == 1
                ? item.Select(item => (long)item.Count())
                : item.Select(item => RoboMove(item, depth - 1))
            )
            .Min()
        ).Sum();
    cache[(item, depth)] = r;
    return r;
}

List<List<string>> ToMoves(List<string> pad, string str) {
    var list = new List<List<string>>();
    var first = 'A';
    foreach (var s in str) {
        list.Add(Moves(pad, first, s).Select((item) => item + 'A').ToList());
        first = s;
    }
    return list;
}

List<string> Moves(List<string> grid, char startChar, char endChar) {
    var dirs = new Dictionary<(int x, int y), char>()
        {
            {(1,0), '>'},
            {(-1,0), '<'},
            {(0,1), 'v'},
            {(0,-1), '^'},
        }
    ;
    var width = grid[0].Length;
    var height = grid.Count();
    var start = GetPos(startChar);
    var end = GetPos(endChar);

    var queue = new Queue<((int x, int y) pos, List<char> list, HashSet<(int x, int y)> set)>();
    queue.Enqueue((start, new List<char>(), new HashSet<(int x, int y)>() { start }));
    var results = new List<List<char>>();
    var shortest = int.MaxValue;
    while (queue.TryDequeue(out var curr)) {
        if (curr.pos == end) {
            if (curr.list.Count() <= shortest) {
                if (curr.list.Count() < shortest) {
                    results.Clear();
                    shortest = curr.list.Count();
                }
                results.Add(curr.list);
            }
            continue;
        }

        foreach (var dir in dirs) {
            (int x, int y) next = (curr.pos.x + dir.Key.x, curr.pos.y + dir.Key.y);
            if (!ValidPos(next) || grid[next.y][next.x] == '#' || curr.set.Contains(next))
                continue;
            queue.Enqueue((next,
                new List<char>(curr.list) { dir.Value },
                new HashSet<(int x, int y)>(curr.set) { next }
            ));
        }
    }
    return results.Select(item => new string(item.ToArray())).ToList();
    bool ValidPos((int x, int y) pos) {
        if (pos.x < 0 || pos.y < 0 || pos.x >= width || pos.y >= height)
            return false;
        return true;
    }

    (int x, int y) GetPos(char c) {
        for (var y = 0; y < height; y++) {
            for (var x = 0; x < width; x++) {
                if (grid[y][x] == c) return (x, y);
            }
        }
        throw new Exception($"{c} not found");
    }
}

/**
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+    
*/