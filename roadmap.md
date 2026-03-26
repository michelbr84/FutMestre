# Roadmap — FutMestre

Jogo de gerenciamento de futebol inspirado em Championship Manager 01/02 e Elifoot 98.
Feito em Rust com interface desktop via Tauri.

**Stack:** Tauri (UI) + Rust (engine) + JSON por idioma.

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
- [ ] Painel "Debug" opcional (dev-only): estado atual, tela atual, careerId, seed

## 0.3 Workspace e infra
- [x] Workspace Rust com 15 crates (DDD)
- [x] Modelos de dominio (Jogador, Clube, Competicao, Nacao, Estadio)
- [x] Sistema de atributos (40+ atributos: tecnicos, mentais, fisicos, goleiro)
- [x] Importador de dados JSON (clubes, competicoes, nacoes, calendario)
- [x] Persistencia SQLite via cm_data
- [x] Utilitarios (RNG, hashing, tempo) via cm_utils
- [x] Formatacao (rustfmt) e linting (clippy)
- [x] Makefile e justfile com atalhos

---

# Fase 1 — Motor de Partida

## 1.1 Simulacao base
- [x] Simulacao probabilistica minuto-a-minuto (90 min)
- [x] Calculo de vantagem ataque/defesa com RNG semeado
- [x] Eventos basicos: gols, inicio/fim de tempo
- [x] Vantagem de jogar em casa (+3 ataque, +2 meio-campo, +2 moral)
- [x] Tempo extra (30 min) para copas
- [x] Disputa de penaltis (5 cobracoes + morte subita)

## 1.2 Cartoes e disciplina
- [x] Sistema de faltas (~14% chance por minuto por time)
- [x] Cartoes amarelos e vermelhos (referee.rs + discipline.rs integrados)
- [x] Segundo amarelo = vermelho automatico
- [x] Reducao de efetividade por expulsao (-8% por jogador expulso)

## 1.3 Lesoes durante partida
- [x] Chance de lesao por minuto baseada em fitness (injuries.rs integrado)
- [x] Severidade de lesao (leve 1-7d, moderada 7-28d, grave 28-90d, severa 90-270d)

## 1.4 Taticas
- [x] Modificadores taticos aplicados (mentalidade, pressao, formacao)
- [x] Bonus/penalidade de formacao para ataque e defesa (tactics.rs integrado)

## 1.5 Bola parada
- [x] Escanteios com chance de gol (3%)
- [x] Faltas com chance de gol (5%)
- [x] Penaltis com chance de gol (75%)
- [x] Tiros laterais com chance de gol (1%)

## 1.6 Estatisticas detalhadas
- [x] Struct MatchStats (posse, finalizacoes, chutes a gol, faltas, escanteios, cartoes)
- [x] Eventos tipados (MatchEvent com MatchEventType)
- [x] Acumulo de posse por minuto com calculo percentual

## 1.7 Fadiga
- [x] Fadiga acumulada durante partida (fatigue.rs integrado)
- [x] Degradacao de efetividade ao longo dos 90 minutos

## 1.8 Comentarios em portugues
- [x] Narracoes em portugues para todos os eventos (gols, cartoes, lesoes, faltas, escanteios, penaltis)
- [x] Comentarios de intervalo e fim de jogo

## Pendente (Fase 1)
- [ ] Substituicoes durante partida (3 + tempo extra)
- [ ] Desempenho individual por jogador (nao so por time)
- [ ] Ratings de desempenho por jogador (ratings.rs — struct existe, falta integrar no loop)

---

# Fase 2 — Competicoes e Calendario

## 2.1 Liga
- [x] Geracao de fixtures (ida e volta, round-robin)
- [x] Tabela de classificacao (pontos, saldo, gols)
- [x] DivisionLevel enum (Serie A, B, C, D) com metodos utilitarios
- [x] Deteccao de fim de temporada (is_season_complete)
- [x] Promocao e rebaixamento (top 3 sobem, bottom 3 descem)
- [x] Geracao de nova temporada (reset tabelas + novos fixtures)

