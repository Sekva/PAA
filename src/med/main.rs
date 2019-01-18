use std::process;

fn main() {
    let str1 = String::from("execution");
    let str2 = String::from("intention");
    let n = str1.len() as i32 - 1;
    let m = str2.len() as i32 - 1;
    let saida_iterativa = med_iterativo(&str1, &str2);
    println!("saida itetativa : {}", saida_iterativa);
    let saida_recursiva = init_med_recursivo(&str1, &str2, n, m);
    println!("saida recursiva : {}", saida_recursiva);
}
const MODIFICADOR_LEVENSHTEIN: i32 = 2;


fn min(a: i32, b: i32, c: i32) -> i32 {
    return std::cmp::min(c, std::cmp::min(a, b));
}

fn med_recursivo_desmemoriado_e_lerdo(str1: &String, str2: &String, i: i32, j: i32) -> i32 {
    if i == 0 {
        return j;
    };
    if j == 0 {
        return i;
    };

    if str1.chars().nth(i as usize - 1) == str2.chars().nth(j as usize - 1) {
        return med_recursivo_desmemoriado_e_lerdo(str1, str2, i - 1, j - 1);
    }

    return 1 + min(
        med_recursivo_desmemoriado_e_lerdo(str1, str2, i - 1, j),
        med_recursivo_desmemoriado_e_lerdo(str1, str2, i, j - 1),
        MODIFICADOR_LEVENSHTEIN - 1 + med_recursivo_desmemoriado_e_lerdo(str1, str2, i - 1, j - 1),
    );
}


fn init_med_recursivo(str1: &String, str2: &String, i: i32, j: i32) -> i32 {
    let n = str1.len() as i32;
    let m = str2.len() as i32;
    let mut distancias = vec![vec![-1 as i32; m as usize + 1]; n as usize + 1];
    return med_recursivo(str1, str2, i, j, &mut distancias);
}


fn med_recursivo(str1: &String, str2: &String, i: i32, j: i32, memoria : &mut Vec<Vec<i32>>) -> i32 {
    // Dois casos casos base, inserção de i ou j caracteres.
    // Já que uma subcadeia é vazia, o custo é a inserção de
    // todos os caracteres da outra subcadeia

    if *memoria.get(i as usize).unwrap().get(j as usize).unwrap() != -1 {
        return *memoria.get(i as usize).unwrap().get(j as usize).unwrap();
    }

    if i == 0 {
        *memoria.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap() = j;
        return j;
    };

    if j == 0 {
        *memoria.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap() = i;
        return i;
    };


    if str1.chars().nth(i as usize - 1) == str2.chars().nth(j as usize - 1) {
        *memoria.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap() = med_recursivo(str1, str2, i - 1, j - 1, memoria);
        return med_recursivo(str1, str2, i - 1, j - 1, memoria);
    }

    *memoria.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap() = 1 + min(
        med_recursivo(str1, str2, i - 1, j, memoria),
        med_recursivo(str1, str2, i, j - 1, memoria),
        MODIFICADOR_LEVENSHTEIN - 1 + med_recursivo(str1, str2, i - 1, j - 1, memoria),
    );

    return *memoria.get(i as usize).unwrap().get(j as usize).unwrap();
}

fn med_iterativo(str1: &String, str2: &String) -> i32 {
    // Considerando a abordagem bottom up, pegando valores de
    // i e j :
    let n = str1.len() as i32;
    let m = str2.len() as i32;

    let mut distancias = vec![vec![0 as i32; m as usize + 1]; n as usize + 1];

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

    return distancias[n as usize][m as usize];
}
