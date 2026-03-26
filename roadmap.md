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
- [x] Substituicoes durante partida (max 3)
- [x] Ratings de desempenho individual por jogador (1-10, craque da partida)

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
- [x] Criterios de desempate avancados (confronto direto via head-to-head)
- [x] Noticias de promocao/rebaixamento no fim de temporada

- [x] Calendario com datas FIFA (pausas internacionais em marco, junho, setembro, outubro, novembro)

---

# Fase 3 — Gestao Financeira

- [x] Bilheteria, patrocinio, premiacoes
- [x] Receita de TV por divisao + merchandising
- [x] Processamento semanal de salarios
- [x] Budget separado (transfer_budget + wage_budget)
- [x] MonthlyReport com breakdown completo
- [x] FFP compliance check
- [x] Aba Financas no TUI com saldo, orcamentos e folha salarial
- [x] Emprestimos bancarios com juros (BankLoan com pagamento mensal)

- [x] Graficos financeiros na GUI (historico mensal de receitas/despesas)

---

# Fase 4 — Mercado de Transferencias

- [x] Modelo de transferencia completo
- [x] Avaliacao de jogadores
- [x] Janela de transferencias (verao e inverno)
- [x] Emprestimos, clausulas, comissao de agentes
- [x] Work permit
- [x] Aba Transferencias no TUI com lista de jogadores disponiveis
- [x] Jogadores livres (fim de contrato gera agentes livres)
- [x] IA de mercado ativa (clubes IA fazem transferencias semanais)
- [x] Rumores de transferencia (mensagens aleatorias durante janela)

## Pendente (Fase 4)
- [ ] Leilao estilo Elifoot

---

# Fase 5 — Inteligencia Artificial

- [x] IA de transferencias, escalacao, taticas
- [x] IA de diretoria, imprensa, staff, scouting
- [x] 6 personalidades de IA
- [x] IA tatica adaptativa durante partida (ajusta mentalidade/tempo pelo placar)
- [x] IA de treinamento por posicao (recomenda foco baseado na posicao do jogador)

---

# Fase 6 — Treinamento e Desenvolvimento

- [x] 4 tipos de treino: Fisico, Tecnico, Tatico, Recuperacao
- [x] Intensidade com ganho, fitness e risco de lesao
- [x] Desenvolvimento por idade (Sub-21 bonus, 30+ declinio)
- [x] Geracao de jovens procedural
- [x] Aba Treino no TUI com selecao de foco e status do elenco
- [x] Categorias de base (academia gera 2-5 jovens/ano baseado em reputacao e divisao)
- [x] Staff tecnico especializado afetando treino (bonus de coaching)

---

# Fase 7 — Sistema de Salvamento

- [x] Salvamento comprimido (gzip) com SHA256
- [x] SaveMetadata e auto-save
- [x] Versionamento com migracao
- [x] UI de multiplos slots de save (list_saves com file_size/created_at)
- [x] Exportacao de estatisticas (tabela, elenco, financas em TXT)

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
- [x] Simulacao ao vivo de partida com eventos minuto-a-minuto
- [x] Selecao de titulares/reservas na aba Elenco (Enter para trocar)
- [x] Negociacao interativa de transferencias (selecionar + oferta)
- [x] Carregar jogo salvo no menu principal
- [x] Substituicoes durante partida (max 3)
- [x] Academia de jovens (aba dedicada com sub-21 e potencial)
- [x] Salvar jogo (Ctrl+S)
- [x] Estadios brasileiros reais e arbitros com perfis
- [x] UI de negociacao com contra-propostas (exibe valor, aceitar/recusar)

---

# Fase 9 — Interface GUI (Tauri/Desktop)

- [x] Projeto Tauri com frontend web
- [x] State Machine de telas e navegacao
- [x] Layout CM-style com glassmorphism
- [x] Integracao completa com engine real (19 comandos Tauri)
- [x] Drag & drop para taticas com mapa de formacoes
- [x] Formacoes mudam posicoes dos bonecos ao trocar dropdown
- [x] Mentalidade e ritmo na tela de taticas

- [x] Match day ao vivo com animacoes melhoradas (ticker de eventos, pulsing em gols)
- [x] Substituicoes durante partida na GUI (painel de banco, max 3)
- [x] Estatisticas individuais de jogadores na partida (ratings, craque da partida)
- [x] Overview de partida (placar, minuto, eventos, estatisticas)

---

# Fase 10 — Dados do Jogo

- [x] 80 clubes brasileiros (Serie A ate D)
- [x] 1.760 jogadores gerados com nomes brasileiros reais
- [x] 16 nacoes (10 sul-americanas + 6 europeias)
- [x] 5 competicoes (4 ligas + Copa do Brasil)
- [x] Atributos proporcionais a reputacao do clube
- [x] Estadios brasileiros com capacidades reais (80 estadios)
- [x] 20 arbitros brasileiros com perfis de personalidade
- [x] Staff tecnico completo (treinadores especializados afetando treino)

