# Questão 1 — Classificação do Problema

## a) Classificação (P, NP ou NP-Completo)

O problema de roteamento da FastBite pertence à classe **NP-Completo** (em sua versão de decisão) e é considerado **NP-Hard** (em sua versão de otimização).

**Justificativa:**
1.  **NP (Verificabilidade):** Dada uma solução candidata (um conjunto de rotas para os entregadores), é possível verificar se ela é válida (atende à capacidade máxima de 3 pedidos e prazos) e calcular seu custo total em tempo polinomial.
2.  **NP-Difícil (Complexidade):** O problema é uma variante do **Problema de Roteamento de Veículos (VRP)**, que por sua vez é uma generalização do **Problema do Caixeiro Viajante (TSP)**. Como o TSP é comprovadamente NP-Completo e o problema da FastBite contém instâncias do TSP (por exemplo, quando há apenas um entregador com capacidade ilimitada), o problema da FastBite é ao menos tão difícil quanto o TSP.
3.  **NP-Completo:** Como o problema está em NP e é NP-Difícil, sua versão de decisão é classificada como NP-Completo.

---

## b) Redução ao TSP (Traveling Salesman Problem)

A redução do problema da FastBite ao TSP pode ser feita da seguinte forma:

1.  **Simplificação da Instância:** Imagine um cenário onde existe apenas **um entregador** com capacidade suficiente para carregar todos os pedidos pendentes.
2.  **Mapeamento de Pontos:** Cada local de coleta (restaurante) e cada local de entrega (cliente) torna-se um "nó" no grafo do TSP.
3.  **Restrições de Precedência:** No TSP tradicional, a ordem de visita é livre. No problema da FastBite, adicionamos a restrição de que o nó de coleta deve ser visitado antes do nó de entrega correspondente.
4.  **Cálculo de Arestas:** As distâncias entre todos os pontos (calculadas via Métrica de Manhattan) representam o peso das arestas entre os nós.
5.  **Objetivo:** Encontrar o ciclo Hamiltoniano de menor peso que visite todos os nós de coleta e entrega, respeitando a precedência, equivale a resolver uma instância específica de TSP com restrições.

Dessa forma, qualquer algoritmo que resolva o TSP de forma ótima poderia ser adaptado para resolver o roteamento da FastBite, demonstrando a equivalência de complexidade.

---

## c) Inviabilidade da Força Bruta

A solução por força bruta é inviável em produção porque o número de combinações cresce de forma **fatorial**, fenômeno conhecido como "explosão combinatória".

### Estimativa para o Cenário (8 pedidos, 3 entregadores):

Para 8 pedidos e 3 entregadores, com capacidade máxima de 3 pedidos por entregador, a única distribuição possível é (3, 3, 2) pedidos por entregador (em qualquer ordem).

1.  **Particionamento dos Pedidos:**
    *   Escolher 3 pedidos para o E1: $\binom{8}{3} = 56$
    *   Escolher 3 pedidos para o E2: $\binom{5}{3} = 10$
    *   Restam 2 pedidos para o E3: $\binom{2}{2} = 1$
    *   Permutações dos entregadores: Como os entregadores são distintos, multiplicamos pela ordem das capacidades (3, 3, 2), (3, 2, 3), (2, 3, 3) $\rightarrow$ 3 formas.
    *   Total de formas de atribuir: $56 \times 10 \times 1 \times 3 = 1.680$ formas.

2.  **Permutações de Rota (Ordem de Entrega):**
    *   Entregador com 3 pedidos: $3! = 6$ ordens possíveis.
    *   Entregador com 2 pedidos: $2! = 2$ ordens possíveis.
    *   Total de rotas por atribuição: $6 \times 6 \times 2 = 72$.

3.  **Total de Combinações:**
    *   $1.680 \text{ (atribuições)} \times 72 \text{ (rotas)} = \mathbf{120.960}$ combinações.

### Conclusão:
Embora 120.960 pareça um número gerenciável para um computador moderno, em um sistema real com **80.000 pedidos/dia** (conforme mencionado no PDF), o número de combinações superaria rapidamente a quantidade de átomos no universo, tornando o processamento em menos de 2 segundos impossível via força bruta. A complexidade assintótica é de ordem **$O(n!)$**, característica de problemas NP-Difíceis.

