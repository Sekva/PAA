// Melhor exemplo:
// let str1 = String::from("atraocaasdqw");
// let str2 = String::from("rtoacabdwqoi");
// http://odur.let.rug.nl/kleiweg/lev/

// Não tem a função de eliminar, já que a inserção/deleção
// do restante dos caracteres faz mais sentido

fn main() {
	//Strings a serem comparadas
    let str1 = String::from("atraocaasdqw");
    let str2 = String::from("rtoacabdwqoi");
	//Comprimento da strings
    let n = str1.len() as i32;
    let m = str2.len() as i32;

	println!("\n\nString_A : \"{}\"", str1);
	println!("String_B : \"{}\"\n", str2);

    let saida_iterativa = med_iterativo(&str1, &str2);
    println!("Saída itetativa : {}", saida_iterativa);

	let saida_recursiva = init_med_recursivo(&str1, &str2);
    println!("Saída recursiva com memorização : {}", saida_recursiva);

	//Essa função está não otimizada (não só pelo fato de que não há memorização)
	//Mas principal influenciador do custo temporal que a mesma possui é a não memorização
	let saida_recursiva_sem_memoria = med_recursivo_desmemoriado_e_lerdo(&str1, &str2, n, m);
    println!("Saída recursiva sem memorização : {}", saida_recursiva_sem_memoria);
}

const CUSTO_SUBSTITUIR: i32 = 2;
const CUSTO_COPIAR: i32 = 0;
const CUSTO_INSERIR: i32 = 1;
const CUSTO_DELETAR: i32 = 1;
const CUSTO_SWAP: i32 = 1;

//Retorna o mínimo dentre três valores
fn min(a: i32, b: i32, c: i32) -> i32 {
    return std::cmp::min(c, std::cmp::min(a, b));
}

fn med_recursivo_desmemoriado_e_lerdo(str1: &String, str2: &String, i: i32, j: i32) -> i32 {
	//Para escrever primeira coluna de [0..len(str1)]
	if i == 0 {
        return j;
    };
	//Para escrever primeira linha de [0..len(str2)]
    if j == 0 {
        return i;
    };

	//Cópia
    if str1.chars().nth(i as usize - 1) == str2.chars().nth(j as usize - 1) {
        return med_recursivo_desmemoriado_e_lerdo(str1, str2, i - 1, j - 1) + CUSTO_COPIAR;
    }

	// Mínimo dentre os subproblemas
    let mut custo = min(
		//Aqui é válido notar a NÃO memorização
		//Toda vez que exige um valor anterior, é necessário recalcular
        med_recursivo_desmemoriado_e_lerdo(str1, str2, i - 1, j) + CUSTO_DELETAR,
        med_recursivo_desmemoriado_e_lerdo(str1, str2, i, j - 1) + CUSTO_INSERIR,
        med_recursivo_desmemoriado_e_lerdo(str1, str2, i - 1, j - 1) + CUSTO_SUBSTITUIR
    );

	//Se já for possível verificar a troca
	//Se a troca for possível de ser realizada
    if i > 1 && j > 1
        && str1.chars().nth(i as usize - 1) == str2.chars().nth(j as usize - 2)
        && str1.chars().nth(i as usize - 2) == str2.chars().nth(j as usize - 1)
    {
		//É necessário verificar se vale a pena fazer a troca
		//Pelo fato de que o custo da troca pode ser muito caro
        custo = std::cmp::min(
            custo,
            med_recursivo_desmemoriado_e_lerdo(str1, str2, i - 2, j - 2) + CUSTO_SWAP,
			//(Válido notar novamente a o efeito da NÃO memorização)
        );
    }

    return custo; //Custo do melhor caso
}

//Criação da matriz e chamada da função recursiva com memorização
fn init_med_recursivo(str1: &String, str2: &String) -> i32 { //Retorna um inteiro
    let n = str1.len() as i32;
    let m = str2.len() as i32;
	//Inicializa a matriz de custo com -1 em todos os campos
    let mut distancias = vec![vec![-1 as i32; m as usize + 1]; n as usize + 1];
    return med_recursivo(str1, str2, n, m, &mut distancias);
}

