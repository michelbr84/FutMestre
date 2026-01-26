# ToDo — CM Rust Modern GUI (Windows Executable)

Roadmap completo para transformar o jogo em um executável Windows com interface moderna 4K, simulação estilo CM01/02, e sem dependências de mídia externa (apenas emojis/código).  
**Stack atual:** Tauri (UI) + Rust (engine) + JSON por idioma.

---

## Legenda
- [ ] Não iniciado
- [~] Em andamento
- [x] Concluído
- [!] Urgente / bloqueador

---

# Phase 0 — Qualidade, Organização e Reprodutibilidade (Fundação)
## 0.1 Documentação e padrões
- [ ] [!] **Atualizar `ToDo.md`** com status real (feitos x pendentes) e links para issues/commits.
- [ ] Verificar se precisa atualizar **`README.md`** (setup, build, estrutura JSON, fluxo de telas).
- [ ] Verificar se precisa atualizar **`.gitignore`** (build artifacts, logs, saves, caches, dist).
- [ ] Padronizar nomenclatura de telas/rotas (Inbox/Squad/Tactics/Transfers/Finance/Competitions).
- [ ] Definir convenção de IDs (teams, players, messages) e versionamento de schema JSON.
- [ ] Checklist de release: versão, changelog, build, smoke test, pacote final.

## 0.2 Logging e diagnósticos
- [x] Logs de UI (navegação, erros de JSON, falha de load/save).
- [ ] Logs do backend (comandos tauri, erros de simulação, save).
- [ ] Painel “Debug” opcional (dev-only): estado atual, tela atual, careerId, seed, etc.

## 0.3 Testes mínimos automatizados
- [x] Validação de JSON (schema/required fields) para `flags.json`, `start.json`, `times.json`, `atual.json`.
- [x] Smoke test do build (rodar, trocar idioma, iniciar jogo, carregar save, advance day).

---

# Phase 1 — UI Shell + Fluxo de Telas (CM-style, 4K)
## 1. Estrutura do Projeto e Máquina de Estados
- [x] Inicialização do projeto GUI
- [x] State Machine (telas)
- [x] “Layout system” consistente: Top Bar / Tabs / Main Content / Bottom Bar (sem sobreposição)
- [x] Persistência de UI state por tela (scroll, item selecionado, filtros, etc.)

## 2. Tela 1: Seleção de Idioma (25 Bandeiras)
- [x] Carregamento de dados (`flags.json`)
- [x] Render Grid 5×5
- [x] Seleção → define `language_code`
- [x] Acessibilidade: navegação por teclado (setas/enter/esc)
- [x] Tratamento de fallback visual (emoji não renderiza → usar texto)

## 3. Sistema de Localização (Fallback)
- [x] Loader de strings por idioma
- [x] [!] Regras de fallback: `pt-BR → pt → en` + logs quando faltar arquivo/chave
- [ ] Hot-reload em dev (opcional)
- [x] Auditoria: detectar chaves ausentes entre idiomas

## 4. Tela 2: Menu Inicial (Start Menu)
- [x] Carrega `start.json` do idioma
- [x] UI (título + botões)
- [x] Ações: iniciar / continuar / opções / sair
- [ ] Visual CM moderno: alinhamento, espaçamento, estados hover/pressed
- [ ] “Safe exit”: confirmar quando houver progresso não salvo

## 5. Navegação por Tabs (Top Tabs)
- [x] [!] **Todas as abas devem funcionar**: Inbox, Squad, Tactics, Transfers, Finance, Competitions
- [ ] Rotas/telas correspondentes (placeholder mínimo se necessário)
- [ ] “Tab highlight” correto + estado persistente da aba atual
- [x] Garantir que a área clicável não seja bloqueada por header/overlays

---

# Phase 2 — New Game (correto) + Boot da Carreira
## 6. Tela 3: Novo Jogo (New Game Form)
- [x] Campos: nome / país / língua
- [x] [!] **Seleção de Time correta: 6 botões (3×2) com 6 times aleatórios**
  - [x] Carregar `times.json` do idioma (30 times)
  - [x] Sortear 6 sem repetição (seed estável por career)
  - [x] “Freeze selection” (não re-sorteia a cada render)
  - [x] Botões exibem nome + cores do time (sem imagens)