# Questão 2 — Abordagem Gulosa (Greedy)

## a) Funcionamento Passo a Passo (Cenário Exemplo)

O algoritmo proposto pela equipe júnior segue a lógica de atribuição imediata baseada na distância atual e roteamento por proximidade.

### Fase 1: Atribuição de Pedidos
1.  **P1 (Restaurante em (0, 2)):** Entregadores disponíveis: E1 (dist 2), E2 (dist 6), E3 (dist 11). **Atribuído ao E1.**
2.  **P2 (Restaurante em (1, 1)):** Entregadores: E1 (dist 0), E2 (dist 6), E3 (dist 11). **Atribuído ao E1** (Capacidade atingida: 2/2).
3.  **P3 (Restaurante em (4, 4)):** Entregadores: E2 (dist 2), E3 (dist 5). **Atribuído ao E2.**
4.  **P4 (Restaurante em (6, 1)):** Entregadores: E2 (dist 3), E3 (dist 6). **Atribuído ao E2** (Capacidade atingida: 2/2).
5.  **P5 (Restaurante em (5, 5)):** Entregador: E3 (dist 6). **Atribuído ao E3.**

### Fase 2: Definição das Rotas (Vizinho mais Próximo)
*   **E1 (P1, P2):** Começa em (1,1). Vai para Rest P2 (dist 0) $\rightarrow$ Rest P1 (dist 2) $\rightarrow$ Cliente P1 (dist 7) $\rightarrow$ Cliente P2 (dist 7).
*   **E2 (P3, P4):** Começa em (5,3). Vai para Rest P3 (dist 2) $\rightarrow$ Rest P4 (dist 5) $\rightarrow$ Cliente P3 (dist 7) $\rightarrow$ Cliente P4 (dist 9).
*   **E3 (P5):** Começa em (7,6). Vai para Rest P5 (dist 3) $\rightarrow$ Cliente P5 (dist 7).

---

## b) Classificação como Algoritmo Guloso

Este algoritmo é classificado como **guloso (greedy)** porque ele toma a decisão que parece melhor no momento imediato (escolha local), sem considerar as consequências futuras para a otimização global do sistema.

**Propriedade de Escolha Local:** Em cada passo, ele seleciona o entregador ou o próximo ponto de entrega baseado unicamente na **menor distância imediata** (distância Manhattan). Ele assume que uma sequência de escolhas locais ótimas levará a uma solução globalmente eficiente, o que nem sempre é verdade.

---

## c) Contraexemplo de Subotimalidade

Imagine um cenário com dois pedidos (P_A e P_B) e dois entregadores (E1 e E2):
*   **Restaurante P_A:** (0, 0). **Cliente P_A:** (10, 10).
*   **Restaurante P_B:** (1, 0). **Cliente P_B:** (2, 0).
*   **E1:** em (0, 0).
*   **E2:** em (1, 0).

**Decisão Gulosa:**
1.  Para P_A, o mais próximo é E1 (dist 0). Atribui P_A para E1.
2.  Para P_B, o mais próximo agora é E2 (dist 0). Atribui P_B para E2.
*   **Resultado:** E1 viaja 20 unidades (ida e volta para (10,10)) e E2 viaja 2 unidades. Total: **22 unidades**.

**Solução Melhor (não gulosa):**
*   Atribuir P_A para E2 e P_B para E1.
*   Embora a escolha inicial de P_A para E1 parecesse perfeita (dist 0), a atribuição cruzada poderia permitir que E2 (que talvez seja uma moto mais rápida) fizesse a rota longa de forma mais eficiente, ou que E1 fizesse a rota curta e ficasse disponível mais rápido para um novo lote. (Neste exemplo simples de distância, o total seria o mesmo, mas se E1 tivesse capacidade para ambos e eles estivessem na mesma direção, o algoritmo guloso poderia separar pedidos que deveriam ir juntos).

---

## d) Complexidade de Tempo

A complexidade é **$O(n \times m)$** para a fase de atribuição, onde $n$ é o número de pedidos e $m$ o número de entregadores.

