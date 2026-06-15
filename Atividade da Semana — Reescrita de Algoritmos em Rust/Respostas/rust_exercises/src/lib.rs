pub fn ex01_verificar_primeiro(lista: &[i32]) -> Option<i32> {
    lista.first().cloned()
}

pub fn ex02_somar_lista(lista: &[i32]) -> i32 {
    lista.iter().sum()
}

pub fn ex03_busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    let mut baixo = 0isize;
    let mut alto = (lista.len() as isize) - 1;

    while baixo <= alto {
        let meio = (baixo + alto) / 2;
        let chute = lista[meio as usize];

        if chute == alvo {
            return Some(meio as usize);
        }
        if chute > alvo {
            alto = meio - 1;
        } else {
            baixo = meio + 1;
        }
    }
    None
}

pub fn ex04_pares_com_soma(lista: &[i32], alvo: i32) {
    for i in 0..lista.len() {
        for j in i + 1..lista.len() {
            if lista[i] + lista[j] == alvo {
                println!("Par encontrado: ({}, {})", lista[i], lista[j]);
            }
        }
    }
}

pub fn ex05_imprimir_pares_e_pares(lista: &[i32]) {
    println!("Números pares na lista:");
    for &num in lista {
        if num % 2 == 0 {
            println!("{}", num);
        }
    }

    println!("Pares de números (i, j):");
    for &i in lista {
        for &j in lista {
            println!("({}, {})", i, j);
        }
    }
}

pub fn ex06_potencias_de_dois(n: u64) {
    let mut i = 1;
    while i <= n {
        println!("{}", i);
        i *= 2;
    }
}

pub fn ex07_fibonacci_recursivo(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        ex07_fibonacci_recursivo(n - 1) + ex07_fibonacci_recursivo(n - 2)
    }
}

pub fn ex08_ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1);
            }
        }
    }
}

pub fn ex09_produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let linhas_a = a.len();
    let colunas_a = a[0].len();
    let colunas_b = b[0].len();

    let mut resultado = vec![vec![0; colunas_b]; linhas_a];

    for i in 0..linhas_a {
        for j in 0..colunas_b {
            for k in 0..colunas_a {
                resultado[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    resultado
}

pub fn ex10_merge_sort(mut lista: Vec<i32>) -> Vec<i32> {
    if lista.len() <= 1 {
        return lista;
    }

    let meio = lista.len() / 2;
    let direita = lista.split_off(meio);
    let esquerda = lista;

    let esquerda_ordenada = ex10_merge_sort(esquerda);
    let direita_ordenada = ex10_merge_sort(direita);

    merge(esquerda_ordenada, direita_ordenada)
}

fn merge(esquerda: Vec<i32>, direita: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::with_capacity(esquerda.len() + direita.len());
    let (mut i, mut j) = (0, 0);

    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            resultado.push(esquerda[i]);
            i += 1;
        } else {
            resultado.push(direita[j]);
            j += 1;
        }
    }

    resultado.extend_from_slice(&esquerda[i..]);
    resultado.extend_from_slice(&direita[j..]);

    resultado
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex01() {
        assert_eq!(ex01_verificar_primeiro(&[1, 2, 3]), Some(1));
        assert_eq!(ex01_verificar_primeiro(&[]), None);
    }

    #[test]
    fn test_ex02() {
        assert_eq!(ex02_somar_lista(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(ex02_somar_lista(&[]), 0);
    }

    #[test]
    fn test_ex03() {
        let lista = [1, 3, 5, 7, 9];
        assert_eq!(ex03_busca_binaria(&lista, 3), Some(1));
        assert_eq!(ex03_busca_binaria(&lista, -1), None);
        assert_eq!(ex03_busca_binaria(&lista, 10), None);
    }

    #[test]
    fn test_ex04() {
        ex04_pares_com_soma(&[1, 2, 3, 4], 5);
    }

    #[test]
    fn test_ex05() {
        ex05_imprimir_pares_e_pares(&[1, 2]);
    }

    #[test]
    fn test_ex06() {
        ex06_potencias_de_dois(10);
    }

    #[test]
    fn test_ex07() {
        assert_eq!(ex07_fibonacci_recursivo(0), 0);
        assert_eq!(ex07_fibonacci_recursivo(1), 1);
        assert_eq!(ex07_fibonacci_recursivo(10), 55);
    }

    #[test]
    fn test_ex08() {
        let mut lista = [5, 3, 8, 4, 2];
        ex08_ordenacao_bolha(&mut lista);
        assert_eq!(lista, [2, 3, 4, 5, 8]);
    }

    #[test]
    fn test_ex09() {
        let a = vec![vec![1, 2], vec![3, 4]];
        let b = vec![vec![5, 6], vec![7, 8]];
        let res = ex09_produto_de_matrizes(&a, &b);
        assert_eq!(res, vec![vec![19, 22], vec![43, 50]]);
    }

    #[test]
    fn test_ex10() {
        let lista = vec![38, 27, 43, 3, 9, 82, 10];
        let res = ex10_merge_sort(lista);
        assert_eq!(res, vec![3, 9, 10, 27, 38, 43, 82]);
    }
}
