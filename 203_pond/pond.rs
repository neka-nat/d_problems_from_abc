
fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned()
}

fn main() {
    let s = read_line();
    let (n, k) = {
        let mut ws = s.split_ascii_whitespace();
        let n: usize = ws.next().unwrap().parse().unwrap();
        let k: usize = ws.next().unwrap().parse().unwrap();
        (n, k)
    };
    let mut amat = vec![];
    for _i in 0..n {
        let s = read_line();
        let v = {
            let mut ws = s.split_ascii_whitespace();
            let mut row = vec![];
            for _j in 0..n {
                let a: i32 = ws.next().unwrap().parse().unwrap();
                row.push(a);
            }
            row
        };
        amat.push(v);
    }
    let mut s = vec![vec![0; n + 1]; n + 1];
    let max_a = 1000000000;
    let mut ng = -1;
    let mut ok = max_a;
    let lim = ((k *k) / 2) + 1;
    while (ng + 1) < ok {
        let mid = (ng + ok) / 2;
        for i in 0..n {
            for j in 0..n {
                s[i + 1][j + 1] = s[i + 1][j] + s[i][j + 1] - s[i][j];
                if amat[i][j] > mid {
                    s[i + 1][j + 1] += 1;
                }
            }
        }
        let mut ext = false;
        for i in 0..(n - k + 1) {
            for j in 0..(n - k + 1) {
                if (s[i + k][j + k] + s[i][j] - s[i][j + k] - s[i + k][j]) < lim {
                    ext = true;
                }
            }
        }
        if ext {
            ok = mid
        } else {
            ng = mid
        }
    }    

    println!("{:?}", ok);
}