## 2.2 Copa
- [x] Geracao de chave de copa (mata-mata com times da Serie A + B)
- [x] Calculo de rodadas eliminatorias

## Pendente (Fase 2)
- [ ] Sistema de 4 divisoes com 12-20 times cada (dados completos)
- [ ] Calendario completo da temporada (agosto-maio)
- [ ] Datas FIFA e pre-temporada
- [ ] Criterios de desempate (SG, GP, confronto direto)
- [ ] Exibir proximos jogos + resultados anteriores na Inbox/Competitions
- [ ] Final de temporada com noticias de promocao/rebaixamento

---

# Fase 3 — Gestao Financeira

## 3.1 Receitas
- [x] Bilheteria por partida (attendance * ticket_price)
- [x] Patrocinio baseado em reputacao
- [x] Premiacao por posicao final
- [x] Receita de TV por divisao (Serie A: 5M, B: 2M, C: 500K, D: 100K + bonus posicao)
- [x] Merchandising baseado em reputacao e resultados recentes

## 3.2 Despesas
- [x] Processamento semanal de salarios
- [x] Calculo de divida com juros

## 3.3 Orcamento
- [x] Budget separado (transfer_budget + wage_budget)
- [x] can_afford_transfer() e can_afford_wage()
- [x] process_transfer_expense() e process_transfer_income()

## 3.4 Relatorios
- [x] MonthlyReport com breakdown de receitas e despesas
- [x] Calculo automatico de lucro liquido e saldo final
- [x] Relatorios mensais gerados pelo finance_system (inclui TV e merchandising)

## 3.5 Fair Play Financeiro
- [x] FFP compliance check (90% expense allowance)
- [x] Regras financeiras (taxa, comissao de agente, salario minimo)

## Pendente (Fase 3)
- [ ] Emprestimos bancarios com juros
- [ ] Interface de financas com graficos na GUI

---

# Fase 4 — Mercado de Transferencias

## 4.1 Implementado
- [x] Modelo de transferencia (proposta → negociacao → aceita/rejeitada)
- [x] Avaliacao de jogadores (valor de mercado baseado em habilidade, idade, potencial)
- [x] Janela de transferencias (verao e inverno com datas)
- [x] Sistema de emprestimo com opcao de recall
- [x] Clausulas de contrato (rescisoria, sell-on %, bonus por jogo, buy-back)
- [x] Comissao de agentes baseada em greed level
- [x] Work permit (requisitos e chance de sucesso)
- [x] Negociacao com avaliacao de proposta, contra-proposta, asking price

## 4.2 Pendente
- [ ] Leilao estilo Elifoot (disputas entre clubes)
- [ ] Jogadores livres (fim de contrato)
- [ ] IA de mercado: clubes rivais comprando/vendendo ativamente
- [ ] Agentes de jogadores influenciando negociacoes
- [ ] Rumores de transferencia na caixa de entrada

---

# Fase 5 — Inteligencia Artificial

## 5.1 Implementado
- [x] IA de transferencias (avalia necessidades, calcula proposta, identifica alvos e jogadores vendaveis)
- [x] IA de escalacao (seleciona melhor 11 por formacao, substitutos, capitao)
- [x] IA de analise de elenco (identifica posicoes carentes, profundidade, elos fracos)
- [x] IA de diretoria (satisfacao, risco de demissao, decisoes de orcamento, expectativas)
- [x] IA tatica (recomenda formacao, mentalidade, pressao baseado no adversario)
- [x] IA de imprensa (gera perguntas e respostas com reacoes da midia)
- [x] IA de staff (contratacao/demissao, analise de necessidades, qualidade)
- [x] IA de scouting (avaliacao, relatorios, recomendacoes, comparacao de jogadores)
- [x] Sistema de personalidades (6 tipos: Defensivo, Equilibrado, Ofensivo, Jovens, Vencer, Financeiro)

