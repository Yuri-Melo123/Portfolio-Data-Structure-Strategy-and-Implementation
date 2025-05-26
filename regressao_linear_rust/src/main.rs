fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.1, 6.0, 8.1, 10.1];

    let (slope, intercept) = regressao_linear(&x, &y).expect("Erro no cálculo da regressão");

    println!("Coeficiente angular (a): {:.4}", slope);
    println!("Coeficiente linear (b): {:.4}", intercept);

    let r2 = calcular_r2(&x, &y, slope, intercept);
    let mse = calcular_mse(&x, &y, slope, intercept);

    println!("R²: {:.4}", r2);
    println!("MSE: {:.4}", mse);

    let previsao = prever(6.0, slope, intercept);
    println!("Previsão para x = 6: {:.4}", previsao);
}

fn regressao_linear(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    if x.len() != y.len() || x.is_empty() {
        return None;
    }

    let n = x.len() as f64;
    let soma_x: f64 = x.iter().sum();
    let soma_y: f64 = y.iter().sum();
    let soma_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
    let soma_x2: f64 = x.iter().map(|xi| xi * xi).sum();

    let numerador = n * soma_xy - soma_x * soma_y;
    let denominador = n * soma_x2 - soma_x * soma_x;

    if denominador == 0.0 {
        return None;
    }

    let a = numerador / denominador;
    let b = (soma_y - a * soma_x) / n;

    Some((a, b))
}

fn calcular_r2(x: &[f64], y: &[f64], a: f64, b: f64) -> f64 {
    let media_y = y.iter().sum::<f64>() / y.len() as f64;
    let ss_total: f64 = y.iter().map(|yi| (yi - media_y).powi(2)).sum();
    let ss_res: f64 = x.iter().zip(y.iter())
        .map(|(xi, yi)| {
            let y_pred = a * xi + b;
            (yi - y_pred).powi(2)
        }).sum();

    1.0 - ss_res / ss_total
}

fn calcular_mse(x: &[f64], y: &[f64], a: f64, b: f64) -> f64 {
    let n = x.len() as f64;
    let erro_quadratico: f64 = x.iter().zip(y.iter())
        .map(|(xi, yi)| {
            let y_pred = a * xi + b;
            (yi - y_pred).powi(2)
        }).sum();

    erro_quadratico / n
}

fn prever(x: f64, a: f64, b: f64) -> f64 {
    a * x + b
}
