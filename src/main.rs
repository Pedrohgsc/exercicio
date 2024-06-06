use std::collections::HashMap;

#[derive(Debug)]
struct Produto {
    id: u32,
    quantidade: u32,
    data_entrega: NaiveDate,
}

#[derive(Debug)]
struct PedidoCompra {
    materia_prima: String,
    quantidade_total: u32,
    data_pedido: NaiveDate,
}

fn calcular_data_pedido(data_entrega: NaiveDate, intervalo_pedido: i64) -> NaiveDate {
    data_entrega - Duration::days(intervalo_pedido)
}

fn main() {
    // Dados de entrada
    let produtos = vec![
        Produto { id: 1, quantidade: 100, data_entrega: NaiveDate::from_ymd(2024, 6, 10) },
        Produto { id: 2, quantidade: 50, data_entrega: NaiveDate::from_ymd(2024, 6, 12) },
    ];

    let materias_primas_por_produto: HashMap<u32, Vec<(&str, u32)>> = [
        (1, vec![("A", 2), ("B", 1)]),
        (2, vec![("A", 1), ("C", 3)]),
    ].iter().cloned().collect();

    let pedidos_minimos: HashMap<&str, (u32, i64)> = [
        ("A", (50, 7)),
        ("B", (20, 7)),
        ("C", (30, 7)),
    ].iter().cloned().collect();

    // Calcular as necessidades de matérias-primas
    let mut necessidades_materias_primas: HashMap<&str, Vec<(u32, NaiveDate)>> = HashMap::new();

    for produto in &produtos {
        if let Some(materias) = materias_primas_por_produto.get(&produto.id) {
            for (materia_prima, quantidade_necessaria) in materias {
                necessidades_materias_primas.entry(materia_prima)
                    .or_insert(vec![])
                    .push((produto.quantidade * quantidade_necessaria, produto.data_entrega));
            }
        }
    }

    // Consolidar os pedidos de compra de matérias-primas
    let mut pedidos_compra = vec![];

    for (materia_prima, necessidades) in &necessidades_materias_primas {
        let quantidade_total: u32 = necessidades.iter().map(|(quantidade, _)| *quantidade).sum();
        let pedido_minimo = pedidos_minimos[materia_prima].0;
        let intervalo_pedido = pedidos_minimos[materia_prima].1;

        let quantidade_pedido = std::cmp::max(quantidade_total, pedido_minimo);
        let data_pedido = calcular_data_pedido(
            necessidades.iter().map(|(_, data_entrega)| *data_entrega).max().unwrap(),
            intervalo_pedido
        );

        pedidos_compra.push(PedidoCompra {
            materia_prima: materia_prima.to_string(),
            quantidade_total: quantidade_pedido,
            data_pedido,
        });
    }

    // Exibir os pedidos de compra
    for pedido in pedidos_compra {
        println!("{:?}", pedido);
    }
}
