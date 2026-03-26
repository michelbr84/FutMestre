# Checklist de Release - FutMestre

## 1. Preparacao da Versao

- [ ] Definir numero da versao seguindo SemVer (MAJOR.MINOR.PATCH)
- [ ] Atualizar `version` no `Cargo.toml` do workspace
- [ ] Atualizar `package.version` no `tauri.conf.json` (GUI)
- [ ] Verificar que todas as dependencias estao com versoes corretas

## 2. Atualizacao do Changelog

- [ ] Criar entrada no `CHANGELOG.md` com a nova versao
- [ ] Listar todas as funcionalidades novas (Added)
- [ ] Listar todas as correcoes de bugs (Fixed)
- [ ] Listar mudancas que quebram compatibilidade (Changed/Breaking)
- [ ] Listar items removidos (Removed)
- [ ] Incluir data de release

## 3. Qualidade de Codigo

### Formatacao
```bash
cargo fmt --all
cargo fmt --all -- --check  # verificar sem alterar
```

### Linting
```bash
cargo clippy --workspace --all-targets -- -D warnings
```

### Testes
```bash
cargo test --workspace
```

### Verificacao completa (CI)
```bash
make ci  # fmt-check + clippy + test
```

## 4. Build de Release

### TUI (Terminal)
```bash
cargo build --release -p cm_tui
```

### CLI
```bash
cargo build --release -p cm_cli
```

### GUI (Tauri)
```bash
cd crates/cm_gui
npm run tauri build
```

### Verificar tamanho dos binarios
- [ ] TUI: < 20 MB
- [ ] CLI: < 15 MB
- [ ] GUI: < 50 MB (instalador)

## 5. Smoke Test

### CLI
- [ ] `cargo run -p cm_cli -- simulate-match --home LIV --away ARS --seed 42`
- [ ] `cargo run -p cm_cli -- new-game --club LIV --manager "Test"`
- [ ] Verificar que o resultado da partida e deterministico com seed fixa

### TUI
- [ ] Iniciar novo jogo e verificar menu principal
- [ ] Avancar 7 dias sem erros
- [ ] Jogar uma partida completa
- [ ] Salvar e carregar jogo

### GUI
- [ ] Iniciar aplicacao Tauri
- [ ] Criar novo jogo com cada modo (Sandbox, Carreira)
- [ ] Verificar elenco e atributos dos jogadores
- [ ] Modificar taticas
- [ ] Avancar dias e verificar resultados
- [ ] Jogar partida e verificar eventos
- [ ] Verificar tabela da liga
- [ ] Testar transferencias (compra)
- [ ] Verificar financas
- [ ] Salvar e carregar jogo
- [ ] Verificar caixa de entrada
- [ ] Testar scouting de jogador
- [ ] Testar sistema de reservas

### Dados
- [ ] Verificar que `assets/data/` contem todos os JSON necessarios
- [ ] Validar integridade dos dados com `cm_data` importer

## 6. Documentacao

- [ ] README.md atualizado
- [ ] `docs/guia-do-jogador.md` revisado
- [ ] `docs/schema.md` atualizado se houve mudancas no schema
- [ ] `roadmap.md` atualizado com progresso

## 7. Controle de Versao

- [ ] Criar branch de release: `release/vX.Y.Z`
- [ ] Commit final com mensagem: `Release vX.Y.Z`
- [ ] Criar tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
- [ ] Push: `git push origin release/vX.Y.Z --tags`
- [ ] Criar Pull Request para `main`
- [ ] Merge apos aprovacao

## 8. Distribuicao

### GitHub Release
- [ ] Criar release no GitHub a partir da tag
- [ ] Anexar binarios compilados:
  - Windows: `.exe` (TUI/CLI) e `.msi` (GUI)
  - Linux: binarios e `.AppImage` (GUI)
  - macOS: binarios e `.dmg` (GUI)
- [ ] Incluir changelog na descricao

### Pacotes
- [ ] Verificar que o instalador Windows funciona
- [ ] Verificar que o AppImage Linux funciona
- [ ] Verificar que o DMG macOS funciona (se aplicavel)

## 9. Pos-Release

- [ ] Verificar que a release no GitHub esta correta
- [ ] Anunciar a versao (se aplicavel)
- [ ] Criar branch para proxima versao de desenvolvimento
- [ ] Atualizar versao no Cargo.toml para proxima dev (`X.Y.Z-dev`)
