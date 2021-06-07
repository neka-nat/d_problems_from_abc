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
    let s = read_line();
    let tmat = {
        let mut ws = s.split_ascii_whitespace();
        let mut ts = vec![];
        for _j in 0..n {
            let t: usize = ws.next().unwrap().parse().unwrap();
            ts.push(t);
        }
        ts
    };

    let tsum = tmat.iter().sum();
    let mut dp = vec![vec![usize::MAX; tsum]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for t in 0..tsum {
            if dp[i][t] == usize::MAX {
                continue;
            }
            if dp[i + 1][t + tmat[i]] > dp[i][t] {
                dp[i + 1][t + tmat[i]] = dp[i][t];
            }
            if dp[i + 1][t] > dp[i][t] + tmat[i] {
                dp[i + 1][t] = dp[i][t] + tmat[i];
            }
        }
    }
    let mut ans = usize::MAX;
    for t in 0..tsum {
        if ans > std::cmp::max(t, dp[n][t]) {
            ans = std::cmp::max(t, dp[n][t]);
        }
    }
    println!("{:?}", ans);
}