## Pendente (Fase 10)
- [ ] Dados configuraveis via JSON

---

# Fase 11 — Polimento e Jogabilidade

- [x] Cores de destaque: verde=titular, amarelo=selecionado, vermelho=lesionado
- [x] Zonas de promocao (verde) e rebaixamento (vermelho) na classificacao
- [x] Footer com atalhos de teclado por aba
- [x] Sistema i18n com 4 idiomas (EN, PT-BR, ES, FR) e 4 moedas (BRL, USD, EUR, GBP)
- [x] Banco de mensagens multilingue (messages_db.js) com templates por categoria
- [x] Mensagens aleatorias diarias (treino, olheiros, torcida, imprensa, diretoria)
- [x] Resultados de rodada visiveis (todos os jogos, nao so do usuario)
- [x] Settings acessivel durante o jogo (engrenagem na barra de tabs)
- [x] Historico de temporadas anteriores (SeasonRecord por clube e PlayerSeasonStats)
- [x] Artilharia e premiacoes (TopScorer por competicao)

- [x] Flashing text para noticias urgentes (MessagePriority: Normal/Important/Urgent)
- [x] Background matches (placares ao vivo de todas as partidas da rodada)

## Pendente (Fase 11)
- [ ] Sons e musica

---

# Fase 12 — Modos de Jogo

- [x] GameMode enum (Sandbox, CareerSerieD)
- [x] Tela de selecao de modo antes de criar carreira
- [x] Filtro de clubes por modo (Serie D apenas no Desafio)
- [x] Objetivo tracking para CareerSerieD (progresso de promocao)
- [x] Tela de vitoria ao conquistar Serie A (mensagem e objetivo "CAMPEAO!")

## Pendente (Fase 12)
- [ ] Modo historia com cutscenes de texto
- [ ] Modo desafio com restricoes (orcamento minimo, elenco fraco)

---

# Fase 13 — Settings Estilo CM 01/02

- [x] Idioma (4 linguas)
- [x] Moeda (4 opcoes)
- [x] Zoom (100-150%)
- [x] GameMode na config
- [x] Exibicao de salarios: semanal / mensal / anual (WageDisplay no config)
- [x] Auto-save por intervalo configuravel (auto_save_interval em dias)
- [x] Velocidade de comentario (match speed 1-5)
- [x] Background matches on/off (background_matches no config)
- [x] Save compressed on/off (save_compressed no config)
- [x] Settings expandidos na TUI (wage_display, match_speed, auto_save_interval editaveis)

## Pendente (Fase 13)
- [ ] Flashing text on/off
- [ ] Tamanho do banco de dados (min/normal/max)
- [ ] Foreground/background leagues
- [ ] Add manager (multiplayer hot-seat)
- [ ] Board confidence / board request
- [ ] Scouting (observar time, pais, regiao, competicao, proximo adversario)
- [ ] Reserve team / control reserve
- [ ] Resign from club

---

# Fase 14 — Testes e Qualidade

- [x] 558+ testes passando em todo o workspace
- [x] Testes de motor de partida, financas, transferencias, IA, competicoes, treinamento
- [x] CI/CD com GitHub Actions (check, fmt, clippy, test, build multi-OS)

- [x] Testes de integracao para game loop completo (7 dias, match day, financas)

## Pendente (Fase 14)
- [ ] Cobertura > 70%

---

# Fase 15 — Release

- [x] README.md e documentacao
- [x] Roadmap unificado
- [x] Changelog (CHANGELOG.md)

## Pendente (Fase 15)
- [ ] Guia do jogador
- [ ] Release v1.0 com binarios

---

# Backlog (pos-v1.0)

- [ ] Editor de dataset (usuario pode adicionar mensagens, jogadores, etc)
- [ ] Replay de partidas
- [ ] Multithreading
- [ ] Web UI alternativa
- [ ] Mod support
- [ ] Multiplayer online
- [ ] 10+ idiomas (Danish, Dutch, German, Italian, Norwegian, Swedish)
- [ ] Impressao de relatorios (TXT/PDF)
- [ ] Fontes customizaveis na interface
- [ ] Leilao estilo Elifoot

---

## Prioridades Atuais

1. **Sons e musica** — efeitos sonoros para eventos de partida
2. **Guia do jogador** — documentacao de como jogar
3. **Release v1.0** — binarios para Windows/Linux
4. **Cobertura > 70%** — testes adicionais
5. **Leilao estilo Elifoot** — sistema de leilao para transferencias
