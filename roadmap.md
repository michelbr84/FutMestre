# Roadmap — FutMestre

Jogo de gerenciamento de futebol inspirado em Championship Manager 01/02 e Elifoot 98.
Feito em Rust com interface TUI (terminal) e desktop via Tauri.

**Stack:** Rust (engine) + Ratatui (TUI) + Tauri (GUI) + JSON dados.

---

## Legenda
- [x] Concluido
- [~] Em andamento
- [ ] Nao iniciado

---

# Fase 0 — Fundacao e Qualidade

## 0.1 Documentacao e padroes
- [x] README.md completo em portugues
- [x] .gitignore configurado (Rust, Tauri, saves, IDE, OS)
- [x] Roadmap detalhado (este arquivo)
- [x] CLAUDE.md com instrucoes para Claude Code
- [x] Padronizar nomenclatura de telas/rotas (Inbox/Squad/Tactics/Transfers/Finance/Competitions)
- [ ] Definir convencao de IDs e versionamento de schema JSON
- [ ] Checklist de release: versao, changelog, build, smoke test, pacote final

## 0.2 Logging e diagnosticos
- [x] Logs de UI (navegacao, erros de JSON, falha de load/save)
- [x] Sistema de telemetria (cm_telemetry: tracing, metricas)
- [ ] Logs do backend (comandos tauri, erros de simulacao, save)
- [ ] Painel "Debug" opcional (dev-only)

## 0.3 Workspace e infra
- [x] Workspace Rust com 15 crates (DDD)
- [x] Modelos de dominio (Jogador, Clube, Competicao, Nacao, Estadio)
- [x] Sistema de atributos (40+ atributos: tecnicos, mentais, fisicos, goleiro)
- [x] Importador de dados JSON com nomes brasileiros reais
- [x] Persistencia SQLite via cm_data
- [x] Utilitarios (RNG, hashing, tempo) via cm_utils

---

# Fase 1 — Motor de Partida

## 1.1 Simulacao base
- [x] Simulacao probabilistica minuto-a-minuto (90 min)
- [x] Calculo de vantagem ataque/defesa com RNG semeado
- [x] Eventos: gols, inicio/fim de tempo
- [x] Vantagem de jogar em casa (+3 ataque, +2 meio-campo, +2 moral)
- [x] Tempo extra (30 min) para copas + disputa de penaltis

## 1.2 Cartoes e disciplina
- [x] Sistema de faltas (~14% chance por minuto por time)
- [x] Cartoes amarelos e vermelhos
- [x] Segundo amarelo = vermelho automatico
- [x] Reducao de efetividade por expulsao (-8%)

## 1.3 Lesoes durante partida
- [x] Chance de lesao por minuto baseada em fitness
- [x] Severidade (leve 1-7d, moderada 7-28d, grave 28-90d, severa 90-270d)

## 1.4 Taticas, Bola Parada, Fadiga, Comentarios
- [x] Modificadores taticos (mentalidade, pressao, formacao)
- [x] Escanteios (3%), faltas (5%), penaltis (75%), laterais (1%)
- [x] Fadiga acumulada durante partida
- [x] Narracoes em portugues para todos os eventos

## 1.5 Integracao com Engine
- [x] MatchSystem conectado ao cm_match (simulacao real, nao stub)
- [x] Resultados atualizam tabela de classificacao automaticamente
- [x] Relatorios de jogo enviados para inbox do usuario

## Pendente (Fase 1)
- [ ] Substituicoes durante partida (3 + tempo extra)
- [ ] Ratings de desempenho individual por jogador

---

# Fase 2 — Competicoes e Calendario

- [x] Geracao de fixtures (ida e volta, round-robin)
- [x] Tabela de classificacao (pontos, saldo, gols)
- [x] DivisionLevel enum (Serie A, B, C, D)
- [x] Promocao e rebaixamento (top 3 sobem, bottom 3 descem)
- [x] Copa do Brasil (mata-mata com Serie A + B)
- [x] 4 divisoes com 20 times cada (80 clubes brasileiros)
- [x] Geracao automatica de fixtures ao iniciar novo jogo
- [x] Exibicao de proximos jogos e resultados na aba Jogos

## Pendente (Fase 2)
- [ ] Calendario completo com datas FIFA
- [ ] Criterios de desempate avancados (confronto direto)
- [ ] Noticias de promocao/rebaixamento no fim de temporada

---

# Fase 3 — Gestao Financeira

- [x] Bilheteria, patrocinio, premiacoes
- [x] Receita de TV por divisao + merchandising
- [x] Processamento semanal de salarios
- [x] Budget separado (transfer_budget + wage_budget)
- [x] MonthlyReport com breakdown completo
- [x] FFP compliance check
- [x] Aba Financas no TUI com saldo, orcamentos e folha salarial

## Pendente (Fase 3)
- [ ] Emprestimos bancarios com juros
- [ ] Graficos financeiros na GUI

---

# Fase 4 — Mercado de Transferencias

- [x] Modelo de transferencia completo
- [x] Avaliacao de jogadores
- [x] Janela de transferencias (verao e inverno)
- [x] Emprestimos, clausulas, comissao de agentes
- [x] Work permit
- [x] Aba Transferencias no TUI com lista de jogadores disponiveis

## Pendente (Fase 4)
- [ ] Leilao estilo Elifoot
- [ ] Jogadores livres (fim de contrato)
- [ ] IA de mercado ativa
- [ ] Rumores de transferencia

---

# Fase 5 — Inteligencia Artificial

