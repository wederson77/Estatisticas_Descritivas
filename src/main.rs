use std::fs::File;
use std::io::{BufRead, BufReader};
use estatisticas_descritivas_structs::executar_estatisticas_descritivas;

fn ler_numeros(nome_arquivo: &str) -> Result<Vec<i32>, std::io::Error>{
    let arquivo = File::open(nome_arquivo)?;
    let leitor = BufReader::new(arquivo);
    let mut numeros = vec![];

    for linha in leitor.lines(){
        let linha = linha?;
        if let Ok(numero) = linha.trim().parse::<i32>(){
            numeros.push(numero);
        }
    }

    Ok(numeros)
}

fn main() {
    let numeros: Vec<i32> = ler_numeros("dados.txt").unwrap_or_else(|err| {
        eprintln!("Erro ao ler arquivo: {}", err);
        std::process::exit(1);
    });
    executar_estatisticas_descritivas(numeros);
}
