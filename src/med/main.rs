// Melhor exemplo:
// let str1 = String::from("atraocaasdqw");
// let str2 = String::from("rtoacabdwqoi");
// http://odur.let.rug.nl/kleiweg/lev/

// Não tem a função de eliminar, já que a inserção/deleção
// do restante dos caracteres faz mais sentido

fn main() {
    let str1 = String::from("atraocaasdqw");
    let str2 = String::from("rtoacabdwqoi");
    let n = str1.len() as i32 - 1;
    let m = str2.len() as i32 - 1;
    let saida_iterativa = med_iterativo(&str1, &str2);
    println!("saida itetativa : {}", saida_iterativa);
    let saida_recursiva = init_med_recursivo(&str1, &str2, n, m);
    println!("saida recursiva : {}", saida_recursiva);
    let saida_recursiva_sem_memoria = med_recursivo_desmemoriado_e_lerdo(&str1, &str2, n, m);
    println!("saida saida_recursiva_sem_memoria : {}", saida_recursiva_sem_memoria);
}

const MODIFICADOR_LEVENSHTEIN: i32 = 2; // CUSTO DE SUBSTITUIÇÃO
const CUSTO_COPIAR: i32 = 0;
const CUSTO_INSERIR: i32 = 1;
const CUSTO_DELETAR: i32 = 1;
const CUSTO_SWAP: i32 = 1;

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
        return med_recursivo_desmemoriado_e_lerdo(str1, str2, i - 1, j - 1) + CUSTO_COPIAR;
    }

    let mut custo = min(
        med_recursivo_desmemoriado_e_lerdo(str1, str2, i - 1, j) + CUSTO_DELETAR,
        med_recursivo_desmemoriado_e_lerdo(str1, str2, i, j - 1) + CUSTO_INSERIR,
        MODIFICADOR_LEVENSHTEIN - 1 + med_recursivo_desmemoriado_e_lerdo(str1, str2, i - 1, j - 1),
    );

    if i > 1
        && j > 1
        && str1.chars().nth(i as usize) == str2.chars().nth(j as usize - 1)
        && str1.chars().nth(i as usize - 1) == str2.chars().nth(j as usize)
    {
        custo =  std::cmp::min(
            custo,
            med_recursivo_desmemoriado_e_lerdo(str1, str2, i-2, j-2) - 1 + CUSTO_SWAP
        );
    }

    return custo;

    // Sem codigo de swap, é o mesmo do com memoria
}

fn init_med_recursivo(str1: &String, str2: &String, i: i32, j: i32) -> i32 {
    let n = str1.len() as i32;
    let m = str2.len() as i32;
    let mut distancias = vec![vec![-1 as i32; m as usize + 1]; n as usize + 1];
    return med_recursivo(str1, str2, i, j, &mut distancias);
}

fn med_recursivo(str1: &String, str2: &String, i: i32, j: i32, memoria: &mut Vec<Vec<i32>>) -> i32 {
    // Dois casos casos base, inserção de i ou j caracteres.
    // Já que uma subcadeia é vazia, o custo é a inserção de
    // todos os caracteres da outra subcadeia

    if *memoria.get(i as usize).unwrap().get(j as usize).unwrap() != -1 {
        return *memoria.get(i as usize).unwrap().get(j as usize).unwrap();
    }

    if i == 0 {
        *memoria.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap() = j;
        return j;
    }

    if j == 0 {
        *memoria.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap() = i;
        return i;
    }

    if str1.chars().nth(i as usize - 1) == str2.chars().nth(j as usize - 1) {
        *memoria.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap() = med_recursivo(str1, str2, i - 1, j - 1, memoria) + CUSTO_COPIAR;
        return med_recursivo(str1, str2, i - 1, j - 1, memoria);
    }

    *memoria.get_mut(i as usize).unwrap().get_mut(j as usize).unwrap() = min(
            med_recursivo(str1, str2, i - 1, j, memoria) + CUSTO_DELETAR,
            med_recursivo(str1, str2, i, j - 1, memoria) + CUSTO_INSERIR,
            MODIFICADOR_LEVENSHTEIN + med_recursivo(str1, str2, i - 1, j - 1, memoria) - 1,
        );

    if i > 1
        && j > 1
        && str1.chars().nth(i as usize) == str2.chars().nth(j as usize - 1)
        && str1.chars().nth(i as usize - 1) == str2.chars().nth(j as usize)
    {
        *memoria
            .get_mut(i as usize)
            .unwrap()
            .get_mut(j as usize)
            .unwrap() = std::cmp::min(
            *memoria.get(i as usize).unwrap().get(j as usize).unwrap(),
            *memoria
                .get(i as usize - 2)
                .unwrap()
                .get(j as usize - 2)
                .unwrap()
                + CUSTO_SWAP
                - 1,
        );
    }

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
                distancias[i][j] = distancias[i - 1][j - 1] + CUSTO_COPIAR;
                continue;
            }

            distancias[i][j] = min(
                distancias[i - 1][j] + CUSTO_DELETAR, // Deleta o caracter de str1 ignorando ele
                distancias[i][j - 1] + CUSTO_INSERIR, // Insere um caracter em str2
                MODIFICADOR_LEVENSHTEIN + distancias[i - 1][j - 1], // Substitui
            );

            if i > 1
                && j > 1
                && str1.chars().nth(i - 1) == str2.chars().nth(j - 2)
                && str1.chars().nth(i - 2) == str2.chars().nth(j - 1)
            {
                distancias[i][j] = std::cmp::min(
                    distancias[i][j],                      //Mantem o mesmo valor
                    distancias[i - 2][j - 2] + CUSTO_SWAP, // Faz uma troca (no caso de ..ab.. e ..ba..)
                );
            }
        }
    }

    return distancias[n as usize][m as usize];
}