- [x] Dropdown País: fonte `paises.json` (25)
- [x] Dropdown Língua: fonte `lingua.json` (10) + pré-selecionar idioma da bandeira
- [ ] Validação + mensagens de erro UI (campos obrigatórios)
- [x] Ação Continuar: criar career + salvar slot inicial

## 7. Geração de Mundo (Engine / Dados)
- [x] Times (base)
- [x] Jogadores (base)
- [x] Técnicos (base)
- [ ] [!] Ajustar técnico faltante (arquivo tem 29 — completar 30)
- [x] Gerador de jogadores: garantir ~510 e distribuição por time (ex.: 17×30)
- [x] Criar 1 jogador do usuário (injetar no time escolhido)
- [x] Atribuição: posições, atributos, idade, potencial, moral, condição, contrato
- [ ] Seed determinística para reproducibilidade (careerId/seed)

---

# Phase 3 — Tela de Notícias (Inbox CM) + Estado Inicial
## 8. Tela 4: Notícias / Inbox (News Screen)
- [x] Template base (notícias)
- [x] Adaptação para CM-style
- [x] Renderização inicial
- [x] [!] Inbox real em 2 colunas:
  - [x] Lista à esquerda (mensagens, não-lidas, tags, horário)
  - [x] Leitura à direita (título, meta, corpo)
- [x] Integrar com `atual.json` do idioma como **estado inicial/template**
- [ ] “Patch” do template com dados reais da carreira (clube, manager, times A–F)
- [ ] Pesquisa: caixa “Pesquisar” funcional (filtrar mensagens)
- [x] Botões de ação: Continue / Save & Exit (posicionados corretamente)

## 9. Bugs urgentes de UI (bloqueadores)
- [x] Fix Continue atrás do header (z-index/clickability)
- [x] Enable tabs (navegação)
- [x] [!] Regressão: garantir que Continue permaneça clicável em todas as resoluções
- [x] Garantir que nenhum overlay bloqueie cliques (hit-test)

---

# Phase 4 — Loop de Jogo (Rodadas, Tabela, Promoção/Rebaixamento)
## 10. Fluxo de Rodadas (A×B, C×D, E×F)
- [x] Criar competição “mini liga” com 6 times selecionados
- [x] Gerar calendário de rodadas (home/away se aplicável)
- [x] Simular partidas por rodada usando o match engine
- [x] Atualizar tabela ao final de cada partida/rodada
- [ ] Exibir próximos jogos + resultados anteriores na Inbox/Competitions

## 11. Classificação e Regras
- [x] Tabela com: P, J, V, E, D, GP, GC, SG, % (opcional)
- [ ] Critérios de desempate (SG, GP, confronto direto opcional)
- [ ] Final de temporada:
  - [ ] 2 sobem
  - [ ] 2 caem
- [ ] “Nova temporada”: reset parcial + notícias + calendário novo

---

# Phase 5 — Integração Tauri ↔ Rust (Comandos e Persistência)
## 12. Conexão Backend Rust (Comandos)
- [x] Structs de exibição (Display*)
- [x] `get_squad(team_id)`
- [x] `advance_day()`
- [x] `save_game()`
- [ ] `load_game(slot)`
- [ ] `new_game(payload)` (recebe nome/país/língua/time escolhido)
- [ ] `get_inbox()` (mensagens do dia)
- [ ] `get_next_fixtures()` / `get_recent_results()`
- [ ] Tratamento de erro padrão (UI recebe erro amigável + log detalhado)

## 13. Save/Load (Continuar Jogo)
- [x] Opção A (simples)
- [x] Opção B (UI de slots)
- [x] Lista de saves com metadata (manager, clube, data, last played)
- [x] “Login” (se desejado): seleção de perfil/slot
- [ ] Migração de saves versionados (quando schema mudar)

---

# Phase 6 — Telas CM (Squad, Player, Tactics, Transfers, Finance, Competitions)
## 14. Tela: Elenco (Squad)
- [x] DataGrid: nome, posição, condição, moral, média, gols
- [x] Filtros: titulares/reservas/não-relacionados
- [ ] Ações básicas: definir titular/reserva, ver perfil
- [ ] Ordenação por coluna + busca rápida

## 15. Tela: Perfil do Jogador
- [x] Atributos 0–20 + cores
- [x] Histórico e stats
- [ ] Contrato (salário, duração, cláusulas)
- [ ] Lesões, forma, treino, moral detalhado

