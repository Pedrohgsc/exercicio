Apresentação do Problema
Motivação:
O planejamento e controle de produção (PCP) é fundamental para a eficiência de uma fábrica. Um sistema bem implementado permite a otimização do uso de matérias-primas, minimizando o estoque e evitando desperdícios.

Problema Abordado:
O problema consiste em calcular a quantidade de matérias-primas necessárias para a produção de diversos produtos, cada um com diferentes subconjuntos de matérias-primas, e determinar quando e quanto comprar dessas matérias-primas para cumprir as datas de entrega dos produtos com o menor custo e desperdício possíveis.

Objetivo da Pesquisa:
Desenvolver um sistema de PCP que otimize a produção e fornecimento de matérias-primas, garantindo que as matérias-primas sejam compradas no momento certo e na quantidade correta, minimizando os estoques e evitando desperdícios.

Complexidade do Problema
Classificação:
O problema de planejamento e controle de produção pode ser classificado como um problema de otimização combinatória.

Justificativa:

P: Problemas que podem ser resolvidos em tempo polinomial. Nosso problema pode, em casos simples, ser resolvido em tempo polinomial com um algoritmo guloso, mas em geral não é garantido que todas as instâncias do problema possam ser resolvidas dessa forma.
NP: Problemas cuja solução pode ser verificada em tempo polinomial.
NP-Completo: Problemas que são tanto NP quanto qualquer outro problema NP pode ser reduzido a eles em tempo polinomial. O problema de PCP, em geral, não se enquadra diretamente em NP-Completo, mas pode ter aspectos que são NP-Hard, como a otimização de custos e a combinação de pedidos.
Algoritmos Conhecidos
Algoritmos:

Algoritmo Guloso (Greedy Algorithm):
Características: Toma a decisão ótima em cada etapa localmente, sem considerar o impacto global.
Aplicação: Pode ser usado para aproximar soluções em problemas simples de PCP.
Programação Dinâmica (Dynamic Programming):
Características: Resolve problemas subproblems menores e combina suas soluções. Ideal para problemas que possuem sobreposição de subproblemas.
Aplicação: Utilizado para problemas de otimização onde as decisões dependem de estados anteriores.
Branch and Bound:
Características: Explora todas as possíveis soluções, eliminando sub-árvores que não podem conter a solução ótima.
Aplicação: Utilizado em problemas de otimização combinatória complexos.
Algoritmos Heurísticos e Meta-Heurísticos (e.g., Algoritmos Genéticos, Simulated Annealing):
Características: Não garantem a solução ótima, mas fornecem boas soluções em tempo razoável.
Aplicação: Utilizados quando os métodos exatos são impraticáveis devido à complexidade do problema.
Algoritmo Escolhido e Implementação
Algoritmo Escolhido:
Para este problema, escolhemos um algoritmo guloso (Greedy Algorithm) simplificado devido à sua facilidade de implementação e eficiência para problemas menores.

Fonte:
Baseado em conceitos de algoritmos gulosos e planejamento de produção.

Implementação e Versionamento:
O código do algoritmo foi implementado em Rust e versionado no GitHub. Repositório GitHub

Complexidade do Algoritmo Escolhido
Classificação Big-O:
O algoritmo guloso escolhido tem uma complexidade de O(n), onde n é o número de produtos. A parte mais complexa é a iteração sobre a lista de produtos e matérias-primas para calcular as necessidades.

Paradigma e Estratégia do Algoritmo Escolhido
Estratégias Utilizadas:

Guloso: Toma decisões baseadas na melhor escolha local em cada etapa.
Divisão e Conquista: Embora o algoritmo não use explicitamente divisão e conquista, a estrutura de calcular necessidades e consolidar pedidos pode ser vista como uma forma simplificada desse paradigma.
Comparação com Outro Algoritmo
Outro Algoritmo:
Compararemos o algoritmo guloso com a programação dinâmica.

Programação Dinâmica:

Complexidade: Pode ser mais complexa, geralmente O(n^2) ou mais, dependendo da sobreposição de subproblemas.
Paradigma: Divide o problema em subproblemas menores, resolve cada um e combina as soluções.
Comparação:

Guloso: Simples e rápido, mas pode não encontrar a solução ótima em todos os casos.
Programação Dinâmica: Mais robusto e pode garantir a solução ótima, mas é mais complexo e pode ser mais lento.
Linguagem
Uso do Rust:
Rust foi utilizado no desenvolvimento do algoritmo devido às suas características de segurança e desempenho. O gerenciamento de memória seguro e a ausência de coleta de lixo fazem de Rust uma escolha sólida para aplicações que exigem alta performance e confiabilidade.

No código, utilizamos:

Estruturas de Dados: Vetores e HashMaps para organizar os produtos e matérias-primas.
Biblioteca Chrono: Para manipulação de datas.
Gerenciamento de Erros: A natureza de Rust de forçar o tratamento de erros contribui para a robustez do código.
Com isso, o sistema de PCP em Rust proporciona um bom equilíbrio entre eficiência e segurança, sendo ideal para o ambiente de produção.
