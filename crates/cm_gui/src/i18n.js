// ─── Internationalization System ──────────────────────────────────────────────

const I18N = {
  current: 'pt-BR',
  currency: 'BRL',

  languages: {
    'en': { name: 'English', flag: '🇬🇧' },
    'pt-BR': { name: 'Portugues', flag: '🇧🇷' },
    'es': { name: 'Espanol', flag: '🇪🇸' },
    'fr': { name: 'Francais', flag: '🇫🇷' },
  },

  currencies: {
    'BRL': { symbol: 'R$', name: 'Real (BRL)', locale: 'pt-BR' },
    'USD': { symbol: '$', name: 'Dollar (USD)', locale: 'en-US' },
    'EUR': { symbol: '€', name: 'Euro (EUR)', locale: 'de-DE' },
    'GBP': { symbol: '£', name: 'Pound (GBP)', locale: 'en-GB' },
  },

  strings: {
    // ─── Main Menu ──────────────────────────────────────────────────────
    'en': {
      // Menu
      game_title: 'FutMestre',
      start_game: 'New Game',
      continue_game: 'Continue',
      options: 'Options',
      exit: 'Exit',

      // New Game
      new_manager: 'New Manager Profile',
      first_name: 'First Name',
      last_name: 'Last Name',
      nationality: 'Nationality',
      select_team: 'Select Team',
      start_career: 'Start Career',
      back: 'Back',
      fill_all: 'Please fill in all fields.',
      loading_clubs: 'Loading clubs...',
      no_clubs: 'Error: no clubs found. Check data in assets/data/.',

      // Load
      load_game: 'Load Game',
      loading: 'Loading...',
      no_saves: 'No saved games found.',
      load_confirm: 'Load this save? Unsaved progress will be lost.',
      load_fail: 'Failed to load game',

      // Options / Settings
      settings: 'Settings',
      language: 'Language',
      currency: 'Currency',
      zoom: 'Zoom',
      zoom_default: '100% (Default)',
      apply: 'Apply',
      close: 'Close',

      // HUD / Tabs
      inbox: 'Inbox',
      squad: 'Squad',
      tactics: 'Tactics',
      fixtures: 'Fixtures',
      transfers: 'Transfers',
      finance: 'Finance',
      standings: 'Standings',
      save_exit: 'Save & Exit',
      continue_btn: 'Continue',
      select_message: 'Select a message',
      no_messages: 'No messages.',

      // Squad
      pos: 'Pos',
      name: 'Name',
      age: 'Age',
      nat: 'Nat',
      ovr: 'Ovr',
      value: 'Value',
      cond: 'Cond',

      // Tactics
      formation: 'Formation',

      // Fixtures
      fixtures_title: 'Fixtures',
      no_fixtures: 'No fixtures scheduled.',

      // Standings
      standings_title: 'Standings',
      club: 'Club',
      played: 'P',
      won: 'W',
      drawn: 'D',
      lost: 'L',
      goals_for: 'GF',
      goals_against: 'GA',
      points: 'Pts',

      // Transfers
      search_placeholder: 'Search player by name...',
      search: 'Search',
      position: 'Position',
      no_players: 'No players found.',
      search_hint: 'Enter a name to search the database.',

      // Finance
      finance_title: 'Club Finances',
      balance: 'Balance',
      transfer_budget: 'Transfer Budget',
      payroll: 'Payroll',
      wage_budget: 'Wage Budget',
      current_wages: 'Current Wages',
      wage_room: 'Available Room',

      // Match
      live: 'LIVE',
      full_time: 'FULL TIME!',
      finish_match: 'Finish Match',
      match_today: 'Match today',
      play_match: 'Play?',

      // Player Profile
      player_profile: 'Player Profile',
      years: 'yrs',
      attributes: 'Attributes',
      contract: 'Contract',
      history: 'History',

      // Save
      game_saved: 'Game saved successfully!',
      save_error: 'Error saving',

      // Settings accessible during game
      division: 'Division',
      rep: 'Rep',
    },

    'pt-BR': {
      game_title: 'FutMestre',
      start_game: 'Novo Jogo',
      continue_game: 'Continuar',
      options: 'Opcoes',
      exit: 'Sair',

      new_manager: 'Novo Treinador',
      first_name: 'Nome',
      last_name: 'Sobrenome',
      nationality: 'Nacionalidade',
      select_team: 'Selecione o Clube',
      start_career: 'Iniciar Carreira',
      back: 'Voltar',
      fill_all: 'Preencha todos os campos.',
      loading_clubs: 'Carregando clubes...',
      no_clubs: 'Erro: nenhum clube encontrado. Verifique assets/data/.',

      load_game: 'Carregar Jogo',
      loading: 'Carregando...',
      no_saves: 'Nenhum jogo salvo encontrado.',
      load_confirm: 'Carregar este save? Progresso nao salvo sera perdido.',
      load_fail: 'Falha ao carregar jogo',

      settings: 'Configuracoes',
      language: 'Idioma',
      currency: 'Moeda',
      zoom: 'Zoom',
      zoom_default: '100% (Padrao)',
      apply: 'Aplicar',
      close: 'Fechar',

      inbox: 'Caixa de Entrada',
      squad: 'Elenco',
      tactics: 'Taticas',
      fixtures: 'Jogos',
      transfers: 'Transferencias',
      finance: 'Financas',
      standings: 'Classificacao',
      save_exit: 'Salvar & Sair',
      continue_btn: 'Continuar',
      select_message: 'Selecione uma mensagem',
      no_messages: 'Nenhuma mensagem.',

      pos: 'Pos',
      name: 'Nome',
      age: 'Idade',
      nat: 'Nac',
      ovr: 'Ovr',
      value: 'Valor',
      cond: 'Cond',

      formation: 'Formacao',

      fixtures_title: 'Jogos',
      no_fixtures: 'Nenhum jogo agendado.',

      standings_title: 'Classificacao',
      club: 'Clube',
      played: 'J',
      won: 'V',
      drawn: 'E',
      lost: 'D',
      goals_for: 'GP',
      goals_against: 'GC',
      points: 'Pts',

      search_placeholder: 'Buscar jogador por nome...',
      search: 'Buscar',
      position: 'Posicao',
      no_players: 'Nenhum jogador encontrado.',
      search_hint: 'Digite um nome para buscar no banco de dados.',

      finance_title: 'Financas do Clube',
      balance: 'Saldo',
      transfer_budget: 'Orcamento de Transferencias',
      payroll: 'Folha Salarial',
      wage_budget: 'Orcamento Salarial',
      current_wages: 'Folha Atual',
      wage_room: 'Espaco Disponivel',

      live: 'AO VIVO',
      full_time: 'FINAL DE JOGO!',
      finish_match: 'Encerrar Partida',
      match_today: 'Jogo hoje',
      play_match: 'Jogar?',

      player_profile: 'Perfil do Jogador',
      years: 'anos',
      attributes: 'Atributos',
      contract: 'Contrato',
      history: 'Historico',

      game_saved: 'Jogo salvo com sucesso!',
      save_error: 'Erro ao salvar',

      division: 'Divisao',
      rep: 'Rep',
    },

    'es': {
      game_title: 'FutMestre',
      start_game: 'Nuevo Juego',
      continue_game: 'Continuar',
      options: 'Opciones',
      exit: 'Salir',

      new_manager: 'Nuevo Entrenador',
      first_name: 'Nombre',
      last_name: 'Apellido',
      nationality: 'Nacionalidad',
      select_team: 'Seleccionar Equipo',
      start_career: 'Iniciar Carrera',
      back: 'Volver',
      fill_all: 'Complete todos los campos.',
      loading_clubs: 'Cargando equipos...',
      no_clubs: 'Error: ningun equipo encontrado.',

      load_game: 'Cargar Juego',
      loading: 'Cargando...',
      no_saves: 'No se encontraron partidas guardadas.',
      load_confirm: 'Cargar esta partida? El progreso no guardado se perdera.',
      load_fail: 'Error al cargar el juego',

      settings: 'Configuracion',
      language: 'Idioma',
      currency: 'Moneda',
      zoom: 'Zoom',
      zoom_default: '100% (Predeterminado)',
      apply: 'Aplicar',
      close: 'Cerrar',

      inbox: 'Bandeja',
      squad: 'Plantilla',
      tactics: 'Tacticas',
      fixtures: 'Partidos',
      transfers: 'Fichajes',
      finance: 'Finanzas',
      standings: 'Clasificacion',
      save_exit: 'Guardar & Salir',
      continue_btn: 'Continuar',
      select_message: 'Seleccione un mensaje',
      no_messages: 'Sin mensajes.',

      pos: 'Pos',
      name: 'Nombre',
      age: 'Edad',
      nat: 'Nac',
      ovr: 'Ovr',
      value: 'Valor',
      cond: 'Cond',

      formation: 'Formacion',

      fixtures_title: 'Partidos',
      no_fixtures: 'No hay partidos programados.',

      standings_title: 'Clasificacion',
      club: 'Club',
      played: 'PJ',
      won: 'PG',
      drawn: 'PE',
      lost: 'PP',
      goals_for: 'GF',
      goals_against: 'GC',
      points: 'Pts',

      search_placeholder: 'Buscar jugador por nombre...',
      search: 'Buscar',
      position: 'Posicion',
      no_players: 'Ningun jugador encontrado.',
      search_hint: 'Ingrese un nombre para buscar en la base de datos.',

      finance_title: 'Finanzas del Club',
      balance: 'Saldo',
      transfer_budget: 'Presupuesto de Fichajes',
      payroll: 'Nomina',
      wage_budget: 'Presupuesto Salarial',
      current_wages: 'Salarios Actuales',
      wage_room: 'Disponible',

      live: 'EN VIVO',
      full_time: 'FINAL!',
      finish_match: 'Finalizar Partido',
      match_today: 'Partido hoy',
      play_match: 'Jugar?',

      player_profile: 'Perfil del Jugador',
      years: 'anos',
      attributes: 'Atributos',
      contract: 'Contrato',
      history: 'Historial',

      game_saved: 'Juego guardado con exito!',
      save_error: 'Error al guardar',

      division: 'Division',
      rep: 'Rep',
    },

    'fr': {
      game_title: 'FutMestre',
      start_game: 'Nouvelle Partie',
      continue_game: 'Continuer',
      options: 'Options',
      exit: 'Quitter',

      new_manager: 'Nouveau Manager',
      first_name: 'Prenom',
      last_name: 'Nom',
      nationality: 'Nationalite',
      select_team: 'Choisir Equipe',
      start_career: 'Demarrer Carriere',
      back: 'Retour',
      fill_all: 'Veuillez remplir tous les champs.',
      loading_clubs: 'Chargement des clubs...',
      no_clubs: 'Erreur: aucun club trouve.',

      load_game: 'Charger Partie',
      loading: 'Chargement...',
      no_saves: 'Aucune sauvegarde trouvee.',
      load_confirm: 'Charger cette sauvegarde? La progression non sauvegardee sera perdue.',
      load_fail: 'Echec du chargement',

      settings: 'Parametres',
      language: 'Langue',
      currency: 'Devise',
      zoom: 'Zoom',
      zoom_default: '100% (Par defaut)',
      apply: 'Appliquer',
      close: 'Fermer',

      inbox: 'Messages',
      squad: 'Effectif',
      tactics: 'Tactiques',
      fixtures: 'Matchs',
      transfers: 'Transferts',
      finance: 'Finances',
      standings: 'Classement',
      save_exit: 'Sauvegarder & Quitter',
      continue_btn: 'Continuer',
      select_message: 'Selectionnez un message',
      no_messages: 'Aucun message.',

      pos: 'Pos',
      name: 'Nom',
      age: 'Age',
      nat: 'Nat',
      ovr: 'Ovr',
      value: 'Valeur',
      cond: 'Cond',

      formation: 'Formation',

      fixtures_title: 'Matchs',
      no_fixtures: 'Aucun match programme.',

      standings_title: 'Classement',
      club: 'Club',
      played: 'MJ',
      won: 'V',
      drawn: 'N',
      lost: 'D',
      goals_for: 'BP',
      goals_against: 'BC',
      points: 'Pts',

      search_placeholder: 'Rechercher un joueur par nom...',
      search: 'Rechercher',
      position: 'Poste',
      no_players: 'Aucun joueur trouve.',
      search_hint: 'Entrez un nom pour rechercher dans la base de donnees.',

      finance_title: 'Finances du Club',
      balance: 'Solde',
      transfer_budget: 'Budget Transferts',
      payroll: 'Masse Salariale',
      wage_budget: 'Budget Salaires',
      current_wages: 'Salaires Actuels',
      wage_room: 'Disponible',

      live: 'EN DIRECT',
      full_time: 'COUP DE SIFFLET FINAL!',
      finish_match: 'Terminer le Match',
      match_today: 'Match aujourd\'hui',
      play_match: 'Jouer?',

      player_profile: 'Profil du Joueur',
      years: 'ans',
      attributes: 'Attributs',
      contract: 'Contrat',
      history: 'Historique',

      game_saved: 'Partie sauvegardee avec succes!',
      save_error: 'Erreur de sauvegarde',

      division: 'Division',
      rep: 'Rep',
    },
  },

  // Get a translated string
  t(key) {
    const lang = I18N.strings[I18N.current] || I18N.strings['en'];
    return lang[key] || I18N.strings['en'][key] || key;
  },

  // Set language and update all UI
  setLanguage(code) {
    if (I18N.strings[code]) {
      I18N.current = code;
      I18N.applyToDOM();
    }
  },

  setCurrency(code) {
    if (I18N.currencies[code]) {
      I18N.currency = code;
    }
  },

  // Apply translations to all elements with data-i18n attribute
  applyToDOM() {
    document.querySelectorAll('[data-i18n]').forEach(el => {
      const key = el.getAttribute('data-i18n');
      const text = I18N.t(key);
      if (el.tagName === 'INPUT' && el.type !== 'hidden') {
        el.placeholder = text;
      } else {
        el.textContent = text;
      }
    });
    // Update title
    document.title = I18N.t('game_title');
  },

  // Format a money value with current currency
  formatMoney(valueStr) {
    if (!valueStr) return valueStr;
    const cur = I18N.currencies[I18N.currency];
    // Replace £ with current currency symbol
    return valueStr.replace(/£/g, cur.symbol);
  }
};

export default I18N;
