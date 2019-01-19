const N: usize = 5;

fn memorized_matrix_chain(p: &mut [i32], m: &mut [[i32; N]; N], s: &mut [[i32; N]; N]) -> i32 {

    let n  = p.len() - 1;

    for i in 1..(n+1) {
        for j in i..(n+1) {
            m[i][j] = std::i32::MAX;
        }
    }
    return lookup_chain(p, m, s, 1, n);
}


fn lookup_chain(p: &mut [i32], m: &mut [[i32; N]; N], s: &mut [[i32; N]; N], i: usize, j: usize) -> i32 {
    let val = m[i][j];
    if val < std::i32::MAX {
        return val;
    } else if i == j {
        m[i][i] = 0;
    } else {
        for k in i..j {
            let q = lookup_chain(p, m, s, i, k)
                    + lookup_chain(p, m, s, k+1, j)
                    + (p[i-1]*p[k]*p[j]);

            if q < m[i][j] {
                m[i][j] = q;
                s[i][j] = k as i32;
            }
        }
    }
    return m[i][j];
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


    println!("{}", memorized_matrix_chain(&mut p, &mut m, &mut s));


    for i in 1..5 {
        for j in 1..5 {
            print!("{:?}", s[i][j]);
        }
        println!("");
    }

}