## 5.2 Pendente
- [ ] IA tatica adaptativa (muda formacao durante partida conforme placar)
- [ ] IA de treinamento (prioriza desenvolvimento por posicao)
- [ ] Personalidade da diretoria (paciente vs exigente, afeta expectativas)
- [ ] IA de clubes rivais com diferentes estrategias de mercado

---

# Fase 6 — Treinamento e Desenvolvimento

## 6.1 Sistema de treino
- [x] 4 tipos de treino: Fisico, Tecnico, Tatico, Recuperacao
- [x] Intensidade (Baixa/Media/Alta) com ganho, custo de fitness e risco de lesao
- [x] Treino Fisico: melhora velocidade, resistencia, forca, aceleracao, agilidade
- [x] Treino Tecnico: melhora finalizacao, passe, drible, cruzamento, desarme
- [x] Treino Tatico: melhora posicionamento, decisao, antecipacao, visao, compostura
- [x] Recuperacao: restaura fitness (+5/+10/+15), sem risco de lesao

## 6.2 Desenvolvimento por idade
- [x] Sub-21: +50% ganho de atributo (bonus de desenvolvimento)
- [x] 21-29: ganho normal
- [x] 30+: -50% ganho, +50% risco de lesao
- [x] 33+: declinio aleatorio de -0.1 por semana (envelhecimento)

## 6.3 Envelhecimento e renovacao
- [x] process_aging(): declinio de veteranos em fim de temporada
- [x] generate_youth_player(): geracao procedural de jovens (16-19 anos, atributos 20-55, potencial 60-90)

## Pendente (Fase 6)
- [ ] Categorias de base (academia de jovens) com geracao anual
- [ ] Staff tecnico: treinadores especializados por area afetando qualidade do treino
- [ ] Moral do jogador afetado por titularidade e resultados

---

# Fase 7 — Sistema de Salvamento

- [x] Salvamento comprimido (gzip) com verificacao SHA256
- [x] Formato .cmsave com magic bytes e versao
- [x] SaveMetadata (nome, data_criacao, data_jogo, manager, clube)
- [x] list_saves(): escaneia diretorio e retorna metadados ordenados por data
- [x] should_auto_save(): auto-save a cada N dias do jogo
- [x] Versionamento de saves com migracao

## Pendente (Fase 7)
- [ ] UI de multiplos slots de save na GUI
- [ ] Exportacao de estatisticas em JSON

---

# Fase 8 — Interface Grafica (Tauri/Desktop)

## 8.1 Estrutura e navegacao
- [x] Projeto Tauri com frontend web
- [x] State Machine de telas
- [x] Layout CM-style: Top Bar / Tabs / Main Content / Bottom Bar
- [x] Persistencia de UI state por tela

## 8.2 Tela de idioma
- [x] Grid 5x5 com 25 bandeiras (flags.json)
- [x] Navegacao por teclado (setas/enter/esc)
- [x] Fallback visual (emoji → texto)

## 8.3 Localizacao
- [x] Loader de strings por idioma
- [x] Regras de fallback: pt-BR → pt → en
- [x] Auditoria de chaves ausentes

## 8.4 Menu Inicial
- [x] Carrega start.json do idioma
- [x] UI (titulo + botoes): iniciar / continuar / opcoes / sair
- [ ] Visual CM moderno: alinhamento, espacamento, estados hover/pressed
- [ ] "Safe exit": confirmar quando houver progresso nao salvo

## 8.5 Novo Jogo
- [x] Campos: nome / pais / lingua
- [x] Selecao de Time: 6 botoes (3x2) com 6 times aleatorios (sem repeticao)
- [x] Dropdown Pais e Lingua
- [x] Criacao de carreira + save inicial
- [ ] Validacao + mensagens de erro UI (campos obrigatorios)
- [ ] Seed deterministica para reproducibilidade

## 8.6 Inbox / Noticias
- [x] Inbox CM-style em 2 colunas (lista + leitura)
- [x] Tags, nao-lidas, horario
- [x] Pesquisa funcional
- [x] Botoes Continue / Save & Exit
- [x] Patch de template com dados reais da carreira

