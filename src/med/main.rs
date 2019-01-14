extern crate time;
use time::PreciseTime;

fn main() {
    let str1 = String::from("cat asdasdasdasdasdasdasdasdasdasdasdasdasd");
    let str2 = String::from("cut asdasdasdasdasdasdasdasdasdasdasdasdasd");
    let n = str1.len() as i32 - 1;
    let m = str2.len() as i32 - 1;
    let mut start = PreciseTime::now();
    let saida_iterativa = med3_op(&str1, &str2);
    let mut end = PreciseTime::now();
    let tempo_itera = start.to(end);
    println!("saida itetativa : {}, com tempo : {}", saida_iterativa, tempo_itera);
    start = PreciseTime::now();
    let saida_recursiva = d_levenshtein(&str1, &str2, n, m);
    end = PreciseTime::now();
    let tempo_recursivo = start.to(end);
    println!("saida recursiva : {}, com tempo : {}", saida_recursiva, tempo_recursivo);
}

const MODIFICADOR_LEVENSHTEIN: i32 = 2;

fn min(a: i32, b: i32, c: i32) -> i32 {
    return std::cmp::min(c, std::cmp::min(a, b));
}

fn d_levenshtein(str1: &String, str2: &String, i: i32, j: i32) -> i32 {
    // Dois casos casos base, inserção de i ou j caracteres.
    // Já que uma subcadeia é vazia, o custo é a inserção de
    // todos os caracteres da outra subcadeia
    if i == 0 {
        return j;
    };
    if j == 0 {
        return i;
    };

    if str1.chars().nth(i as usize) == str2.chars().nth(j as usize) {
        return d_levenshtein(str1, str2, i - 1, j - 1);
    }

    return 1 + min(
        d_levenshtein(str1, str2, i - 1, j),
        d_levenshtein(str1, str2, i, j - 1),
        MODIFICADOR_LEVENSHTEIN - 1 + d_levenshtein(str1, str2, i - 1, j - 1),
    );
}

fn med3_op(str1: &String, str2: &String) -> i32 {
    // Considerando a abordagem bottom up, pegando valores de
    // i e j :
    let n = str1.len() as i32;
    let m = str2.len() as i32;

    let mut distancias = vec![vec![0 as i32; m as usize + 1]; n as usize + 1];

    let mut start = PreciseTime::now();


    for i in 0..=n as usize {
        for j in 0..=m as usize {
            if i == 0 {
                distancias[i][j] = j as i32;
                continue;
            };
            if j == 0 {
                distancias[i][j] = i as i32;
                continue;
            };

            if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                distancias[i][j] = distancias[i - 1][j - 1];
                continue;
            }

            distancias[i][j] = min(
                1 + distancias[i - 1][j],
                1 + distancias[i][j - 1],
                MODIFICADOR_LEVENSHTEIN + distancias[i - 1][j - 1],
            );
        }
    }

    let mut end = PreciseTime::now();
    let tempo_itera = start.to(end);
    println!("asdasasdasasdasdasdasdasdasddasd {} asdasd", tempo_itera);
    return distancias[n as usize][m as usize];
}
