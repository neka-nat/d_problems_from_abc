use std::cmp::min;

fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned()
}

fn main() {
    let s = read_line();
    let (n, m) = {
        let mut ws = s.split_ascii_whitespace();
        let n: usize = ws.next().unwrap().parse().unwrap();
        let m: usize = ws.next().unwrap().parse().unwrap();
        (n, m)
    };
    let mut abcs = vec![];
    for _i in 0..m {
        let s = read_line();
        let abc = {
            let mut ws = s.split_ascii_whitespace();
            let a: usize = ws.next().unwrap().parse().unwrap();
            let b: usize = ws.next().unwrap().parse().unwrap();
            let c: usize = ws.next().unwrap().parse().unwrap();
            (a, b, c)
        };
        abcs.push(abc);
    }
    const INF: usize = 1 << 60;
    let mut d = vec![vec![INF; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for (a, b, c) in abcs.iter() {
        d[a - 1][b - 1] = *c;
    }
    let mut ans: usize = 0;
    for k in 0..n {
        let mut nxt = vec![vec![0usize; n]; n];
        for i in 0..n {
            for j in 0..n {
                nxt[i][j] = min(d[i][j], d[i][k] + d[k][j]);
                if nxt[i][j] < INF {
                    ans += nxt[i][j];
                }
            }
        }
        d = nxt;
    }
    println!("{:?}", ans);
}