## 8.7 Elenco (Squad)
- [x] DataGrid: nome, posicao, condicao, moral, media, gols
- [x] Filtros: titulares/reservas/nao-relacionados
- [x] Badges de posicao coloridos (GK, DEF, MID, ATT)
- [x] Overall rating com cores
- [ ] Acoes: definir titular/reserva, ver perfil
- [ ] Ordenacao por coluna + busca rapida

## 8.8 Perfil do Jogador
- [x] Atributos 0-20 + cores
- [x] Historico e stats
- [ ] Contrato (salario, duracao, clausulas)
- [ ] Lesoes, forma, treino, moral detalhado

## 8.9 Taticas
- [x] Visualizacao de campo 2D
- [x] Instrucoes: mentalidade, passe, pressao
- [x] Drag & Drop de camisas para mudar posicao
- [ ] Formacao + papeis (GK/DEF/MID/ATT)
- [ ] Set pieces simplificado
- [ ] Setas de movimento (corrida com/sem bola)
- [ ] Validacao em tempo real (posicoes invalidas)
- [ ] Feedback de familiaridade com a tatica

## 8.10 Dia de Jogo (Match Day)
- [x] Pos-jogo: ratings e estatisticas
- [ ] Ao vivo: eventos por minuto + destaques (narracoes em portugues)
- [ ] Escalacao confirmada + substituicoes

## 8.11 Competicoes
- [x] Backend LeagueTable + command
- [x] UI table
- [x] Calendario (fixtures) + resultados
- [x] Navegacao por rodada/temporada

## 8.12 Transferencias
- [x] Search global
- [x] Offer transfer
- [x] Negociacao simples (accept/reject)
- [ ] Shortlist, observacao, scout reports
- [ ] Emprestimos + clausulas na UI
- [ ] UI de negociacao detalhada (salario, luvas, bonus, clausulas)
- [ ] Feedback do agente com contra-proposta
- [ ] Query Builder UI com filtros combinatorios ([Zagueiro] + [Brasileiro] + [Idade < 23])
- [ ] Rede de olheiros: designar por pais/regiao, retorno gradual

## 8.13 Financas
- [x] Salarios detalhados (folha, contratos/budget)
- [x] Ticketing / receitas
- [x] Patrocinios
- [x] Regras financeiras (FFP opcional)
- [x] Relatorios mensais

## 8.14 Opcoes
- [x] Idioma (troca + reload)
- [x] UI scale (4K)
- [x] Windowed/fullscreen
- [x] Velocidade de simulacao
- [x] Reset / limpar saves (com confirmacao)

## 8.15 Comandos Backend (Tauri)
- [x] get_squad(team_id)
- [x] advance_day()
- [x] save_game()
- [x] get_league_table()
- [x] start_match()
- [x] offer_transfer()
- [x] search_players()
- [x] get_player_details()
- [x] get_saved_games() / load_game()
- [ ] load_game(slot) com migracao
- [ ] new_game(payload) completo
- [ ] get_inbox() (mensagens do dia)
- [ ] get_next_fixtures() / get_recent_results()
- [ ] Tratamento de erro padrao (UI recebe erro amigavel + log detalhado)

---

# Fase 9 — CLI e TUI

- [x] CLI: novo-jogo, avancar-dia, simular-partida (Clap)
- [x] TUI: interface de terminal com Ratatui
- [ ] CLI completo com todos os comandos de gestao
- [ ] TUI com navegacao completa (elenco, taticas, mercado)

---

# Fase 10 — Dados do Jogo

- [x] Clubes ingleses de referencia (Liverpool, Arsenal, etc.)
- [ ] Clubes brasileiros completos (Serie A ate D, 80 times)
- [ ] Jogadores gerados proceduralmente com nomes regionais brasileiros
- [ ] Estadios brasileiros com capacidades reais
- [ ] Arbitros com perfis de personalidade
- [ ] Staff tecnico (tecnicos, olheiros, preparadores fisicos)
- [ ] Selecoes nacionais
- [ ] Dados de temporada configuraveis via JSON

---

# Fase 11 — Polimento e Jogabilidade

