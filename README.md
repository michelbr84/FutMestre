# FutMestre

Jogo de gerenciamento de futebol inspirado nos classicos **Championship Manager 01/02** e **Elifoot 98**. Construido em Rust com interface desktop via Tauri e terminal via Ratatui.

Gerencie seu clube, escale o time, defina taticas, contrate jogadores e leve seu time da quarta divisao ao topo do campeonato brasileiro.

## Inicio Rapido

```bash
# Compilar o projeto
cargo build --workspace

# Interface desktop (GUI - recomendado)
cd crates/cm_gui && npx tauri build --debug

# Interface de terminal (TUI)
cargo run -p cm_tui

# Simular uma partida via CLI
cargo run -p cm_cli -- simulate-match --home FLA --away PAL --seed 42
```

## Screenshots

A GUI desktop usa glassmorphism com visual estilo Championship Manager.

## Funcionalidades

### Jogabilidade
- **80 clubes brasileiros** em 4 divisoes (Serie A, B, C, D)
- **1.760+ jogadores** com nomes brasileiros reais e atributos proporcionais
- **Copa do Brasil** com times da Serie A e B
- **Motor de partida** probabilistico minuto-a-minuto com cartoes, lesoes, faltas, bola parada e penaltis
- **Promocao e rebaixamento** entre divisoes (top 4 sobem, bottom 4 descem)
- **Calendario FIFA** com pausas internacionais
- **Partida ao vivo** com ticker de eventos, animacoes de gol e estatisticas

### Gestao
- **9 abas**: Elenco, Taticas, Treino, Jogos, Classificacao, Financas, Transferencias, Noticias, Academia
- **Taticas editaveis**: 10 formacoes, mentalidade, ritmo, pressao, linha defensiva, largura
- **Treinamento**: fisico, tecnico, tatico, recuperacao com evolucao de atributos
- **Financas**: saldo, orcamento de transferencias, folha salarial, TV, merchandising, FFP, emprestimos bancarios
- **Transferencias**: janela de mercado, avaliacao, negociacao com contra-propostas, leilao estilo Elifoot
- **Academia**: geracao anual de 2-5 jovens por clube baseado em reputacao
- **Scouting**: observar jogadores, clubes, nacoes e proximo adversario

### Motor
- **40+ atributos** por jogador (tecnicos, mentais, fisicos, goleiro)
- **IA completa**: escalacao, transferencias, taticas adaptativas, diretoria, imprensa, scouting
- **6 personalidades de IA** para gerentes
- **Desenvolvimento**: jogadores jovens evoluem, veteranos declinam
- **Ratings de desempenho** (1-10) com craque da partida
- **Historico de temporadas** por clube e jogador
- **Artilharia** por competicao

### Interface
- **GUI Desktop** via Tauri com glassmorphism (estilo CM 01/02)
- **TUI Terminal** via Ratatui com 9 abas navegaveis
- **i18n**: 4 idiomas (PT-BR, EN, ES, FR) e 4 moedas (BRL, USD, EUR, GBP)
- **Save/Load**: salvamento comprimido com SHA256, multiplos slots

### Modos de Jogo
- **Sandbox**: escolha qualquer clube, sem restricoes
- **Carreira Serie D**: comece na quarta divisao, conquiste a Serie A
- **Desafio**: restricoes de orcamento e elenco

## Controles (TUI)

| Tecla | Acao |
|-------|------|
| `Tab` / `Shift+Tab` | Trocar aba |
| `1-9` | Ir direto para aba |
| `Espaco` / `N` | Avancar 1 dia |
| `A` | Avancar 1 semana |
| `Setas` | Navegar / Alterar valores |
| `Enter` | Confirmar |
| `Ctrl+S` | Salvar jogo |
| `Esc` | Voltar / Menu |

## Arquitetura

Workspace Rust com 15 crates seguindo Domain-Driven Design:

```
crates/
cm_utils        Utilitarios (RNG, hashing, tempo)
cm_telemetry    Logging e tracing
cm_core         Modelos de dominio (Jogador, Clube, Competicao, 40+ atributos)
cm_data         Importador JSON + SQLite
cm_match        Motor de partida (simulacao minuto-a-minuto, ratings)
cm_ai           IA (escalacao, transferencias, tatica, diretoria, scouting)
cm_finance      Financas (salarios, patrocinio, FFP, emprestimos)
cm_transfers    Mercado (avaliacao, negociacao, leilao, contratos)
cm_save         Save/Load (gzip + SHA256, exportacao)
cm_engine       Game loop com 13+ sistemas orquestrados
cm_cli          Comandos CLI (Clap)
cm_tui          Interface terminal (Ratatui)
cm_api          API REST (DTOs)
cm_server       Servidor HTTP (Axum)
cm_gui          Desktop GUI (Tauri + Glassmorphism)
```

## Dados do Jogo

- `assets/data/clubs.json` — 80 clubes brasileiros (Serie A ate D)
- `assets/data/competitions.json` — 4 divisoes + Copa do Brasil
- `assets/data/nations.json` — 16 nacoes
- `assets/data/stadiums.json` — 80 estadios brasileiros reais
- `assets/data/referees.json` — 20 arbitros com perfis
- `assets/data/staff.json` — Staff tecnico
- `assets/data/tactics_presets.json` — Presets de taticas
- `assets/data/calendar.json` — Calendario da temporada

## Desenvolvimento

```bash
cargo build --workspace                                    # Compilar
cargo test --workspace                                     # Testes (558+)
cargo fmt --all                                            # Formatar
cargo clippy --workspace --all-targets -- -D warnings      # Lint

# GUI Desktop
cd crates/cm_gui && npx tauri dev                          # Dev mode
cd crates/cm_gui && npx tauri build --debug                # Build
```

## Pre-requisitos

- [Rust 1.75+](https://www.rust-lang.org/tools/install)
- [Node.js 18+](https://nodejs.org/) (apenas para GUI Tauri)
- Visual Studio Build Tools 2022 (Windows)

## Roadmap

Consulte [roadmap.md](roadmap.md) para o plano completo com 15 fases.
Consulte [CHANGELOG.md](CHANGELOG.md) para historico de versoes.

## Documentacao

- [Guia do Jogador](docs/guia-do-jogador.md)
- [Schema de Dados](docs/schema.md)
- [Checklist de Release](docs/release-checklist.md)

## Licenca

MIT License

---

*Desenvolvido com dedicacao para os fas de jogos de gerenciamento de futebol.*
