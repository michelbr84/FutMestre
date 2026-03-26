# Changelog — FutMestre

Todas as mudancas notaveis do projeto serao documentadas neste arquivo.

O formato segue [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/).

---

## [Nao Lancado]

### Adicionado
- Ratings de desempenho individual por jogador (1.0-10.0) com "Craque da Partida"
- Criterios de desempate por confronto direto (head-to-head) na classificacao
- Noticias de promocao e rebaixamento ao fim de temporada com mensagens no inbox
- Emprestimos bancarios com juros (BankLoan com pagamento mensal automatico)
- Jogadores livres: agentes livres quando contrato expira
- Contra-propostas nas negociacoes de transferencias na TUI
- IA de mercado ativa: clubes controlados pela IA fazem transferencias semanais
- Rumores de transferencias durante janela de mercado
- IA tatica adaptativa durante partida (ajusta mentalidade e tempo pelo placar)
- IA de treinamento por posicao (recomenda foco baseado na posicao do jogador)
- Categorias de base: geracao anual de 2-5 jovens por clube baseado em academia e divisao
- Staff tecnico afetando efetividade do treino (bonus de coaching e fitness)
- Historico de temporadas anteriores por clube (SeasonRecord) e jogador (PlayerSeasonStats)
- Artilharia por competicao (TopScorer com gols e assistencias)
- Objetivo tracking para modo Carreira Serie D com mensagens de promocao
- Tela de vitoria ao conquistar Serie A no modo CareerSerieD
- Settings expandidos na TUI: wage display, match speed, auto-save interval
- Save compressed on/off e background matches on/off no config
- Multiplos slots de save com tamanho de arquivo e data de criacao
- Exportacao de estatisticas do jogo para arquivo texto
- CI/CD com GitHub Actions (check, fmt, clippy, test, build Windows/Linux)
- CHANGELOG.md (este arquivo)

---

## [0.1.0] — 2026-03-26

### Adicionado
- Motor de partida probabilistico minuto-a-minuto com 90 min
- Cartoes amarelos e vermelhos com segundo amarelo automatico
- Lesoes durante partida baseadas em fitness
- Substituicoes durante partida (max 3)
- Modificadores taticos (mentalidade, pressao, formacao)
- Bola parada: escanteios, faltas, penaltis, laterais
- Fadiga acumulada durante partida
- Narracoes em portugues para todos os eventos
- Tempo extra (30 min) e disputa de penaltis para copas
- 80 clubes brasileiros (Serie A ate D) com 1.760 jogadores
- 4 divisoes com promocao/rebaixamento (top 3 / bottom 3)
- Copa do Brasil em formato mata-mata
- Sistema financeiro completo (bilheteria, TV, patrocinio, merchandising, FFP)
- Mercado de transferencias com janelas, emprestimos, clausulas
- 5 sistemas de IA (transferencias, escalacao, taticas, diretoria, imprensa)
- 6 personalidades de gerente IA
- 4 tipos de treino com intensidade e risco de lesao
- Desenvolvimento por idade (Sub-21 bonus, 30+ declinio)
- Salvamento comprimido (gzip) com SHA256 e auto-save
- TUI completa com 9 abas navegaveis
- Simulacao ao vivo de partida com eventos minuto-a-minuto
- GUI desktop via Tauri com glassmorphism e 19 comandos
- i18n com 4 idiomas (EN, PT-BR, ES, FR) e 4 moedas
- CLI com comandos para simulacao e gestao de jogo
- REST API via Axum
- 80 estadios brasileiros reais e 20 arbitros com perfis
- Sistema de telemetria (tracing e metricas)
- Persistencia SQLite via cm_data
- 551+ testes passando em todo o workspace