**Justificativa:**
*   Para cada um dos $n$ pedidos, o algoritmo percorre a lista de $m$ entregadores para encontrar o mais próximo.
*   A fase de roteamento para cada entregador é $O(k^2)$, onde $k$ é a capacidade (máximo 3), o que é constante ($3^2 = 9$).
*   Portanto, a complexidade total é dominada por $O(n \cdot m)$. Em um sistema com 10.000 pedidos e 1.000 entregadores, isso resulta em 10 milhões de operações, o que é facilmente processado em menos de 2 segundos.

# Questão 3 — Programação Dinâmica e Divisão e Conquista

## a) Programação Dinâmica (PD) para Roteamento Individual

A Programação Dinâmica é perfeitamente aplicável para otimizar a rota de um único entregador que já recebeu $k$ pedidos.

*   **Aplicabilidade:** Sim, o roteamento de um entregador com $k$ pedidos é uma instância pequena do TSP. Como o objetivo é minimizar a distância total percorrida visitando pontos obrigatórios, a PD oferece a solução ótima de forma muito mais eficiente que a força bruta.
*   **Subproblema:** O subproblema pode ser definido como `dp(mask, u)`, que representa a distância mínima para visitar o conjunto de pontos representados pelo bitmask `mask`, terminando no ponto `u`.
*   **Custo de Memória e Tempo:**
    *   **Tempo:** $O(2^k \cdot k^2)$. Para o limite da FastBite ($k=3$), isso resulta em apenas ~72 operações por entregador, o que é instantâneo.
    *   **Espaço:** $O(2^k \cdot k)$, para armazenar a tabela de estados da PD.
*   **Limite de Praticidade:** Esta solução começa a se tornar impraticável em tempo real (considerando o limite de 2 segundos para milhares de entregadores simultâneos) quando $k$ ultrapassa **15 a 20 pedidos**, devido ao crescimento exponencial $2^k$.

---

## b) Divisão e Conquista (D&C)

A estratégia de Divisão e Conquista pode ser aplicada ao problema global da FastBite através do particionamento geográfico.

*   **Subproblemas Independentes:** É possível dividir o problema se assumirmos que os entregadores de uma zona só atendem pedidos daquela zona. As condições seriam uma densidade de pedidos alta o suficiente para que não haja necessidade de deslocamentos entre zonas.
*   **Exploração Geográfica:** A cidade pode ser dividida em quadrantes ou setores (como os Setores A, B e C do cenário). O sistema resolve o problema de atribuição para o Setor A, depois para o Setor B, e assim por diante. Isso reduz o tamanho de $n$ (pedidos) e $m$ (entregadores) em cada execução, acelerando drasticamente o processamento.
*   **Limitações e Fronteiras:**
    *   **Subotimalidade:** Um entregador que está no limite do Setor A pode ser a melhor escolha para um restaurante que está a poucos metros, mas dentro do Setor B. A divisão rígida impede essa atribuição ótima.
    *   **Efeito de Borda:** Pedidos nas fronteiras tendem a ter soluções piores. Uma solução comum de engenharia é criar zonas com sobreposição (overlaps) ou processar as fronteiras como uma "zona extra" após o processamento das zonas principais.

# Questão 4 — Comparação das Abordagens

## Tabela Comparativa

| Critério | Greedy (Guloso) | Programação Dinâmica | Divisão e Conquista |
| :--- | :--- | :--- | :--- |
| **Qualidade da solução** | Média/Baixa (Subótima) | Ótima (Localmente) | Média (Efeito de borda) |
| **Complexidade de tempo** | Baixa ($O(n \cdot m)$) | Exponencial ($O(2^k)$) | Média ($O(n \log n)$) |
| **Complexidade de espaço** | Baixa ($O(n + m)$) | Média ($O(2^k \cdot k)$) | Média ($O(n)$) |
| **Viabilidade (≤ 2s)** | Altíssima | Sim (apenas para $k \le 15$) | Alta |
| **Escalabilidade** | Alta | Baixa (Globalmente) | Altíssima |
| **Facilidade de adaptação** | Muito Alta | Baixa | Média |

---

## Análise Crítica

Para o contexto operacional da FastBite, nenhuma das abordagens puras é ideal isoladamente. No entanto, a **abordagem mais adequada como solução principal seria uma combinação de Divisão e Conquista com Heurística Gulosa**.

