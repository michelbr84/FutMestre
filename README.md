# FutMestre

Jogo de gerenciamento de futebol inspirado nos classicos **Championship Manager 01/02** e **Elifoot 98**. Construido em Rust com interface TUI (terminal) e desktop via Tauri.

Gerencie seu clube, escale o time, defina taticas, contrate jogadores e leve seu time da quarta divisao ao topo do campeonato brasileiro.

## Inicio Rapido

```bash
# Compilar o projeto
cargo build --workspace

# Interface de terminal (recomendado)
cargo run -p cm_tui

# Simular uma partida via CLI
cargo run -p cm_cli -- simulate-match --home FLA --away PAL --seed 42

# Servidor HTTP
cargo run -p cm_server
```

## Funcionalidades

### Jogabilidade
- **80 clubes brasileiros** em 4 divisoes (Serie A, B, C, D)
- **1.760 jogadores** com nomes brasileiros reais e atributos proporcionais
- **Copa do Brasil** com times da Serie A e B
- **Motor de partida** probabilistico minuto-a-minuto com cartoes, lesoes, faltas e bola parada
- **Promocao e rebaixamento** entre divisoes (top 3 sobem, bottom 3 descem)

### Gestao
- **8 abas**: Elenco, Taticas, Treino, Jogos, Classificacao, Financas, Transferencias, Noticias
- **Taticas editaveis**: formacao (10 opcoes), mentalidade, ritmo, pressao, linha defensiva
- **Treinamento**: fisico, tecnico, tatico, recuperacao com evolucao de atributos
- **Financas**: saldo, orcamento de transferencias, folha salarial, TV, merchandising, FFP
- **Transferencias**: janela de mercado, avaliacao de jogadores, negociacao

### Motor
- **40+ atributos** por jogador (tecnicos, mentais, fisicos, goleiro)
- **IA completa**: escalacao, transferencias, taticas, diretoria, imprensa, scouting
- **Desenvolvimento**: jogadores jovens evoluem, veteranos declinam
- **Save/Load**: salvamento comprimido com verificacao SHA256

## Controles (TUI)

| Tecla | Acao |
|-------|------|
| `Tab` / `Shift+Tab` | Trocar aba |
| `1-8` | Ir direto para aba |
| `Espaco` / `N` | Avancar 1 dia |
| `A` | Avancar 1 semana |
| `Setas` | Navegar / Alterar valores |
| `Enter` | Confirmar |
| `Esc` | Voltar / Menu |

## Arquitetura

Workspace Rust com 15 crates seguindo Domain-Driven Design:

```
crates/
cm_utils        Utilitarios (RNG, hashing, tempo)
cm_telemetry    Logging e tracing
cm_core         Modelos de dominio (Jogador, Clube, Competicao)
cm_data         Importador JSON + SQLite
cm_match        Motor de partida (simulacao minuto-a-minuto)
cm_ai           IA (escalacao, transferencias, tatica, diretoria)
cm_finance      Financas (salarios, patrocinio, FFP)
cm_transfers    Mercado (avaliacao, negociacao, contratos)
cm_save         Save/Load (gzip + SHA256)
cm_engine       Game loop com 13+ sistemas
cm_cli          Comandos CLI
cm_tui          Interface terminal (Ratatui)
cm_api          API REST (DTOs)
cm_server       Servidor HTTP (Axum)
cm_gui          Desktop GUI (Tauri)
```

## Dados do Jogo

- `assets/data/clubs.json` — 80 clubes brasileiros
- `assets/data/competitions.json` — 4 divisoes + Copa do Brasil
- `assets/data/nations.json` — 16 nacoes
- `assets/names/` — Nomes brasileiros para geracao de jogadores

## Desenvolvimento

```bash
cargo build --workspace              # Compilar
cargo test --workspace               # Testes (508+)
cargo fmt --all                      # Formatar
cargo clippy --workspace -- -D warnings  # Lint
```

## Pre-requisitos

- [Rust 1.75+](https://www.rust-lang.org/tools/install)
- [Node.js 18+](https://nodejs.org/) (apenas para GUI Tauri)

## Roadmap

Consulte [roadmap.md](roadmap.md) para o plano completo com 13 fases.

## Licenca

MIT License

---

*Desenvolvido com dedicacao para os fas de jogos de gerenciamento de futebol.*
