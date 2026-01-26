# ToDo — CM Rust Modern GUI (Windows Executable)

Roadmap completa para transformar o jogo em um executável Windows com interface moderna 4K, simulação estilo CM01/02, e sem dependências de mídia externa (apenas emojis/code).

## 1. Estrutura do Projeto e Fluxo de Telas (Máquina de Estados)
- [x] **Inicialização do Projeto GUI**
- [x] **Gerenciador de Estados (State Machine)**

## 2. Tela 1: Seleção de Idioma (25 Bandeiras)
- [x] **Carregamento de Dados**
- [x] **Renderização (Grid 5x5)**
- [x] **Lógica**

## 3. Sistema de Localização (Fallback)
- [x] **Carregador de Strings**

## 4. Tela 2: Menu Inicial (Start Menu)
- [x] **Dados**
- [x] **Interface (UI)**
- [x] **Ações**

## 5. Tela 3: Novo Jogo (New Game Form)
- [x] **Carregamento de Times**
- [x] **Interface**
- [x] **Lógica de Seleção**
- [x] **Ação**

## 6. Geração de Mundo (Engine)
- [x] **Times**
- [x] **Jogadores**
- [x] **Técnicos**

## 7. Tela 4: Notícias (News Screen)
- [x] **Template**
- [x] **Adaptação**
- [x] **Renderização (CM01/02 Style)**

## 8. Funcionalidade: Continuar Jogo
- [x] **Opção A (Simples)**
- [x] **Opção B (Interface - Futuro)**

## 9. Integração & Build
- [x] Padronizar pasta `resources/JSON` para distribuição.
- [x] Ajustar `tauri.conf.json` para incluir assets no build.
- [x] Verificar build final (`npm run tauri build`).

---
# Phase 2: Core Gameplay & Rust Integration

## 10. Conexão Backend Rust (Expor Comandos)
- [x] Criar Structs de exibição (`DisplayPlayer`, `DisplaySquad`) em `src-tauri`.
- [x] Implementar comando `get_squad(team_id)`: Retornar lista real de jogadores.
- [x] Implementar comando `advance_day()`: Processar simulação de dia.
- [x] Implementar comando `save_game()`: Gravar estado atual.

## 11. Tela 5: Elenco (Squad Screen)
- [x] **DataGrid**: Tabela com Nome, Posição, Condição, Moral, Média, Gols.
- [x] **Filtros**: Titulares, Reservas, Não-relacionados.
- [x] **Interação**: Drag & Drop ou Context Menu para definir time.

## 12. Tela 6: Perfil do Jogador (Player Profile)
- [x] **Atributos**: Grade de habilidades (0-20) com color code.
- [x] **Histórico**: Estatísticas da temporada e clubes anteriores.
- [x] **Ações**: Oferecer contrato / Listar para transferência.

# Phase 3: Tactics & Match Engine

## 13. Tela 7: Tática
- [x] **Visualização**: Campo 2D com posições.
- [x] **Instruções**: Mentalidade, Estilo de Passe, Pressão.

## 14. Tela 8: Dia de Jogo (Match Day)
- [x] **Pós-jogo**: Ratings e Estatísticas.

# Phase 4: Season Structure & Transfers

## 15. Competitions (League Tables)
- [x] **Backend**: Create `LeagueTable` struct (Points, GD, Played).
- [x] **Command**: `get_league_table(competition_id)`.
- [x] **UI**: Render the table view.

## 16. Transfers & Market
- [x] **Search**: Implement `search_players(query)` for global database.
- [x] **Bidding**: Implement `offer_transfer(player_id, amount)`.
- [x] **Negotiation**: Simple AI response (Accept/Reject).

## 17. Season Management
- [x] **Fixtures**: Generate a schedule for the league (Home/Away).
- [x] **Progression**: Advance week-by-week playing scheduled matches.

# Phase 5: Refinement & Bug Fixes (Urgent)

## 18. UI/UX Fixes
- [x] **Layout**: Fix 'Continue' button z-index/clickability (Button was behind header).
- [x] **Navigation**: Enable all sidebar tabs (Squad, Tactics, Transfers, Finance, Competitions).
- [x] **Structure**: Consolidate views inside `.main-content` for correct layout.
