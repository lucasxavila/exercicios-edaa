# Atividade da Semana: Reescrita de Algoritmos em Rust

Este repositório contém a reescrita de 10 algoritmos de Python para Rust, como parte da disciplina de Estrutura de Dados e Algoritmos.

## Como Executar os Testes

Certifique-se de ter o Rust e o Cargo instalados. Navegue até a pasta `rust_exercises` e execute:

```bash
cargo test
```

## Análise de Complexidade (Big O)

| Exercício | Algoritmo | Complexidade de Tempo | Complexidade de Espaço |
| :--- | :--- | :--- | :--- |
| 01 | Verificar Primeiro | $O(1)$ | $O(1)$ |
| 02 | Somar Lista | $O(n)$ | $O(1)$ |
| 03 | Busca Binária | $O(\log n)$ | $O(1)$ |
| 04 | Pares com Soma | $O(n^2)$ | $O(1)$ |
| 05 | Imprimir Pares e Pares | $O(n^2)$ | $O(1)$ |
| 06 | Potências de Dois | $O(\log n)$ | $O(1)$ |
| 07 | Fibonacci Recursivo | $O(2^n)$ | $O(n)$ (pilha de recursão) |
| 08 | Ordenação Bolha | $O(n^2)$ | $O(1)$ |
| 09 | Produto de Matrizes | $O(n \cdot m \cdot p)$ | $O(n \cdot p)$ |
| 10 | Merge Sort | $O(n \log n)$ | $O(n)$ |

## Descrição dos Exercícios

1.  **ex01_verificar_primeiro**: Retorna o primeiro elemento de uma lista usando `Option`.
2.  **ex02_somar_lista**: Calcula a soma de todos os elementos de um slice.
3.  **ex03_busca_binaria**: Implementação clássica de busca binária em lista ordenada.
4.  **ex04_pares_com_soma**: Encontra pares em uma lista que somam um valor alvo (Brute Force).
5.  **ex05_imprimir_pares_e_pares**: Demonstra loops sequenciais e aninhados.
6.  **ex06_potencias_de_dois**: Gera potências de 2 até um limite $n$.
7.  **ex07_fibonacci_recursivo**: Cálculo de Fibonacci via recursão simples.
8.  **ex08_ordenacao_bolha**: Ordenação de um slice mutável usando Bubble Sort.
9.  **ex09_produto_de_matrizes**: Multiplicação de matrizes representadas por `Vec<Vec<i64>>`.
10. **ex10_merge_sort**: Ordenação estável usando a estratégia de Dividir e Conquistar.
