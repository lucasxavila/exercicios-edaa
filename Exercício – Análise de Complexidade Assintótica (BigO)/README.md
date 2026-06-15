# Exercício: Análise de Complexidade Assintótica (Big O)

Esta pasta contém a análise de complexidade de 10 algoritmos propostos no PDF "Análise de Complexidade Assintótica (BigO).pdf". Para validar as análises, os algoritmos foram implementados em Rust.

## Como Executar

### 1. Pré-requisitos
- Ter o **Rust** e o **Cargo** instalados em sua máquina.

### 2. Executando os Testes
Navegue até a pasta do projeto Rust e execute os testes automatizados para verificar a corretude das implementações:

```bash
cd Respostas/bigo_analysis
cargo test
```

## Tabela de Complexidade Assintótica (Pior Caso)

| Exercício | Algoritmo | Complexidade | Justificativa |
| :--- | :--- | :--- | :--- |
| 01 | Acesso Constante | $O(1)$ | O acesso ao primeiro elemento de um array ocorre em tempo fixo, independente do tamanho $n$. |
| 02 | Soma Linear | $O(n)$ | O algoritmo percorre cada um dos $n$ elementos da lista exatamente uma vez para realizar a soma. |
| 03 | Busca Binária | $O(\log n)$ | A cada iteração, o espaço de busca é reduzido pela metade, resultando em um crescimento logarítmico. |
| 04 | Encontrar Pares | $O(n^2)$ | Possui dois loops aninhados onde, no pior caso, cada elemento é comparado com todos os outros. |
| 05 | Loops Aninhados | $O(n^2)$ | Embora tenha um bloco linear, o bloco de loops aninhados $n \times n$ domina a complexidade final. |
| 06 | Redução Logarítmica | $O(\log n)$ | O valor de $n$ é dividido por 2 em cada iteração até chegar a zero. |
| 07 | Fibonacci Recursivo | $O(2^n)$ | Cada chamada gera duas novas chamadas recursivas, criando uma árvore de execução exponencial. |
| 08 | Bubble Sort | $O(n^2)$ | No pior caso (lista inversamente ordenada), realiza $n \times n$ comparações e trocas. |
| 09 | Produto de Matrizes | $O(n \cdot m \cdot p)$ | Triplo loop aninhado para processar linhas e colunas das duas matrizes. |
| 10 | Merge Sort | $O(n \log n)$ | Divide a lista logaritmicamente e realiza a intercalação (merge) linear em cada nível. |

---

## Detalhes das Implementações em Rust
- **ex01**: Retorna o primeiro item de um slice usando `.first()`.
- **ex02**: Utiliza iteradores e o método `.sum()` para somar os valores.
- **ex03**: Implementação clássica com ponteiros `baixo` e `alto`.
- **ex04**: Retorna um `Vec` com tuplas dos pares encontrados.
- **ex07**: Fibonacci puro para demonstrar o custo da recursão sem memorização.
- **ex08**: Ordenação *in-place* utilizando `lista.swap()`.
- **ex10**: Utiliza `split_off` para dividir os vetores e recursão para ordenar.
