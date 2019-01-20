// Imports pra poder usar a heap de minimo implementada pelo sistema
use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Esse derive é para implementar automaticamente copia e verificações para o
// struct.
// Esse struct serve para guardar os lugares que podem ou estão visitados.
// Quando forem usados fica mais claro o seu proposito.
#[derive(Copy, Clone)]
struct Lugar {
    // Distancia como float
    distancia: f64,
    // Como os nós vão ser indices de vetores de arestas, tratar logo como
    // indice (usize)
    no_atual: usize,
}

//Codigo abaixo para criar a heap de minima
///////////////////////////////////////////////////////////////////////////////
// Implementa como dois nós são parcialmente comparados
impl PartialEq for Lugar {
    fn eq(&self, other: &Lugar) -> bool {
        self.no_atual == other.no_atual
    }
}
// Deixando em branco é usado a implementação de PartialEq
impl Eq for Lugar {}

// Definição de como dois nós são comparados (primeiro vem other e depois self)
// essa troca inverte a regra da heap
impl Ord for Lugar {
    fn cmp(&self, other: &Lugar) -> Ordering {
        other
            .distancia
            .partial_cmp(&self.distancia)
            .unwrap()
            .then_with(|| self.no_atual.cmp(&other.no_atual))
    }
}

// Define a outra função usada como a mesma da comparação completa
impl PartialOrd for Lugar {
    fn partial_cmp(&self, other: &Lugar) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
///////////////////////////////////////////////////////////////////////////////

struct Aresta {
    // aponta_para indica a posição no vetor de vetor de arestas, que é um
    // vertice
    aponta_para: usize,
    // Custo de um vertice a outro
    custo: f64,
}

// Definindo tipo pra facilitar
type Lista = Vec<Vec<Aresta>>;

// Função principal
// Recebe uma lista de adjacencia onde cada posição é um vertice com arestas
// que apontam para outros vertices. Recebe qual a posição do vertice inicial
// e a do final. Retorna o menor custo de um a outro. Retorna None quando não
// não tem caminho.
fn dijkstra(lista: &Lista, pos_inicial: usize, no_final: usize) -> Option<f64> {
    // Array de distancias onde cada indice é a representação de um vetor
    // Exemplo: distancias[5] é a distancia de pos_inicial até o vertice 5
    let mut distancias: Vec<f64> = vec![99999999.99; lista.len()];

    // Ai vem a fila de prioridades de minima
    // Ela vai guardar instancias do struct Lugar e na primeira posição vai
    // esta o lugar qual o menor lugar para ir (com o menor custo) a partir
    // do vertice atual
    let mut q = BinaryHeap::new();

    // Seguindo o algoritmo, a distancia até o nó inicial é 0
    distancias[pos_inicial] = 0.0;

    // Até agora o menor custo é ficar onde se está,
    // botamos na fila de prioridade

    q.push(Lugar {
        distancia: 0.0,
        no_atual: pos_inicial,
    });

    //Iteramos até que a lista de prioridades esteja vazia
    while let Some(Lugar {
        distancia,
        no_atual,
    }) = q.pop()
    {
        // Se conseguimos chegar no no_final, chegamos ao destino
        // e retorna o custo até o lugar que é custo entre
        // pos_inicial e no_final
        if no_final == no_atual {
            return Some(distancia);
        }

        // Se a distancia até aqui for maior que a distancias que já tinhamos
        // até aqui, foi porque tomamos um caminho mais longo, logo não vale
        // a pena, proxima iteração
        if distancia > distancias[no_atual] {
            continue;
        }

        // Agora pegamos todas as arestas do vertice que saiu da lista
        // cuja distancia é a menor possivel
        for aresta in &lista[no_atual] {
            // Pego uma aresta, vamos considerar que é o nosso proximo lugar
            // de visita
            let prox_lugar = Lugar {
                distancia: distancia + aresta.custo,
                no_atual: aresta.aponta_para,
            };

            // Se a distancia até ele for menor que a distancia que já
            // conheciamos até lá, pode ser uma boa escolha ir para lá
            // e então colocamos na lista de prioridade que vai organizar
            // qual a melhor opção para ir
            if prox_lugar.distancia < distancias[prox_lugar.no_atual] {
                q.push(prox_lugar);
                // E dizemos que o menor custo até o nó é o custo deste
                // até o proximo
                distancias[prox_lugar.no_atual] = prox_lugar.distancia;
            }
        }
    }

    // Caso a fila esvaziou e nada foi retornado, não tem nenhum caminho
    None
}


fn main() {

    // Grafo: http://siddarthareddy.weebly.com/uploads/2/8/7/9/28799429/4999443.png

    let grafo = vec![
        // Vertice 0
        vec![
            Aresta { aponta_para: 1, custo: 10.0,},
            Aresta { aponta_para: 3, custo: 5.0 },
        ],
        // Vertice 1
        vec![
            Aresta { aponta_para: 2, custo: 1.0 },
            Aresta { aponta_para: 3, custo: 2.0 },
        ],
        // Vertice 2
        vec![
            Aresta { aponta_para: 4, custo: 4.0 },
        ],
        // Vertice 3
        vec![
            Aresta { aponta_para: 1, custo: 3.0 },
            Aresta { aponta_para: 2, custo: 9.0 },
            Aresta { aponta_para: 4, custo: 2.0 },
        ],
        // Vertice 4
        vec![
            Aresta { aponta_para: 2, custo: 6.0 },
            Aresta { aponta_para: 0, custo: 7.0 },
        ],
    ];

    match dijkstra(&grafo, 0, 4) {
        Some(x) => println!("A distancia entre o vertice {} e o vertice {} é {}", 0, 4, x),
        None => println!("Não tem caminhos de {} até {}", 0, 4)
    }
}
