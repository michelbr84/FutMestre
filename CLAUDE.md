# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Visao Geral

**FutMestre** e um jogo de gerenciamento de futebol em Rust, inspirado em Championship Manager 01/02 e Elifoot 98. Workspace com 15 crates seguindo Domain-Driven Design, interface desktop via Tauri.

## Comandos Essenciais

```bash
cargo build --workspace                                    # Compilar
cargo test --workspace                                     # Todos os testes
cargo test -p cm_match                                     # Testes de um crate
cargo fmt --all                                            # Formatar
cargo clippy --workspace --all-targets -- -D warnings      # Lint
make ci                                                    # fmt-check + clippy + test
make all                                                   # fmt + clippy + test
cargo run -p cm_cli -- simulate-match --home LIV --away ARS --seed 42
cargo run -p cm_cli -- new-game --club LIV --manager "Nome"
cargo run -p cm_tui                                        # Terminal UI
cargo run -p cm_server                                     # Servidor HTTP :3000
```

## Arquitetura dos Crates

Ordem de dependencia (de baixo para cima):

1. **cm_utils** — RNG, hashing, tempo, filesystem
2. **cm_telemetry** — logging e tracing
3. **cm_core** — modelos de dominio (World, Player, Club, Competition, 40+ atributos de jogador)
4. **cm_data** — importador JSON, persistencia SQLite, repositorios
5. **cm_match** — motor de partida probabilistico minuto-a-minuto
6. **cm_ai** — IA de escalacao, transferencias, tatica, diretoria, imprensa (5.2k LOC, maior crate)
7. **cm_finance** — salarios, bilheteria, patrocinio, FFP
8. **cm_transfers** — mercado, avaliacao, negociacao, emprestimos, janelas
9. **cm_save** — salvamento comprimido com SHA256
10. **cm_engine** — game loop diario com 13+ sistemas orquestrados
11. **cm_cli** — comandos CLI (Clap)
12. **cm_tui** — interface terminal (Ratatui)
13. **cm_api** + **cm_server** — REST API (Axum)
14. **cm_gui** — desktop GUI (Tauri + frontend web com Glassmorphism)

## Dados do Jogo

Arquivos JSON em `assets/data/`: clubs, competitions, nations, stadiums, calendar, referees, staff, tactics_presets. Atualmente com dados de referencia (clubes ingleses). Dados brasileiros planejados.

## Estado Atual

**Completo**: modelos de dominio, IA (5 sistemas), financas, transferencias, save/load, CLI, estrutura Tauri
**Parcial**: motor de partida (funciona mas sem cartoes/lesoes/substituicoes durante jogo), TUI, API REST
**Planejado**: 4 divisoes brasileiras, copa, treinamento com evolucao, categorias de base

## Convencoes

- Projeto inteiro em **portugues** (documentacao, comentarios, UI)
- GitHub: **michelbr84**
- Roadmap completo em `roadmap.md` com 13 fases
- Apos cada edicao: rodar `cargo fmt`, `cargo clippy`, e testes relevantes
- Commits atomicos e frequentes

## ClaudeMaxPower

- Agent teams habilitado (`CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`)
- Auto Dream habilitado (`autoDreamEnabled: true`)
- Hook lifecycle: SessionStart (recuperar .estado.md) → PreToolUse (seguranca) → PostToolUse (auto-testes) → Stop (persistir estado)