- [x] IA de transferencias, escalacao, taticas
- [x] IA de diretoria, imprensa, staff, scouting
- [x] 6 personalidades de IA

## Pendente (Fase 5)
- [ ] IA tatica adaptativa durante partida
- [ ] IA de treinamento por posicao

---

# Fase 6 — Treinamento e Desenvolvimento

- [x] 4 tipos de treino: Fisico, Tecnico, Tatico, Recuperacao
- [x] Intensidade com ganho, fitness e risco de lesao
- [x] Desenvolvimento por idade (Sub-21 bonus, 30+ declinio)
- [x] Geracao de jovens procedural
- [x] Aba Treino no TUI com selecao de foco e status do elenco

## Pendente (Fase 6)
- [ ] Categorias de base (academia)
- [ ] Staff tecnico especializado afetando treino

---

# Fase 7 — Sistema de Salvamento

- [x] Salvamento comprimido (gzip) com SHA256
- [x] SaveMetadata e auto-save
- [x] Versionamento com migracao

## Pendente (Fase 7)
- [ ] UI de multiplos slots de save
- [ ] Exportacao de estatisticas

---

# Fase 8 — Interface TUI (Terminal)

- [x] Menu principal: Novo Jogo / Configuracoes / Sair
- [x] Selecao de clube com 80 times (nome, divisao, reputacao)
- [x] Input de nome do tecnico
- [x] Header com info do clube (nome, divisao, data, saldo, tecnico)
- [x] 8 abas navegaveis: Elenco, Taticas, Treino, Jogos, Classificacao, Financas, Transferencias, Noticias
- [x] Aba Elenco: tabela com nome, posicao, idade, OVR, fitness, forma, moral, valor
- [x] Aba Taticas: formacao, mentalidade, ritmo, pressao, linha, largura, passe (editaveis)
- [x] Aba Taticas: visualizacao ASCII da formacao
- [x] Aba Treino: selecao de foco + status do elenco
- [x] Aba Jogos: fixtures do time com resultados e proximos jogos
- [x] Aba Classificacao: 4 divisoes navegaveis com destaque do time do usuario
- [x] Aba Financas: saldo, orcamentos, folha salarial detalhada
- [x] Aba Transferencias: janela de mercado e jogadores disponiveis
- [x] Aba Noticias: inbox com mensagens do jogo
- [x] Avancar dia (Espaco/N) e semana (A)
- [x] Navegacao por Tab/BackTab e atalhos numericos (1-8)
- [x] Tela de Configuracoes (idioma, moeda)

## Pendente (Fase 8)
- [ ] Simulacao ao vivo de partida com eventos minuto-a-minuto
- [ ] Selecao de titulares/reservas na aba Elenco
- [ ] Negociacao interativa de transferencias
- [ ] Carregar jogo salvo

---

# Fase 9 — Interface GUI (Tauri/Desktop)

- [x] Projeto Tauri com frontend web
- [x] State Machine de telas e navegacao
- [x] Layout CM-style com glassmorphism

## Pendente (Fase 9)
- [ ] Telas completas no Tauri
- [ ] Match day ao vivo com animacoes
- [ ] Drag & drop para taticas

---

# Fase 10 — Dados do Jogo

- [x] 80 clubes brasileiros (Serie A ate D)
- [x] 1.760 jogadores gerados com nomes brasileiros reais
- [x] 16 nacoes (10 sul-americanas + 6 europeias)
- [x] 5 competicoes (4 ligas + Copa do Brasil)
- [x] Atributos proporcionais a reputacao do clube

## Pendente (Fase 10)
- [ ] Estadios brasileiros com capacidades reais
- [ ] Arbitros com perfis de personalidade
- [ ] Staff tecnico completo
- [ ] Dados configuraveis via JSON

---

# Fase 11 — Polimento e Jogabilidade

- [x] Cores de destaque: verde=titular, amarelo=selecionado, vermelho=lesionado
- [x] Zonas de promocao (verde) e rebaixamento (vermelho) na classificacao
- [x] Footer com atalhos de teclado por aba

## Pendente (Fase 11)
- [ ] Sistema de noticias expandido (resultados, lesoes, transferencias, rumores)
- [ ] Historico de temporadas anteriores
- [ ] Artilharia e premiacoes
- [ ] Multiplos idiomas na interface
- [ ] Sons e musica

---

# Fase 12 — Testes e Qualidade

- [x] 508+ testes passando em todo o workspace
- [x] Testes de motor de partida, financas, transferencias, IA, competicoes, treinamento

## Pendente (Fase 12)
- [ ] Cobertura > 70%
- [ ] Testes de integracao para game loop completo
- [ ] CI/CD com GitHub Actions

---

# Fase 13 — Release

- [x] README.md e documentacao
- [x] Roadmap unificado

## Pendente (Fase 13)
- [ ] Guia do jogador
- [ ] Changelog
- [ ] Release v1.0 com binarios

---

# Backlog (pos-v1.0)

- [ ] Editor de dataset
- [ ] Replay de partidas
- [ ] Multithreading
- [ ] Web UI alternativa
- [ ] Mod support
- [ ] Multiplayer online

---

## Prioridades Atuais

1. **Simulacao ao vivo** — match day interativo com eventos minuto-a-minuto
2. **Selecao de titulares** — arrastar jogadores entre titular/reserva
3. **Negociacao interativa** — propor transferencias e receber contra-propostas
4. **Carregar jogo salvo** — UI de load game no menu principal
5. **Dados expandidos** — estadios, arbitros, staff