// Função recursiva com memorização
fn med_recursivo(str1: &String, str2: &String, i: i32, j: i32, memoria: &mut Vec<Vec<i32>>) -> i32 {
	//Caso o valor recorrido já tenha sido calculaddo, retorna-o ao invéz de recalculá-lo
	if *memoria.get(i as usize).unwrap().get(j as usize).unwrap() != -1 {
		//Utilização de memorização
        return *memoria.get(i as usize).unwrap().get(j as usize).unwrap();
    }

	//Para escrever primeira coluna de [0..len(str1)]
    if i == 0 {
        *memoria
            .get_mut(i as usize)
            .unwrap()
            .get_mut(j as usize)
            .unwrap() = j;
        return j;
    }

	//Para escrever primeira linha de [0..len(str2)]
    if j == 0 {
        *memoria
            .get_mut(i as usize)
            .unwrap()
            .get_mut(j as usize)
            .unwrap() = i;
        return i;
    }

	// Caso de cópia, onde os caracteres são iguais
    if str1.chars().nth(i as usize - 1) == str2.chars().nth(j as usize - 1) {
        *memoria
            .get_mut(i as usize)
            .unwrap()
            .get_mut(j as usize)
            .unwrap() = med_recursivo(str1, str2, i - 1, j - 1, memoria) + CUSTO_COPIAR;
        return med_recursivo(str1, str2, i - 1, j - 1, memoria);
    }

	//Atribui à posição observada atualmente o melhor valor para o determinado subproblema
	//No nosso caso, o melhor valor é o menor valor
    *memoria
        .get_mut(i as usize)
        .unwrap()
        .get_mut(j as usize)
        .unwrap() = min(
        med_recursivo(str1, str2, i - 1, j, memoria) + CUSTO_DELETAR,
        med_recursivo(str1, str2, i, j - 1, memoria) + CUSTO_INSERIR,
        med_recursivo(str1, str2, i - 1, j - 1, memoria) + CUSTO_SUBSTITUIR,
    );


	//Se já for possível verificar a troca
	//Se a troca for possível de ser realizada
    if i > 1 && j > 1
        && str1.chars().nth(i as usize - 1) == str2.chars().nth(j as usize - 2)
        && str1.chars().nth(i as usize - 2) == str2.chars().nth(j as usize - 1)
    {
		//É necessário verificar se vale a pena fazer a troca
		//Pelo fato de que o custo da troca pode ser muito caro
        *memoria
            .get_mut(i as usize)
            .unwrap()
            .get_mut(j as usize)
            .unwrap() = std::cmp::min(
            *memoria.get(i as usize).unwrap().get(j as usize).unwrap(),
            med_recursivo(str1, str2, i - 2, j - 2, memoria) + CUSTO_SWAP,
        );
    }

    return *memoria.get(i as usize).unwrap().get(j as usize).unwrap();
}

// Implementação iterativa com abordagem 'bottom up'
fn med_iterativo(str1: &String, str2: &String) -> i32 {

    //coleta dos valores a serem percorridos por 'i' e 'j' :
    let n = str1.len() as i32;
    let m = str2.len() as i32;

	//Criação e inicialização da matriz de custo; Valores inicializados com 0
    let mut distancias = vec![vec![0 as i32; m as usize + 1]; n as usize + 1];

	//Percorre as linhas matriz
    for i in 0..=n as usize {
		//Percorre as colunas da matriz
        for j in 0..=m as usize {

			//Para escrever primeira coluna de [0..len(str1)]
            if i == 0 {
                distancias[i][j] = j as i32;
                continue;
            };
			//Para escrever primeira linha de [0..len(str2)]
            if j == 0 {
                distancias[i][j] = i as i32;
                continue;
            };

			// Caso de cópia, onde os caracteres são iguais
            if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                distancias[i][j] = distancias[i - 1][j - 1] + CUSTO_COPIAR;
                continue;
            }

			//Atribui à posição observada atualmente o valor ótimo para o determinado subproblema
            distancias[i][j] = min(
                distancias[i - 1][j] + CUSTO_DELETAR, // Deleta o caracter de str1 ignorando-o
                distancias[i][j - 1] + CUSTO_INSERIR, // Insere um caracter em str2
                distancias[i - 1][j - 1] + CUSTO_SUBSTITUIR // Substitui (caracteres distintos)
            );

			//Se já for possível verificar a troca
				//Se a troca for possível de ser realizada
            if i > 1 && j > 1
                && str1.chars().nth(i - 1) == str2.chars().nth(j - 2)
                && str1.chars().nth(i - 2) == str2.chars().nth(j - 1)
            {
				//É necessário verificar se vale a pena fazer a troca
				//Pelo fato de que o custo da troca pode ser muito caro
                distancias[i][j] = std::cmp::min(
                    distancias[i][j],                      //Mantem o mesmo valor
                    distancias[i - 2][j - 2] + CUSTO_SWAP, // Faz uma troca (no caso de ..ab.. e ..ba..)
                );
            }
        }
    }

    return distancias[n as usize][m as usize];
}
