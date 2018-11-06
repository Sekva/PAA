const N: usize = 5;

fn matrix_chain_order(p: &mut [i32], m: &mut [[i32; N]; N], s: &mut [[i32; N]; N]) {
    let n = N ;//- 1;
    for i in 1..n {
        m[i][i] = 0;
    }
    for l in 2..n {
        for i in 1..(n - l + 1) {
            let j = i + l - 1;
            m[i][j] = std::i32::MAX;
            for k in i..(j - 1 + 1) {
                let q = m[i][k] + m [k+1][j] + (p[i-1]*p[k]*p[j]);

                if q < m[i][j] {
                    m[i][j] = q;
                    s[i][j] = k as i32;
                }
            }
        }
    }
}

fn main() {

    let mut p: [i32; N] = [0; N];
    p[0] = 4;
    p[1] = 2;
    p[2] = 3;
    p[3] = 1;
    p[4] = 3;


    let mut m: [[i32; N]; N] = [[0; N]; N];
    let mut s: [[i32; N]; N] = [[0; N]; N];

    matrix_chain_order(&mut p, &mut m, &mut s);
    println!("{:?}", m[1][N-1]);

    for i in 1..5 {
        for j in 1..5 {
            print!("{:?}", s[i][j]);
        }
        println!("");
    }

}