- [ ] Sistema de noticias (resultados, lesoes, transferencias, rumores)
- [ ] Historico de temporadas anteriores
- [ ] Recordes e estatisticas acumuladas
- [ ] Artilharia da competicao
- [ ] Premiacoes de final de temporada (melhor jogador, artilheiro, revelacao)
- [ ] Sistema de sons e musica de fundo
- [ ] Multiplos idiomas na interface (PT-BR, EN, ES)
- [ ] Modo multiplayer hot-seat (local)
- [ ] Consistencia visual entre telas (margens, fontes, cores, botoes)
- [ ] Barra de atalhos (teclas) no rodape
- [ ] Estados: hover/selected/disabled/unread
- [ ] Performance (scroll listas grandes, tabela, render)

---

# Fase 12 — Testes e Qualidade

- [x] Testes unitarios: 508 testes passando em todo o workspace
- [x] Testes de motor de partida (simulacao, taticas, fadiga, comentarios, ratings)
- [x] Testes de financas (bilheteria, patrocinio, premiacoes, TV, merchandising, FFP)
- [x] Testes de transferencias (negociacao, janela, avaliacao)
- [x] Testes de IA (escalacao, transferencias, taticas, diretoria, imprensa, scouting)
- [x] Testes de competicoes (divisoes, promocao/rebaixamento, copa)
- [x] Testes de treinamento (4 tipos, intensidade, idade, envelhecimento, geracao de jovens)
- [x] Testes de save (auto-save, listagem, metadados)
- [ ] Cobertura de testes > 70% em todos os crates
- [ ] Testes de integracao para game loop completo
- [ ] Testes de performance para simulacao de temporada
- [ ] Testes E2E para a interface Tauri
- [ ] Benchmarks de simulacao (1000 partidas < 5s)
- [ ] CI/CD com GitHub Actions

---

# Fase 13 — Documentacao e Release

- [x] README.md completo em portugues
- [x] .gitignore configurado
- [x] Roadmap unificado (este arquivo)
- [ ] Documentacao de arquitetura (cargo doc)
- [ ] Guia do jogador (como jogar)
- [ ] Guia de contribuicao
- [ ] Changelog organizado
- [ ] Release v1.0 com binarios para Windows/Linux/macOS
- [ ] Pagina do projeto no GitHub com screenshots

---

# Fase 14 — Empacotamento e Distribuicao

- [x] resources/JSON incluido no build Tauri
- [x] Ajuste do tauri.conf.json
- [x] Build final
- [ ] Instalador (MSI/NSIS) e assinatura (opcional)
- [ ] Verificacao em PC limpo (sem ambiente dev)

---

# Backlog (pos-v1.0)

- [ ] Editor de dataset
- [ ] Replay de partidas
- [ ] Multithreading para simulacao
- [ ] Persistencia full SQLite (substituir JSON)
- [ ] Web UI alternativa
- [ ] Mod support
- [ ] Multiplayer online

---

## Prioridades de Desenvolvimento

1. **Motor de partida** (Fase 1) — substituicoes e desempenho individual
2. **Dados brasileiros** (Fase 10) — identidade do jogo
3. **Interface GUI** (Fase 8) — telas pendentes: match day ao vivo, negociacao
4. **Competicoes completas** (Fase 2) — calendario e desempate
5. **Polimento** (Fase 11) — noticias, historico, sons

---

## Metricas de Sucesso

- [x] 508 testes passando sem falhas
- [x] Motor de partida com cartoes, lesoes, taticas, bola parada
- [x] Sistema financeiro com TV, merchandising, FFP
- [x] 4 divisoes com promocao/rebaixamento implementados
- [x] Treinamento com evolucao de atributos por idade
- [x] Auto-save e multiplos slots
- [ ] Jogar uma temporada completa sem bugs
- [ ] Simular 1000 partidas com resultados realistas
- [ ] IA fazendo transferencias inteligentes
- [ ] Promocao/rebaixamento funcionando em 10+ temporadas
- [ ] Tempo de carregamento < 2s
- [ ] Binario < 50MB
