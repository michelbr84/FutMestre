# ToDo — CM Rust Modern GUI (Windows Executable)

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

## 5. Tela 3: Novo Jogo (New Game Form)
- [x] **Carregamento de Times**
  - [x] Ler `times.json` do idioma selecionado.
  - [x] Sortear 6 times aleatórios (sem repetição).
  - [x] Congelar seleção (não mudar ao re-renderizar).
- [x] **Interface**
  - [x] Formulário: Nome, Sobrenome, Nacionalidade (Dropdown), Língua.
  - [x] **Grid de Seleção de Time (3x2)**: 6 Botões grandes com Nome + Cores do time.
- [x] **Lógica de Seleção**
  - [x] Highlight no time clicado.
  - [x] Validar se todos os campos estão preenchidos antes de habilitar "Confirmar".
- [x] **Ação**
  - [x] Botão "Confirmar": Disparar geração de mundo (Passo 6).

## 6. Geração de Mundo (Engine)
- [x] **Times**
  - [x] Carregar os 30 times do `times.json`.
- [x] **Jogadores**
  - [x] Gerar ~510 jogadores (17 por time).
  - [x] Usar `nomes.json` (nomes + sobrenomes) para gerar nomes aleatórios.
- [x] **Técnicos**
  - [x] Carregar `tecnicos.json`.
  - [x] Assignar User Manager ao time selecionado.
  - [x] Assignar bots aos outros 29 times.

## 7. Tela 4: Notícias (News Screen)
- [x] **Template**
  - [x] Carregar `atual.json` como template de estrutura.
- [x] **Adaptação**
  - [x] Substituir dados estáticos pelos dados da carreira gerada (Nome do time, Data atual).
  - [x] Inserir mensagem de boas vindas personalizada ("Bem-vindo ao [Time Selecionado]").
- [x] **Renderização**
  - [x] Exibir Header com Data/Hora.
  - [x] Exibir Lista de Mensagens (Inbox).
  - [x] Exibir Detalhe da Mensagem selecionada.

## 8. Funcionalidade: Continuar Jogo
- [x] **Opção A (Simples)**
  - [x] Verificar existência de save mais recente.
  - [x] Se existir: Carregar e ir para o jogo.
  - [x] Se não: Exibir Toast/Modal "Nenhum save encontrado".

## 9. Integração & Build
- [x] Padronizar pasta `resources/JSON` para distribuição.
- [x] Ajustar `tauri.conf.json` para incluir assets no build.
- [x] Verificar build final (`npm run tauri build`).

