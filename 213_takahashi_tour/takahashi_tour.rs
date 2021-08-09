fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned()
}

fn main() {
    let s = read_line();
    let n = {
        let mut ws = s.split_ascii_whitespace();
        let n: usize = ws.next().unwrap().parse().unwrap();
        n
    };
    let mut graph = vec![vec![]; n + 1];
    for _n in 0..(n - 1) {
        let s = read_line();
        let (a, b) = {
            let mut ws = s.split_ascii_whitespace();
            let a: usize = ws.next().unwrap().parse().unwrap();
            let b: usize = ws.next().unwrap().parse().unwrap();
            (a, b)
        };
        graph[a].push(b);
        graph[b].push(a);
    }
    for g in graph.iter_mut() {
        g.sort();
    }

    fn dfs(ans: &mut Vec<usize>, graph: &Vec<Vec<usize>>, crr: usize, pre: usize) {
        ans.push(crr);
        for nxt in graph[crr].iter() {
            if *nxt != pre {
                dfs(ans, graph, *nxt, crr);
                ans.push(crr);
            }
        }
    }
    let mut ans = vec![];
    dfs(&mut ans, &graph, 1, 0);
    println!("{:?}", ans);
}