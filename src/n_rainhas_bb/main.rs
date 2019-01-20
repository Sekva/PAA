
extern crate time;
use time::PreciseTime;

// Utilidades pra facilitar o uso das matrizes
// ############################################################################

type Matriz = Vec<Vec<char>>;
type MatrizInt = Vec<Vec<i32>>;

fn get_pos(matriz: &Matriz, linha: usize, coluna: usize) -> char {
    matriz[linha][coluna]
}

fn get_pos_i(matriz: &MatrizInt, linha: usize, coluna: usize) -> i32 {
    matriz[linha][coluna]
}

fn set_pos(matriz: &mut Matriz, linha: usize, coluna: usize, valor: char) {
    *matriz
        .get_mut(linha)
        .unwrap()
        .get_mut(coluna)
        .unwrap() = valor;
}

fn set_pos_i(matriz: &mut MatrizInt, linha: usize, coluna: usize, valor: i32) {
    *matriz
        .get_mut(linha)
        .unwrap()
        .get_mut(coluna)
        .unwrap() = valor;
}

fn printar_matriz_i(matriz: &MatrizInt) {
    println!();
    for i in matriz {
        println!("{:?}", i);
        println!();
    }
    println!();
}

fn printar_matriz(matriz: &Matriz) {
    println!();
    for i in matriz {
        println!("{:?}", i);
        println!();
    }
    println!();
}
// ############################################################################

fn valido(matriz_risco : &MatrizInt, linha : usize, coluna : usize) -> bool {
    // Se o elemento da posição da matriz de risco for maior que 1
    // tem ao menos 1 rainha que possivelmente o comeria
    if get_pos_i(matriz_risco, linha, coluna) > 0 {
        return false;
    }

    true
}

fn set_risco(matriz_risco : &mut MatrizInt, linha : usize, coluna : usize, risco : bool) {

    // Se é pra por risco, é somado 1, se não, é subtraido 1
    let modf = if risco {1} else {-1};

    // Marcando todas as linhas e colunas
    for i in 0..TAMANHO {
        let val_i = get_pos_i(matriz_risco, linha, i);
        set_pos_i(matriz_risco, linha, i, modf + val_i);
        let val_j = get_pos_i(matriz_risco, i, coluna);
        set_pos_i(matriz_risco, i, coluna, modf + val_j);
    }

    // Direção NO
    let mut i = linha as i32;
    let mut j = coluna as i32;

    while i >= 0 && j >= 0 {
        let val = get_pos_i(matriz_risco, i as usize, j as usize);
        set_pos_i(matriz_risco, i as usize, j as usize, modf + val);
        i = i - 1;
        j = j - 1;
    }
    i = linha as i32;
    j = coluna as i32;

    // Direção NE
    while i >= 0 && j < TAMANHO as i32 {
        let val = get_pos_i(matriz_risco, i as usize, j as usize);
        set_pos_i(matriz_risco, i as usize, j as usize, modf + val);
        i = i - 1;
        j = j + 1;
    }

    i = linha as i32;
    j = coluna as i32;

    // Direção SE
    while i < TAMANHO as i32 && j < TAMANHO as i32 {
        let val = get_pos_i(matriz_risco, i as usize, j as usize);
        set_pos_i(matriz_risco, i as usize, j as usize, modf + val);
        i = i + 1;
        j = j + 1;
    }

    i = linha as i32;
    j = coluna as i32;
    // Direção SO
    while i < TAMANHO as i32 && j >= 0 {
        let val = get_pos_i(matriz_risco, i as usize, j as usize);
        set_pos_i(matriz_risco, i as usize, j as usize, modf + val);
        i = i + 1;
        j = j - 1;
    }

}

fn resolver(matriz : &mut Matriz, matriz_risco : &mut MatrizInt, linha : usize) -> bool {

    // Se a linha que estamos tentando inserir for fora do tabuleiro
    // quer dizer que colocamos todas as N rainhas
    if linha >= TAMANHO {
        return true;
    }

    // Se ainda tem vaga, veja todas as possibilidades de coluna,
    // itere por elas, e pra cada uma, veja como funciona para as linhas abaixo
    for i in 0..TAMANHO {
        if valido(matriz_risco, linha, i) {
            // Se é valido por uma rainha, ponha e tente resolver para o resto
            set_pos(matriz, linha, i, RAINHA);
            set_risco(matriz_risco, linha, i, RISCO);

            //Resolvendo para o resto
            if resolver(matriz, matriz_risco, linha + 1) {
                // Se conseguiu, temos ums solução, retorne
                return true;
            }

            // Se não, remova a rainha e tente na proxima coluna
            set_pos(matriz, linha, i, VAGA);
            set_risco(matriz_risco, linha, i, LIVRE);
        }
    }
    // Se não conseguiu por mais nenhuma, não tem solução, retorna false
    false
}

const TAMANHO : usize = 5;
const VAGA : char = 'o';
const RAINHA : char = '♛';
const RISCO : bool = true;
const LIVRE : bool = false;

fn main() {

    // Cria as matrizes (poderia fazer em uma só, e quando printar
    // se for maior que zero printava a uma rainha, mas achei melhor assim)
    let mut matriz : Matriz = vec![vec![VAGA; TAMANHO]; TAMANHO];
    let mut matriz_risco : MatrizInt = vec![vec![0; TAMANHO]; TAMANHO];



    let start = PreciseTime::now();
    match resolver(&mut matriz, &mut matriz_risco, 0) {
        true => printar_matriz(&matriz),
        _ => println!("Não achei uma solução")
    }

    let end = PreciseTime::now();
    let tempo = start.to(end);

    println!("Demorou {:?} nanosegundos para achar a solução para dimensao {}", tempo, TAMANHO);

}
