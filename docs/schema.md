# Schema e Convencoes de ID

## Convencoes de ID

Todas as entidades do jogo usam IDs tipados (newtype wrappers sobre `String`) definidos em `cm_core::ids`.

### ClubId

- Formato: codigo de 3 letras maiusculas (ex: `LIV`, `ARS`, `MAN`)
- Baseado na abreviacao oficial do clube
- Unico por clube

### PlayerId

- Formato: prefixo + numero sequencial (ex: `P001`, `PA003`)
- O prefixo pode indicar a origem dos dados
- Unico globalmente

### StaffId

- Formato: `S` + numero sequencial (ex: `S001`, `S042`)
- Unico globalmente

### NationId

- Formato: codigo de 3 letras ISO 3166-1 alpha-3 (ex: `ENG`, `BRA`, `ARG`)
- Segue o padrao FIFA quando possivel

### CompetitionId

- Formato: abreviacao da competicao (ex: `PL`, `SA`, `SB`)
- Unico por competicao

### StadiumId

- Formato: abreviacao do estadio (ex: `ANF`, `MAR`)
- Unico por estadio

### MatchId

- Formato: gerado automaticamente pelo sistema de fixtures
- Composto por competicao + rodada + times

### RefereeId

- Formato: `R` + numero sequencial (ex: `R001`)

### ContractId / TransferId

- Gerados internamente pelo sistema de contratos/transferencias

## Versoes do Schema JSON

Os dados do jogo sao armazenados em arquivos JSON em `assets/data/`.

### v1.0 (atual)

Arquivos:
- `clubs.json` - Clubes com elencos
- `competitions.json` - Competicoes e divisoes
- `nations.json` - Nacoes
- `stadiums.json` - Estadios
- `calendar.json` - Calendario com datas FIFA
- `referees.json` - Arbitros
- `staff.json` - Comissao tecnica
- `tactics_presets.json` - Presets de taticas

### Campos obrigatorios vs opcionais

Campos marcados com `#[serde(default)]` sao opcionais e terao valor padrao se ausentes:
- `Club::history` - historico vazio
- `Club::reserve_ids` - lista vazia
- `Competition::division_level` - None
- `Player::secondary_positions` - lista vazia

## Procedimento de Migracao

### Ao adicionar um novo campo

1. Adicionar o campo na struct com `#[serde(default)]`
2. Atualizar o construtor `new()` com valor padrao
3. Atualizar os testes existentes
4. Saves antigos serao compativeis automaticamente (serde preenche o default)

### Ao renomear um campo

1. Usar `#[serde(alias = "nome_antigo")]` para manter compatibilidade
2. Atualizar os arquivos JSON em `assets/data/`
3. Atualizar o importer em `cm_data`

### Ao remover um campo

1. Marcar com `#[deprecated]` por uma versao
2. Remover na versao seguinte
3. Saves com o campo antigo serao ignorados pelo serde

## Formato de Save

O sistema de save (`cm_save`) usa:
- Serializacao: bincode
- Compressao: zstd
- Integridade: SHA-256
- Extensao: `.cmsave`

Estrutura do save:
```
[Header: 8 bytes magic + versao]
[SHA-256 hash: 32 bytes]
[Payload comprimido: SaveSnapshot serializado]
```
