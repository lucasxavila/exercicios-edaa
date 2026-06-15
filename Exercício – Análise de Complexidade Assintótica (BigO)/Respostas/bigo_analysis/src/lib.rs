pub fn ex01_acesso_constante(lista: &[i32]) -> Option<i32> {
    lista.first().cloned()
}

pub fn ex02_soma_linear(lista: &[i32]) -> i32 {
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

pub fn ex04_encontrar_pares(lista: &[i32], alvo: i32) -> Vec<(i32, i32)> {
    let mut pares = Vec::new();
    for i in 0..lista.len() {
        for j in i + 1..lista.len() {
            if lista[i] + lista[j] == alvo {
                pares.push((lista[i], lista[j]));
            }
        }
    }
    pares
}

pub fn ex05_loops_aninhados(lista: &[i32]) {
    // Bloco linear: O(n)
    for &num in lista {
        if num % 2 == 0 {
            println!("{}", num);
        }
    }

    // Bloco quadrático: O(n^2)
    for &i in lista {
        for &j in lista {
            println!("({}, {})", i, j);
        }
    }
}

pub fn ex06_reducao_logaritmica(mut n: u64) {
    while n > 0 {
        println!("{}", n);
        n /= 2;
    }
}

pub fn ex07_fibonacci_recursivo(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        ex07_fibonacci_recursivo(n - 1) + ex07_fibonacci_recursivo(n - 2)
    }
}

pub fn ex08_bubble_sort(lista: &mut [i32]) {
    let n = lista.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if lista[j] > lista[j + 1] {
                lista.swap(j, j + 1);
            }
        }
    }
}

pub fn ex09_produto_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
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
        assert_eq!(ex01_acesso_constante(&[10, 20, 30]), Some(10));
    }

    #[test]
    fn test_ex02() {
        assert_eq!(ex02_soma_linear(&[1, 2, 3]), 6);
    }

    #[test]
    fn test_ex03() {
        assert_eq!(ex03_busca_binaria(&[1, 2, 3, 4, 5], 4), Some(3));
    }

    #[test]
    fn test_ex04() {
        assert_eq!(ex04_encontrar_pares(&[1, 2, 3, 4], 5), vec![(1, 4), (2, 3)]);
    }

    #[test]
    fn test_ex07() {
        assert_eq!(ex07_fibonacci_recursivo(5), 5);
    }

    #[test]
    fn test_ex08() {
        let mut v = [3, 1, 2];
        ex08_bubble_sort(&mut v);
        assert_eq!(v, [1, 2, 3]);
    }

    #[test]
    fn test_ex10() {
        assert_eq!(ex10_merge_sort(vec![5, 1, 4, 2]), vec![1, 2, 4, 5]);
    }
}
