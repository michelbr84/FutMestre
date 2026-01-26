# ToDo — CM Rust Modern UI (Windows Executable)

Roadmap completa para transformar o jogo em um executável Windows com interface moderna 4K, simulação estilo CM01/02, e sem dependências de mídia externa (apenas emojis/code).

## 1. Estrutura do Projeto e Fluxo de Telas (Máquina de Estados)
- [x] **Inicialização do Projeto GUI**
  - [x] Escolher/Configurar framework (Sugestão: Tauri + React/Vanilla JS para UI moderna e fácil estilização).
  - [x] Configurar janela principal: Resolução resizável, suporte a High DPI/4K.
  - [x] Configurar ícone e metadados do executável Windows.
- [x] **Gerenciador de Estados (State Machine)**
  - [x] Implementar sistema de troca de telas (`CurrentScreen` enum/state).
  - [x] Telas necessárias: `LanguageSelect`, `StartMenu`, `NewGame`, `Options`, `LoadGame`, `MainGameHub`.

## 2. Tela 1: Seleção de Idioma (25 Bandeiras)
- [x] **Carregamento de Dados**
  - [x] Ler `/JSON/flags.json` na inicialização.
  - [x] Parsear lista de 25 países com `flag` (emoji) e `language.code`.
- [x] **Renderização (Grid 5x5)**
  - [x] Criar layout Grid 5x5 centralizado.
  - [x] Renderizar botões com o emoji da bandeira (tamanho grande) + nome (opcional).
  - [x] Implementar Hover effects (modernos/glasmorphism).
- [x] **Lógica**
  - [x] `OnClick` -> Salvar `selected_language_code`.
  - [x] Disparar carregamento do idioma (passo 3).
  - [x] Transição para tela `StartMenu`.

## 3. Sistema de Localização (Fallback)
- [x] **Carregador de Strings**
  - [x] Receber `selected_language_code`.
  - [x] Tentar carregar `/JSON/<code>/start.json` (ex: `pt-BR`).
  - [x] Fallback: Tentar `/JSON/<base>/start.json` (ex: `pt`).
  - [x] Default: Usar `en` se falhar.
  - [x] Armazenar strings em memória para uso na UI.

## 4. Tela 2: Menu Inicial (Start Menu)
- [x] **Dados**
  - [x] Usar strings carregadas do `start.json`.
  - [x] Campos: Título do jogo, Labels dos botões (Iniciar, Continuar, Opções, Sair).
- [x] **Interface (UI)**
  - [x] Título grande centralizado ("Futebol Simulador 2026").
  - [x] 4 Botões grandes verticais.
  - [x] Estilo "Premium": Bordas suaves, sombras, tipografia limpa, sem imagens de fundo (apenas cores/gradientes).
- [x] **Ações**
  - [x] **Iniciar Jogo**: Navegar para `NewGameScreen`.
  - [x] **Continuar Jogo**: Executar lógica de Load (Passo 5).
  - [x] **Opções**: Navegar para `OptionsScreen`.
  - [x] **Sair**: Fechar a aplicação.

## 5. Funcionalidade: Continuar Jogo
- [x] **Opção A (Simples)**
  - [x] Verificar existência de save mais recente.
  - [x] Se existir: Carregar e ir para o jogo.
  - [x] Se não: Exibir Toast/Modal "Nenhum save encontrado".
- [x] **Opção B (Interface - Futuro)**
  - [x] Listar slots de save com metadados.

## 6. Tela 3: Novo Jogo (New Game Form)
- [x] **Interface**
  - [x] Formulário de criação de treinador (Nome, Sobrenome).
  - [x] Seleção de Nacionalidade (Dropdown usando dados do `flags.json`).
  - [x] Seleção de Clube Inicial (Dados do `clubs.json` ou engine).
- [x] **Dados**
  - [x] Carregar `new_game.json` do idioma selecionado.
- [x] **Ação**
  - [x] Botão "Confirmar": Inicializar engine rust, criar save inicial, ir para o jogo.

## 7. Tela 4: Opções (Mínimo Viável)
- [x] **Configurações**
  - [x] Selector de Idioma (reutilizar lógica de flags ou dropdown).
  - [x] Modo de Janela (Windowed/Fullscreen).
  - [x] Escala de UI (1x, 1.5x, 2x - importante para 4K).
- [x] Persistência de configurações.

## 8. Integração Backend (Rust)
- [x] Conectar UI (Frontend) com lógica de jogo (`cm_engine`/`cm_core`).
- [x] Comandos Invoke para: Criar Jogo, Carregar Jogo, Salvar, Avançar Dia.

## 9. Polimento Visual (Look & Feel)
- [x] Garantir tipografia consistente (System fonts ou Google Fonts modernas).
- [x] Paleta de cores sóbria e elegante (Dark mode default?).
- [x] Responsividade do layout.
