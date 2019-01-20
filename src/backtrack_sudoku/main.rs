// Utilidades pra facilitar o uso das matrizes
// ############################################################################

type Matriz = Vec<Vec<i32>>;

fn get_pos(matriz: &Matriz, linha: usize, coluna: usize) -> i32 {
    matriz[linha - 1][coluna - 1]
}

fn set_pos(matriz: &mut Matriz, linha: usize, coluna: usize, valor: i32) {
    *matriz
        .get_mut(linha - 1)
        .unwrap()
        .get_mut(coluna - 1)
        .unwrap() = valor;
}

fn printar_matriz(matriz: &mut Matriz) {
    for i in matriz {
        println!("{:?}", i);
    }
}

// ############################################################################

// Como sudoku normalmente se joga em um tabuleiro de n² x n²
const SUB_N: usize = 3;
const N: usize = SUB_N * SUB_N;

// Retorna uma tupla com true e as coordenadas de um campo não preenchido
// no tabuleiro. Caso não encontre nenhum campo vazio, retorna false
// e os valores não importam
fn achar_vaga(matriz: &Matriz) -> (bool, usize, usize) {
    // Laço percorrendo todo o tabuleiro, isso pode ser otimizado depois
    for i in 1..=N {
        for j in 1..=N {
            if get_pos(matriz, i, j) == 0 {
                return (true, i, j);
            }
        }
    }
    (false, 0, 0)
}

// Função que verifica se é valido inserir um numero dig na linha e na coluna
// da matriz
fn valido(matriz: &mut Matriz, linha: usize, coluna: usize, dig: i32) -> bool {
    //Se o dig já existe na linha que se deseja inserir
    if matriz.get(linha - 1).unwrap().contains(&dig) {
        return false;
    }

    //Se o dig já existe na coluna que se deseja inserir
    for i in matriz.iter() {
        if i[coluna - 1] == dig {
            return false;
        }
    }

    // Verifica se a posição é mesmo desocupada
    // Redundande, mas vale a pena
    if get_pos(matriz, linha, coluna) != 0 {
        return false;
    }

    // Pega qual submatriz 3x3 (no caso de N = 9) o dig quer ser inserido
    // Exemplo da submatriz 2, 2
    // x x x x x x x x x
    // x x x x x x x x x
    // x x x x x x x x x
    // x x x o o o x x x
    // x x x o o o x x x
    // x x x o o o x x x
    // x x x x x x x x x
    // x x x x x x x x x
    // x x x x x x x x x
    //
    let linha_quadrante = (linha as f64 / SUB_N as f64).ceil() as usize;
    let coluna_quadrante = (coluna as f64 / SUB_N as f64).ceil() as usize;

    // Dado o quadrante, verifica se todos os elementos dele são diferentes
    // do valor de dig que quer ser inserido
    for i in 0..SUB_N {
        for j in 0..SUB_N {
            if get_pos(
                matriz,
                SUB_N * (linha_quadrante - 1) + 1 + i,
                SUB_N * (coluna_quadrante - 1) + 1 + j,
            ) == dig
            {
                return false;
            }
        }
    }

    true
}

fn resolver(matriz: &mut Matriz) -> bool {
    // Verifica se ainda tem algum campo do tabuleiro vago
    let (tem_vaga, linha, coluna) = achar_vaga(matriz);

    // Se não tem vaga, o jogo está completo, retorna true dizendo que acabou
    if !tem_vaga {
        return true;
    }

    // Dado uma linha e uma coluna vaga, tenta inserir algum numero na vaga
    for digito in 1..=N as i32 {
        // Se é valido por o digito na posição, ok, se não itera para o proximo
        // digito e tenta ele
        if valido(matriz, linha, coluna, digito) {

            // Colocando o digito no tabuleiro
            set_pos(matriz, linha, coluna, digito);

            // Se com esse digito o tabuleiro ficou copleto, retorna true
            // LABEL : 1
            if resolver(matriz) {
                return true;
            }

            // Se não for resolvido, é removido o digito e itera para o proximo
            // digito
            set_pos(matriz, linha, coluna, 0);
        }
    }

    // Caso tenha iterado todos os digitos possiveis, algum digito posto
    // anteriormente impediu quem o jogo seja resolvido.
    // Retornando false, o if abaixo de "LABEL : 1" não entra, e o digito posto
    // anteriormente é removido. Essa é a chave do backtracking
    false
}

fn main() {
    // Tabuleiro 9x9 simples com alguns campos vagos
    // https://en.wikipedia.org/wiki/Sudoku#/media/File:Sudoku_Puzzle_by_L2G-20050714_solution_standardized_layout.svg
    let mut matriz = vec![
        vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
        vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
        vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
        vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
        vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
        vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
        vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
        vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
        vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    // Chamada princial, retorna true se tem solução ou false C.C.
    resolver(&mut matriz);
    // Pra ficar bonito
    printar_matriz(&mut matriz);
}