## 16. Tela: Táticas
- [x] Visualização de campo 2D
- [x] Instruções: mentalidade, passe, pressão
- [ ] Formação + papéis (GK/DEF/MID/ATT)
- [ ] Set pieces (escanteio/falta) simplificado

## 17. Tela: Dia de Jogo (Match Day)
- [x] Pós-jogo: ratings e estatísticas
- [ ] Ao vivo: eventos por minuto + destaques
- [ ] Escalação confirmada + substituições

## 18. Tela: Competitions
- [x] Backend LeagueTable + command
- [x] UI table
- [x] Calendário (fixtures) + resultados
- [x] Navegação por rodada/temporada

## 19. Tela: Transfers & Market
- [x] Search global
- [x] Offer transfer
- [x] Negociação simples (accept/reject)
- [ ] Shortlist, observação, scout reports
- [ ] Empréstimos + cláusulas básicas

## 20. Tela: Finance
- [x] Salários detalhados (folha, contratos/budget)
- [x] Ticketing / receitas
- [x] Patrocínios
- [x] Regras financeiras (FFP opcional)
- [x] Relatórios mensais

## 21. Tela: Options
- [ ] Idioma (troca + reload)
- [ ] UI scale (4K)
- [ ] Windowed/fullscreen
- [ ] Velocidade de simulação
- [ ] Reset / limpar saves (com confirmação)

---

# Phase 7 — Polimento final e Release
## 22. UI/UX e Fidelidade CM01/02
- [ ] Consistência visual entre telas (margens, fontes, cores, botões)
- [ ] Barra de atalhos (teclas) no rodapé
- [ ] Estados: hover/selected/disabled/unread
- [ ] Performance (scroll listas grandes, tabela, render)

## 23. Empacotamento Windows
- [x] `resources/JSON` incluído no build
- [x] Ajuste do `tauri.conf.json`
- [x] Build final
- [ ] Instalador (MSI/NSIS) e assinatura (opcional)
- [ ] Verificação em PC limpo (sem ambiente dev)

## 24. Backlog (pós 1.0)
- [ ] Editor de dataset
- [ ] Replay de partidas
- [ ] Multithreading
- [ ] Persistência full SQLite
- [ ] Web UI
- [ ] Mod support
- [ ] Multiplayer

---

# Phase 8 — "The Soul of CM" (Os 3 Pilares Modernos)
*Estes são os diferenciais para trazer a profundidade do CM01/02 para a era moderna.*

## 25. UI de Contratos (Negotiation Engine)
- [ ] **Interface de Negociação**: Criar modal/tela de oferta detalhada.
    - [ ] Campos: Salário Mensal/Semanal, Duração (anos), Luvas (Signing Fee), Bônus por Gol/Jogo.
    - [ ] Cláusulas: Rescisão (Release Clause), Aumento anual (YoY Rise).
- [ ] **Feedback do Agente**:
    - [ ] Expor respostas do backend: "O cliente quer mais salário", "Não tem interesse em mudar de país".
    - [ ] Sistema de contra-proposta (agente retorna valores desejados).
- [ ] **Integração com `crates/cm_transfers`**: Conectar essa UI com a lógica complexa já existente no Rust.

## 26. Editor de Táticas Visual (Tactics Board)
- [x] **Drag & Drop**: Tornar o campo tático interativo.
    - [x] Arrastar camisa para mudar posição (ex: puxar volante para virar meia).
    - [ ] Setas de movimento (arrastar com botão direito para indicar corrida 'com' e 'sem' bola).
- [ ] **Validação em Tempo Real**:
    - [ ] Alertar posições inválidas (ex: goleiro no ataque).
    - [ ] Feedback visual de "Familiaridade" com a tática.

## 27. Filtros de Busca Avançada (Scouting Network)
- [ ] **Query Builder UI**: Substituir a busca simples de texto por filtros combinatórios.
    - [ ] UI de "Tags": [Zagueiro] + [Brasileiro] + [Idade < 23] + [Passe > 15].
- [ ] **Network de Olheiros**:
    - [ ] Tela de "Designar Olheiro": País/Região.
    - [ ] Retorno gradual de resultados (não mostrar o banco de dados inteiro instantaneamente, simular o tempo de scout).