A **Divisão e Conquista** (via particionamento geográfico) é essencial para garantir a escalabilidade do sistema, permitindo que milhares de pedidos sejam processados em paralelo por zonas. Dentro de cada zona, o algoritmo **Greedy** garante que as decisões de atribuição sejam tomadas instantaneamente, respeitando o limite de 2 segundos. A **Programação Dinâmica** entra como um "ajudante" final, sendo usada apenas para ordenar as 2 ou 3 entregas de cada entregador individualmente, garantindo que, uma vez que o entregador tenha seus pedidos, ele faça o melhor caminho possível entre eles. Essa arquitetura híbrida equilibra a necessidade de velocidade extrema com uma qualidade de serviço aceitável.

# Questão 5 — Solução de Engenharia Real

## a) O que é uma Heurística?

No contexto de algoritmos, uma **heurística** é uma técnica projetada para resolver um problema de forma mais rápida quando métodos clássicos (exatos) são lentos demais, ou para encontrar uma solução aproximada quando métodos exatos falham. Ela é uma "regra prática" que sacrifica a garantia de otimalidade em troca de velocidade e eficiência.

**Por que são preferíveis?** Em sistemas como o da FastBite, o custo de esperar 1 hora para encontrar a "rota perfeita" que economiza 100 metros é muito maior do que o benefício. O cliente prefere que o pedido saia em 2 segundos com uma rota "boa" do que esperar minutos por uma rota "ideal".

---

## b) Estrutura de uma Solução de Engenharia Real

Uma solução robusta para a FastBite poderia ser estruturada em 4 etapas:

1.  **Particionamento (Fase de Divisão):** O sistema divide a cidade em micro-regiões (ex: grades de 2km x 2km). Isso permite que o problema de 80.000 pedidos seja quebrado em centenas de problemas menores de 100 pedidos, que podem ser processados em paralelo.
2.  **Construção (Heurística Gulosa):** Para cada micro-região, aplica-se um algoritmo guloso para criar uma atribuição inicial rápida. Em milissegundos, temos uma solução funcional (mesmo que não seja a melhor).
3.  **Refinamento Local (Local Search):** Com o tempo restante (até os 2 segundos), o sistema tenta melhorar a solução inicial. Ele pode "trocar" pedidos entre entregadores próximos (2-opt ou swap) e verificar se a distância total diminuiu. Se a nova configuração for melhor, ela é adotada.
4.  **Limite de Tempo Estrito (Interruptor):** O algoritmo é do tipo *anytime*. Assim que o cronômetro atinge 1,9 segundos, o processo de refinamento é interrompido e a melhor solução encontrada até aquele momento é enviada para execução.

---

## c) Quando vale a pena buscar a solução ótima?

A busca pela solução ótima é razoável quando o **custo da computação é menor que o custo da ineficiência**, ou quando a escala é pequena o suficiente.

**Exemplo:** No transporte de órgãos para transplante ou cargas de altíssimo valor (como transporte de valores/diamantes). Nessas situações, há poucos pontos (restaurantes/clientes não são milhares), o tempo de processamento de alguns minutos é aceitável, e o erro de uma rota subótima pode ter consequências catastróficas ou prejuízos financeiros imensos que justificam o gasto computacional exaustivo.

# Questão 6 — Reflexão Crítica

### O Dilema do Engenheiro: Ideal vs. Possível

No contexto de sistemas de larga escala como a FastBite, a solução "bom o suficiente" torna-se a melhor decisão técnica quando o custo marginal para alcançar a perfeição (em termos de tempo de computação, infraestrutura e latência) supera os benefícios operacionais.

Devido à natureza **NP-Difícil** dos problemas de roteamento, a busca pela solução ótima global em tempo real é matematicamente inviável. Um engenheiro que insiste no ideal coloca em risco a disponibilidade do sistema: uma decisão que leva 30 segundos para ser "perfeita" causaria um engarrafamento de dados e atrasaria milhares de outras entregas.

Portanto, em engenharia de software, "bom o suficiente" não é mediocridade, mas sim **otimização pragmática**. A melhor decisão técnica é aquela que entrega o maior valor dentro das restrições do mundo real (2 segundos de processamento e milhares de pedidos simultâneos), aceitando uma pequena margem de erro para garantir a escalabilidade e a sobrevivência do negócio.
