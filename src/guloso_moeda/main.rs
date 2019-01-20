fn main() {
    let moedas_disponiveis = vec![50, 25, 10, 5, 1];
    let n_moedas = moedas_disponiveis.len();
    let mut moedas_entregues = vec![0; n_moedas];
    println!("{:?}", moedas_entregues);

    let mut troco = 86;

    while troco > 0 {
        let mut pos : usize = 0;
        for &i in &moedas_disponiveis {
            while troco >= i {
                troco = troco - i;
                moedas_entregues[pos] = moedas_entregues[pos] + 1;
            }
            pos = pos + 1;
        }
    }


    println!("{:?}", moedas_entregues);


}
