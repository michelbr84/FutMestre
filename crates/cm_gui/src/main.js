import I18N from './i18n.js';
import { getRandomMessage } from './messages_db.js';

const { invoke } = window.__TAURI__.core;

// ─── Formation Position Maps ──────────────────────────────────────────────
const FORMATION_POSITIONS = {
  '4-4-2': [
    { top: '87%', left: '50%', label: 'GK' },
    { top: '68%', left: '15%', label: 'LE' },
    { top: '68%', left: '38%', label: 'ZC' },
    { top: '68%', left: '62%', label: 'ZC' },
    { top: '68%', left: '85%', label: 'LD' },
    { top: '42%', left: '15%', label: 'ME' },
    { top: '42%', left: '38%', label: 'MC' },
    { top: '42%', left: '62%', label: 'MC' },
    { top: '42%', left: '85%', label: 'MD' },
    { top: '18%', left: '35%', label: 'CA' },
    { top: '18%', left: '65%', label: 'CA' },
  ],
  '4-3-3': [
    { top: '87%', left: '50%', label: 'GK' },
    { top: '68%', left: '15%', label: 'LE' },
    { top: '68%', left: '38%', label: 'ZC' },
    { top: '68%', left: '62%', label: 'ZC' },
    { top: '68%', left: '85%', label: 'LD' },
    { top: '42%', left: '25%', label: 'MC' },
    { top: '42%', left: '50%', label: 'MC' },
    { top: '42%', left: '75%', label: 'MC' },
    { top: '18%', left: '20%', label: 'PE' },
    { top: '14%', left: '50%', label: 'CA' },
    { top: '18%', left: '80%', label: 'PD' },
  ],
  '3-5-2': [
    { top: '87%', left: '50%', label: 'GK' },
    { top: '68%', left: '25%', label: 'ZC' },
    { top: '68%', left: '50%', label: 'ZC' },
    { top: '68%', left: '75%', label: 'ZC' },
    { top: '42%', left: '10%', label: 'ALE' },
    { top: '42%', left: '30%', label: 'MC' },
    { top: '42%', left: '50%', label: 'MC' },
    { top: '42%', left: '70%', label: 'MC' },
    { top: '42%', left: '90%', label: 'ALD' },
    { top: '18%', left: '35%', label: 'CA' },
    { top: '18%', left: '65%', label: 'CA' },
  ],
  '4-5-1': [
    { top: '87%', left: '50%', label: 'GK' },
    { top: '68%', left: '15%', label: 'LE' },
    { top: '68%', left: '38%', label: 'ZC' },
    { top: '68%', left: '62%', label: 'ZC' },
    { top: '68%', left: '85%', label: 'LD' },
    { top: '42%', left: '10%', label: 'ME' },
    { top: '42%', left: '30%', label: 'MC' },
    { top: '42%', left: '50%', label: 'MC' },
    { top: '42%', left: '70%', label: 'MC' },
    { top: '42%', left: '90%', label: 'MD' },
    { top: '16%', left: '50%', label: 'CA' },
  ],
  '4-2-3-1': [
    { top: '87%', left: '50%', label: 'GK' },
    { top: '68%', left: '15%', label: 'LE' },
    { top: '68%', left: '38%', label: 'ZC' },
    { top: '68%', left: '62%', label: 'ZC' },
    { top: '68%', left: '85%', label: 'LD' },
    { top: '50%', left: '35%', label: 'VOL' },
    { top: '50%', left: '65%', label: 'VOL' },
    { top: '32%', left: '20%', label: 'ME' },
    { top: '32%', left: '50%', label: 'MEI' },
    { top: '32%', left: '80%', label: 'MD' },
    { top: '14%', left: '50%', label: 'CA' },
  ],
  '3-4-1-2': [
    { top: '87%', left: '50%', label: 'GK' },
    { top: '68%', left: '25%', label: 'ZC' },
    { top: '68%', left: '50%', label: 'ZC' },
    { top: '68%', left: '75%', label: 'ZC' },
    { top: '45%', left: '10%', label: 'ALE' },
    { top: '45%', left: '38%', label: 'MC' },
    { top: '45%', left: '62%', label: 'MC' },
    { top: '45%', left: '90%', label: 'ALD' },
    { top: '28%', left: '50%', label: 'MEI' },
    { top: '15%', left: '35%', label: 'CA' },
    { top: '15%', left: '65%', label: 'CA' },
  ],
  '5-3-2': [
    { top: '87%', left: '50%', label: 'GK' },
    { top: '68%', left: '10%', label: 'LE' },
    { top: '68%', left: '30%', label: 'ZC' },
    { top: '68%', left: '50%', label: 'ZC' },
    { top: '68%', left: '70%', label: 'ZC' },
    { top: '68%', left: '90%', label: 'LD' },
    { top: '42%', left: '25%', label: 'MC' },
    { top: '42%', left: '50%', label: 'MC' },
    { top: '42%', left: '75%', label: 'MC' },
    { top: '18%', left: '35%', label: 'CA' },
    { top: '18%', left: '65%', label: 'CA' },
  ],
  '4-1-4-1': [
    { top: '87%', left: '50%', label: 'GK' },
    { top: '68%', left: '15%', label: 'LE' },
    { top: '68%', left: '38%', label: 'ZC' },
    { top: '68%', left: '62%', label: 'ZC' },
    { top: '68%', left: '85%', label: 'LD' },
    { top: '52%', left: '50%', label: 'VOL' },
    { top: '35%', left: '10%', label: 'ME' },
    { top: '35%', left: '38%', label: 'MC' },
    { top: '35%', left: '62%', label: 'MC' },
    { top: '35%', left: '90%', label: 'MD' },
    { top: '14%', left: '50%', label: 'CA' },
  ],
  '4-4-1-1': [
    { top: '87%', left: '50%', label: 'GK' },
    { top: '68%', left: '15%', label: 'LE' },
    { top: '68%', left: '38%', label: 'ZC' },
    { top: '68%', left: '62%', label: 'ZC' },
    { top: '68%', left: '85%', label: 'LD' },
    { top: '45%', left: '15%', label: 'ME' },
    { top: '45%', left: '38%', label: 'MC' },
    { top: '45%', left: '62%', label: 'MC' },
    { top: '45%', left: '85%', label: 'MD' },
    { top: '26%', left: '50%', label: 'SS' },
    { top: '14%', left: '50%', label: 'CA' },
  ],
  '3-4-3': [
    { top: '87%', left: '50%', label: 'GK' },
    { top: '68%', left: '25%', label: 'ZC' },
    { top: '68%', left: '50%', label: 'ZC' },
    { top: '68%', left: '75%', label: 'ZC' },
    { top: '42%', left: '10%', label: 'ME' },
    { top: '42%', left: '38%', label: 'MC' },
    { top: '42%', left: '62%', label: 'MC' },
    { top: '42%', left: '90%', label: 'MD' },
    { top: '18%', left: '20%', label: 'PE' },
    { top: '14%', left: '50%', label: 'CA' },
    { top: '18%', left: '80%', label: 'PD' },
  ],
};

const app = {
  state: {
    language: 'pt-BR',
    strings: {},
    newGameData: {
      teamPool: [],
      randomTeams: []
    },
    gameState: null,
    currentSquad: []
  },

  init: async () => {
    app.log("App initializing...");
    I18N.setLanguage('pt-BR');
    I18N.applyToDOM();
    await app.loadFlags();
    app.log("Flags loaded.");
  },

  log: (msg, level = 'info') => {
    const ts = new Date().toISOString();
    if (level === 'error') console.error(`[${ts}] ERROR: ${msg}`);
    else console.log(`[${ts}] INFO: ${msg}`);
  },

  loadJSON: async (path) => {
    try {
      const res = await fetch(path);
      if (!res.ok) return null;
      return await res.json();
    } catch (e) {
      return null;
    }
  },

  // ─── Language ───────────────────────────────────────────────────────────

  loadFlags: async () => {
    const data = await app.loadJSON('assets/JSON/flags.json');
    if (data) app.renderFlags(data.paises);
  },

  renderFlags: (countries) => {
    const grid = document.getElementById('flags-grid');
    grid.innerHTML = '';
    countries.forEach(country => {
      const btn = document.createElement('div');
      btn.className = 'flag-btn';
      btn.innerHTML = `
        <div class="flag-emoji">${country.flag}</div>
        <div class="flag-name">${country.nome}</div>
      `;
      btn.onclick = () => app.selectLanguage(country.language.code);
      grid.appendChild(btn);
    });
  },

  selectLanguage: async (code) => {
    // Map country language codes to our supported i18n languages
    const langMap = { 'pt-BR': 'pt-BR', 'es': 'es', 'es-MX': 'es', 'fr': 'fr' };
    const i18nCode = langMap[code] || 'en';
    app.state.language = i18nCode;
    I18N.setLanguage(i18nCode);
    I18N.applyToDOM();
    app.renderStartMenuFromI18N();
    app.showScreen('start');
  },

  // ─── Start Menu ─────────────────────────────────────────────────────────

  renderStartMenuFromI18N: () => {
    const container = document.getElementById('menu-buttons');
    container.innerHTML = '';
    const items = [
      { id: 'start_game', key: 'start_game' },
      { id: 'continue_game', key: 'continue_game' },
      { id: 'options', key: 'options' },
      { id: 'exit', key: 'exit' },
    ];
    items.forEach(item => {
      const btn = document.createElement('button');
      btn.className = 'menu-btn';
      btn.textContent = I18N.t(item.key);
      btn.onclick = () => app.handleMenuAction(item.id);
      container.appendChild(btn);
    });
  },

  handleMenuAction: (actionId) => {
    switch (actionId) {
      case 'start_game': app.showGameModeScreen(); break;
      case 'continue_game': app.showLoadScreen(); break;
      case 'options': app.showScreen('options'); break;
      case 'exit': window.close(); break;
    }
  },

  // ─── Load Game ──────────────────────────────────────────────────────────

  showLoadScreen: async () => {
    app.showScreen('load');
    const list = document.getElementById('save-list');
    list.innerHTML = `<div style="text-align:center; color:#888;">${I18N.t('loading')}</div>`;

    try {
      const saves = await invoke('get_saved_games');
      list.innerHTML = '';
      if (saves.length === 0) {
        list.innerHTML = `<div style="text-align:center;">${I18N.t('no_saves')}</div>`;
        return;
      }
      saves.forEach(s => {
        const div = document.createElement('div');
        div.className = 'save-item';
        div.style.cssText = 'padding:1rem; border-bottom:1px solid rgba(255,255,255,0.1); cursor:pointer; display:flex; justify-content:space-between;';
        div.onmouseover = () => div.style.background = 'rgba(255,255,255,0.05)';
        div.onmouseout = () => div.style.background = 'transparent';
        div.innerHTML = `
          <div>
            <div style="font-weight:bold; color:white;">${s.manager_name}</div>
            <div style="font-size:0.9rem; color:#aaa;">${s.club}</div>
          </div>
          <div style="text-align:right;">
            <div style="color:var(--accent-color);">${s.date}</div>
          </div>
        `;
        div.onclick = () => app.loadGame(s.slot_id);
        list.appendChild(div);
      });
    } catch (e) {
      app.log('Erro ao carregar saves: ' + e, 'error');
      list.innerHTML = '<div style="color:red; text-align:center;">Falha ao carregar lista de saves.</div>';
    }
  },

  loadGame: async (slotId) => {
    if (!confirm(I18N.t('load_confirm'))) return;
    try {
      const gameState = await invoke('load_game', { slotId });
      app.state.gameState = {
        meta: { clubName: gameState.club_name, managerName: gameState.manager_name, clubId: gameState.club_id },
        game: { dayLabel: gameState.date }
      };
      app.renderGameHub();
      app.showScreen('news');
    } catch (e) {
      console.error(e);
      alert(I18N.t('load_fail') + ': ' + e);
    }
  },

  // ─── New Game ───────────────────────────────────────────────────────────

  showGameModeScreen: () => {
    app.showScreen('gamemode');
  },

  selectGameMode: async (mode) => {
    app.state.selectedGameMode = mode;
    await app.prepNewGame(mode);
  },

  prepNewGame: async (gameMode) => {
    const mode = gameMode || app.state.selectedGameMode || 'Sandbox';

    try {
      const clubs = await invoke('get_available_clubs', { gameMode: mode });
      if (!clubs || clubs.length === 0) {
        alert(I18N.t('no_clubs'));
        return;
      }

      app.state.newGameData.teamPool = clubs;

      // Shuffle and pick 6
      const pool = [...clubs];
      for (let i = pool.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [pool[i], pool[j]] = [pool[j], pool[i]];
      }
      app.state.newGameData.randomTeams = pool.slice(0, 6);
    } catch (e) {
      console.error("Erro ao carregar clubes:", e);
      alert(I18N.t('no_clubs'));
      return;
    }

    // Populate Nations
    const natSelect = document.getElementById('manager-nationality');
    if (natSelect.children.length === 0) {
      const flagsData = await app.loadJSON('assets/JSON/flags.json');
      if (flagsData) {
        flagsData.paises.sort((a, b) => a.nome.localeCompare(b.nome));
        flagsData.paises.forEach(p => {
          const opt = document.createElement('option');
          opt.value = p.nome;
          opt.textContent = `${p.flag} ${p.nome}`;
          natSelect.appendChild(opt);
        });
      }
    }

    app.renderTeamGrid();
    app.showScreen('newgame');
  },

  renderTeamGrid: () => {
    const grid = document.getElementById('team-selection-grid');
    grid.innerHTML = '';
    app.state.newGameData.randomTeams.forEach(team => {
      const card = document.createElement('div');
      card.className = 'team-card';
      card.dataset.id = team.id;
      card.onclick = () => app.selectTeam(team.id);
      card.innerHTML = `
        <div class="team-name">${team.nome}</div>
        <div class="team-stripe" style="background: linear-gradient(90deg, ${team.corPrimaria || '#333'}, ${team.corSecundaria || '#666'})"></div>
        <div style="font-size:0.75rem; color:#888; margin-top:4px;">${team.division || ''} • Rep: ${team.reputation || '?'}</div>
      `;
      grid.appendChild(card);
    });
    document.getElementById('selected-team-id').value = '';
    document.getElementById('btn-start-career').disabled = true;
  },

  selectTeam: (teamId) => {
    document.querySelectorAll('.team-card').forEach(el => {
      el.classList.toggle('selected', el.dataset.id == teamId);
    });
    document.getElementById('selected-team-id').value = teamId;
    document.getElementById('btn-start-career').disabled = false;
  },

  // ─── Create Career ──────────────────────────────────────────────────────

  createCareer: async () => {
    const name = document.getElementById('manager-name').value;
    const surname = document.getElementById('manager-surname').value;
    const teamId = document.getElementById('selected-team-id').value;

    if (!name || !surname || !teamId) {
      alert(I18N.t('fill_all'));
      return;
    }

    try {
      const gameState = await invoke("start_new_game", {
        name,
        surname,
        teamId: teamId.toString(),
        gameMode: app.state.selectedGameMode || 'Sandbox'
      });

      app.state.gameState = {
        meta: { managerName: `${name} ${surname}`, clubId: teamId, clubName: gameState.club_name },
        game: { dayLabel: gameState.date },
        gs: gameState
      };

      app.renderGameHub();
      app.showScreen('news');
    } catch (e) {
      console.error("Falha ao iniciar jogo:", e);
      alert("Erro ao criar carreira: " + e);
    }
  },

  // ─── Game Hub ───────────────────────────────────────────────────────────

  renderGameHub: async () => {
    // Update HUD from backend
    try {
      const gs = await invoke('get_game_state');
      if (gs) {
        app.state.gameState.gs = gs;
        app.state.gameState.meta.clubName = gs.club_name;
        app.state.gameState.meta.clubId = gs.club_id;
        app.state.gameState.game = { dayLabel: gs.date };

        document.getElementById('hud-club-name').textContent = gs.club_name;
        document.getElementById('hud-game-date').textContent = gs.date;
        document.getElementById('hud-meta').innerHTML =
          `<span class="meta-item">${gs.division}</span>
           <span class="meta-item">• Pos: ${gs.position}</span>
           <span class="meta-item">• ${gs.balance}</span>`;
      }
    } catch (e) {
      console.error("Erro ao obter estado:", e);
    }

    // Load inbox
    app.loadInbox();
  },

  loadInbox: async () => {
    try {
      const messages = await invoke('get_inbox');
      const list = document.getElementById('inbox-list');
      list.innerHTML = '';

      if (!messages || messages.length === 0) {
        list.innerHTML = `<div style="padding:1rem; color:#666;">${I18N.t('no_messages')}</div>`;
        return;
      }

      messages.forEach((msg, index) => {
        const item = document.createElement('div');
        item.className = `inbox-item ${msg.unread ? 'unread' : ''}`;
        if (index === 0) item.classList.add('active');
        item.innerHTML = `
          <div class="msg-header-row">
            <span class="msg-icon">${msg.type === 'system' ? '📢' : '📧'}</span>
            <span class="msg-time">${msg.time}</span>
          </div>
          <div class="msg-title">${msg.title}</div>
        `;
        item.onclick = () => app.openMessage(msg, item);
        list.appendChild(item);
      });

      if (messages.length > 0) {
        app.openMessage(messages[0], list.children[0]);
      }
    } catch (e) {
      console.error("Erro ao carregar inbox:", e);
    }
  },

  openMessage: (msg, element) => {
    document.querySelectorAll('.inbox-item').forEach(el => el.classList.remove('active'));
    if (element) element.classList.add('active');

    const pane = document.getElementById('reading-pane');
    pane.innerHTML = `
      <div class="article-header">
        <div class="article-title">${msg.title}</div>
        <div class="article-meta">
          <span>${msg.date} ${msg.time}</span>
          <span>•</span>
          <span>${(msg.tags || []).join(', ')}</span>
        </div>
      </div>
      <div class="article-body">
        <p>${msg.text}</p>
      </div>
    `;
    if (element) element.classList.remove('unread');
  },

  // ─── Tab Navigation ─────────────────────────────────────────────────────

  toggleTab: (tabKey) => {
    document.querySelectorAll('.nav-tab').forEach(el => el.classList.remove('active'));
    const tab = document.querySelector(`.nav-tab[data-tab="${tabKey}"]`);
    if (tab) tab.classList.add('active');

    // Hide all views
    const views = ['inbox-list', 'reading-pane', 'squad-view', 'tactics-view', 'comps-view', 'transfers-view', 'finance-view', 'fixtures-view'];
    views.forEach(v => {
      const el = document.getElementById(v);
      if (el) el.style.display = 'none';
    });

    if (tabKey === 'inbox') {
      document.getElementById('inbox-list').style.display = 'flex';
      document.getElementById('reading-pane').style.display = 'block';
      app.loadInbox();
    } else if (tabKey === 'squad') {
      document.getElementById('squad-view').style.display = 'flex';
      if (app.state.gameState) app.loadSquad(app.state.gameState.meta.clubId);
    } else if (tabKey === 'tactics') {
      document.getElementById('tactics-view').style.display = 'block';
      app.renderTactics();
    } else if (tabKey === 'standings') {
      document.getElementById('comps-view').style.display = 'block';
      app.loadCompetitions();
    } else if (tabKey === 'transfers') {
      document.getElementById('transfers-view').style.display = 'block';
    } else if (tabKey === 'finance') {
      document.getElementById('finance-view').style.display = 'block';
      app.loadFinances();
    } else if (tabKey === 'fixtures') {
      document.getElementById('fixtures-view').style.display = 'block';
      app.loadFixtures();
    }
  },

  // ─── Squad ──────────────────────────────────────────────────────────────

  squadSort: { col: null, asc: true },

  loadSquad: async (teamId) => {
    try {
      const players = await invoke('get_team_squad', { teamId: teamId.toString() });
      app.state.currentSquad = players;
      app.squadSort = { col: null, asc: true };
      app.filterSquad('ALL');
    } catch (e) {
      console.error("Erro ao carregar elenco:", e);
    }
  },

  filterSquad: (filter) => {
    document.querySelectorAll('.filter-btn').forEach(b => {
      b.classList.toggle('active', b.textContent.toUpperCase() === filter || (filter === 'ALL' && b.textContent === 'All'));
    });

    const players = app.state.currentSquad || [];
    const filtered = players.filter(p => {
      if (filter === 'ALL') return true;
      if (filter === 'GK') return p.position === 'GK';
      if (filter === 'DEF') return p.position.includes('D') || p.position.includes('WB');
      if (filter === 'MID') return p.position.includes('M');
      if (filter === 'ATT') return p.position.includes('F') || p.position.includes('S');
      return true;
    });
    app.state.filteredSquad = filtered;
    app.renderSquadTable(filtered);
  },

  sortSquad: (col) => {
    if (app.squadSort.col === col) {
      app.squadSort.asc = !app.squadSort.asc;
    } else {
      app.squadSort.col = col;
      app.squadSort.asc = true;
    }
    const posOrder = { 'GK': 0, 'DC': 1, 'DL': 2, 'DR': 3, 'DM': 4, 'MC': 5, 'ML': 5, 'MR': 5, 'AM': 6, 'FC': 7, 'FL': 7, 'FR': 7 };
    const players = [...(app.state.filteredSquad || app.state.currentSquad || [])];
    const dir = app.squadSort.asc ? 1 : -1;

    players.sort((a, b) => {
      switch (col) {
        case 'pos': return dir * ((posOrder[a.position] || 5) - (posOrder[b.position] || 5));
        case 'name': return dir * a.name.localeCompare(b.name);
        case 'age': return dir * (a.age - b.age);
        case 'nat': return dir * a.nationality.localeCompare(b.nationality);
        case 'ovr': return dir * (a.overall - b.overall);
        case 'value': return dir * (a.overall - b.overall); // approximate by ovr
        case 'cond': return dir * (a.condition - b.condition);
        default: return 0;
      }
    });
    app.renderSquadTable(players);
  },

  renderSquadTable: (players) => {
    const tbody = document.querySelector('#squad-table tbody');
    tbody.innerHTML = '';

    players.forEach(p => {
      const tr = document.createElement('tr');
      tr.style.cursor = 'pointer';
      tr.onclick = () => app.openProfile(p.id);

      let posClass = 'pos-MID';
      if (p.position === 'GK') posClass = 'pos-GK';
      else if (p.position.includes('D') || p.position.includes('WB')) posClass = 'pos-DEF';
      else if (p.position.includes('F') || p.position.includes('S')) posClass = 'pos-ATT';

      tr.innerHTML = `
        <td><span class="pos-badge ${posClass}">${p.position}</span></td>
        <td style="font-weight:600">${p.name}</td>
        <td>${p.age}</td>
        <td>${p.nationality}</td>
        <td class="${p.overall > 70 ? 'val-high' : 'val-med'}">${p.overall}</td>
        <td>${I18N.formatMoney(p.value)}</td>
        <td>${p.condition}%</td>
      `;
      tbody.appendChild(tr);
    });
  },

  // ─── Player Profile ─────────────────────────────────────────────────────

  openProfile: async (playerId) => {
    try {
      const profile = await invoke('get_player_details', { playerId: playerId.toString() });
      if (!profile) return;

      document.getElementById('p-name').textContent = profile.display.name;
      document.getElementById('p-meta').textContent =
        `${profile.display.age} ${I18N.t('years')} • ${profile.display.nationality} • ${profile.display.position}`;

      const grid = document.getElementById('p-attributes');
      grid.innerHTML = '';

      const renderCat = (title, attrs) => {
        const div = document.createElement('div');
        div.className = 'attr-category';
        div.innerHTML = `<div class="attr-cat-title">${title}</div>`;
        attrs.forEach(([k, v]) => {
          let colorClass = 'avg';
          if (v >= 16) colorClass = 'excellent';
          else if (v >= 11) colorClass = 'good';
          div.innerHTML += `
            <div class="attr-row">
              <span>${k}</span>
              <span class="attr-val ${colorClass}">${v}</span>
            </div>
          `;
        });
        grid.appendChild(div);
      };

      renderCat('Technical', profile.attributes.technical);
      renderCat('Mental', profile.attributes.mental);
      renderCat('Physical', profile.attributes.physical);

      document.getElementById('profile-modal').style.display = 'flex';
    } catch (e) {
      console.error(e);
    }
  },

  closeProfile: () => {
    document.getElementById('profile-modal').style.display = 'none';
  },

  // ─── Transfers ──────────────────────────────────────────────────────────

  searchPlayers: async () => {
    const query = document.getElementById('search-input').value;
    const results = await invoke('search_players', { query });

    const table = document.getElementById('transfer-table');
    const tbody = table.querySelector('tbody');
    const empty = document.getElementById('search-empty');
    tbody.innerHTML = '';

    if (results.length > 0) {
      table.style.display = 'table';
      empty.style.display = 'none';
      results.forEach(p => {
        const tr = document.createElement('tr');
        const posClass = p.position === 'GK' ? 'pos-GK' : p.position.includes('D') || p.position === 'DM' ? 'pos-DEF' : p.position.includes('F') ? 'pos-ATT' : 'pos-MID';
        tr.innerHTML = `
          <td style="font-weight:bold; cursor:pointer;" onclick="app.openProfile('${p.id}')">${p.name}</td>
          <td>${p.age}</td>
          <td><span class="pos-badge ${posClass}">${p.position}</span></td>
          <td>${p.club_name || 'Livre'}</td>
          <td>${I18N.formatMoney(p.value)}</td>
          <td>${p.overall}</td>
          <td>
            <button class="action-btn primary" style="padding:0.3rem 0.8rem; font-size:0.75rem;"
              onclick="event.stopPropagation(); app.makeOffer('${p.id}', '${p.name}', '${p.value}')">
              Comprar
            </button>
          </td>
        `;
        tbody.appendChild(tr);
      });
    } else {
      table.style.display = 'none';
      empty.style.display = 'block';
      empty.textContent = I18N.t('no_players');
    }
  },

  makeOffer: async (playerId, playerName, valueStr) => {
    // Parse value from display string (e.g. "£5.0M" -> 5000000)
    let amount = 0;
    const clean = valueStr.replace(/[^0-9.MKk]/g, '');
    if (clean.includes('M')) {
      amount = Math.round(parseFloat(clean) * 1000000);
    } else if (clean.toUpperCase().includes('K')) {
      amount = Math.round(parseFloat(clean) * 1000);
    } else {
      amount = parseInt(clean) || 0;
    }

    const bid = prompt(`Oferta por ${playerName}:\nValor estimado: ${valueStr}\n\nDigite o valor da proposta (ex: 5000000):`, amount.toString());
    if (!bid) return;

    const bidAmount = parseInt(bid);
    if (isNaN(bidAmount) || bidAmount <= 0) {
      alert('Valor invalido.');
      return;
    }

    try {
      const response = await invoke('offer_transfer', { playerId, amount: bidAmount });
      alert(response);
      // Refresh squad if transfer was accepted
      if (response.includes('aceita') || response.includes('accepted')) {
        if (app.state.gameState) app.loadSquad(app.state.gameState.meta.clubId);
      }
    } catch (e) {
      console.error(e);
      alert('Erro na transferencia: ' + e);
    }
  },

  sellPlayer: async (playerId, playerName) => {
    // Simple sell: list on market (other clubs will make offers via AI)
    alert(`${playerName} foi listado no mercado. Clubes interessados farao propostas em breve.`);
  },

  // ─── Tactics ────────────────────────────────────────────────────────────

  renderTactics: () => {
    const formSelect = document.getElementById('formation-select');
    if (formSelect) {
      formSelect.onchange = () => {
        app.renderFormationOnPitch(formSelect.value);
        app.saveTactics();
      };
    }
    // Render initial formation
    const formation = formSelect ? formSelect.value : '4-4-2';
    app.renderFormationOnPitch(formation);
  },

  renderFormationOnPitch: (formation) => {
    const pitch = document.getElementById('pitch-players');
    if (!pitch) return;
    // Remove only player elements, keep pitch lines
    pitch.querySelectorAll('.pitch-player').forEach(el => el.remove());

    const positions = FORMATION_POSITIONS[formation] || FORMATION_POSITIONS['4-4-2'];
    const squad = app.state.currentSquad || [];

    positions.forEach((pos, idx) => {
      const p = document.createElement('div');
      p.className = 'pitch-player';
      p.style.top = pos.top;
      p.style.left = pos.left;
      p.draggable = true;
      p.dataset.idx = idx;

      // Try to assign a player name from squad
      const playerName = squad[idx] ? squad[idx].name.split(' ').pop() : '';

      p.ondragstart = (e) => {
        e.dataTransfer.setData("text/plain", idx.toString());
        e.target.style.opacity = '0.5';
      };
      p.ondragend = (e) => e.target.style.opacity = '1';

      p.innerHTML = `<span>${pos.label}</span>${playerName ? `<span class="p-name">${playerName}</span>` : ''}`;
      pitch.appendChild(p);
    });

    // Pitch drop zone for repositioning
    pitch.ondragover = (e) => e.preventDefault();
    pitch.ondrop = (e) => {
      e.preventDefault();
      const idx = e.dataTransfer.getData("text/plain");
      const playerEl = document.querySelector(`.pitch-player[data-idx='${idx}']`);
      if (!playerEl) return;
      const rect = pitch.getBoundingClientRect();
      playerEl.style.left = ((e.clientX - rect.left) / rect.width * 100).toFixed(0) + '%';
      playerEl.style.top = ((e.clientY - rect.top) / rect.height * 100).toFixed(0) + '%';
    };
  },

  saveTactics: async () => {
    const formSelect = document.getElementById('formation-select');
    const formation = formSelect ? formSelect.value : '4-4-2';
    const mentSelect = document.getElementById('mentality-select');
    const mentality = mentSelect ? mentSelect.value : 'Balanced';
    const tempoSelect = document.getElementById('tempo-select');
    const tempo = tempoSelect ? tempoSelect.value : 'Normal';
    try {
      await invoke('update_tactics', {
        formation,
        mentality,
        tempo,
        pressing: 50,
        defLine: 50,
        width: 50,
        direct: 50
      });
    } catch (e) {
      console.error("Erro ao salvar taticas:", e);
    }
  },

  // ─── Competitions ───────────────────────────────────────────────────────

  loadCompetitions: async () => {
    try {
      const table = await invoke('get_league_table');
      if (!table) return;
      document.getElementById('comp-name').textContent = table.name;
      const tbody = document.querySelector('#league-table tbody');
      tbody.innerHTML = '';

      table.rows.forEach(r => {
        const tr = document.createElement('tr');
        let posClass = '';
        if (r.position <= 3) posClass = 'color: #4ade80; font-weight:bold;';
        else if (r.position >= 18) posClass = 'color: #f87171;';

        // Highlight user club
        const gs = app.state.gameState;
        const isUser = gs && r.club_name === gs.meta.clubName;
        if (isUser) posClass += 'background: rgba(59,130,246,0.15);';

        tr.style.cssText = posClass;
        tr.innerHTML = `
          <td>${r.position}</td>
          <td style="font-weight:600">${r.club_name}</td>
          <td>${r.played}</td>
          <td>${r.won}</td>
          <td>${r.drawn}</td>
          <td>${r.lost}</td>
          <td>${r.gf}</td>
          <td>${r.ga}</td>
          <td style="font-weight:700">${r.points}</td>
        `;
        tbody.appendChild(tr);
      });
    } catch (e) { console.error(e); }
  },

  // ─── Fixtures ───────────────────────────────────────────────────────────

  loadFixtures: async () => {
    try {
      const fixtures = await invoke('get_all_fixtures');
      const container = document.getElementById('fixtures-list');
      if (!container) return;
      container.innerHTML = '';

      if (!fixtures || fixtures.length === 0) {
        container.innerHTML = `<div style="text-align:center; color:#666; padding:2rem;">${I18N.t('no_fixtures')}</div>`;
        return;
      }

      fixtures.forEach(f => {
        const div = document.createElement('div');
        div.style.cssText = 'padding:0.5rem 1rem; border-bottom:1px solid #333; display:flex; justify-content:space-between; align-items:center;';
        const resultText = f.played ? `<span style="color:#4ade80; font-weight:bold;">${f.result}</span>` : '<span style="color:#888;">vs</span>';
        div.innerHTML = `
          <div style="flex:1;">
            <span style="color:#888; font-size:0.8rem;">R${f.round} • ${f.date}</span>
            <div>${f.home_name} ${resultText} ${f.away_name}</div>
          </div>
          <div style="font-size:0.8rem; color:#60a5fa;">${f.competition}</div>
        `;
        container.appendChild(div);
      });
    } catch (e) { console.error(e); }
  },

  // ─── Finance ────────────────────────────────────────────────────────────

  loadFinances: async () => {
    try {
      const fin = await invoke('get_finances');
      if (!fin) return;

      const container = document.getElementById('finance-content');
      if (!container) return;

      const $ = I18N.formatMoney.bind(I18N);
      container.innerHTML = `
        <div style="display:grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
          <div class="glass-container" style="padding:1.5rem;">
            <div class="panel-header">${I18N.t('balance')}</div>
            <div style="font-size:2rem; font-weight:bold; color: #4ade80; margin:1rem 0;">${$(fin.balance)}</div>
          </div>
          <div class="glass-container" style="padding:1.5rem;">
            <div class="panel-header">${I18N.t('transfer_budget')}</div>
            <div style="font-size:2rem; font-weight:bold; color: #60a5fa; margin:1rem 0;">${$(fin.transfer_budget)}</div>
          </div>
        </div>
        <div class="glass-container" style="margin-top: 2rem; padding: 1.5rem;">
          <div class="panel-header" style="margin-bottom:1rem;">${I18N.t('payroll')}</div>
          <table style="width:100%; text-align:left; border-collapse:collapse; color:#ccc;">
            <tr style="border-bottom:1px solid #333;">
              <th style="padding:0.5rem;"></th>
              <th style="padding:0.5rem; text-align:right;"></th>
            </tr>
            <tr>
              <td style="padding:0.5rem;">${I18N.t('wage_budget')}</td>
              <td style="padding:0.5rem; text-align:right; color:#60a5fa;">${$(fin.wage_budget)}</td>
            </tr>
            <tr>
              <td style="padding:0.5rem;">${I18N.t('current_wages')}</td>
              <td style="padding:0.5rem; text-align:right; color:#f87171;">${$(fin.wage_bill)}</td>
            </tr>
            <tr>
              <td style="padding:0.5rem;">${I18N.t('wage_room')}</td>
              <td style="padding:0.5rem; text-align:right; color:#4ade80;">${$(fin.wage_room)}</td>
            </tr>
          </table>
        </div>
      `;
    } catch (e) { console.error(e); }
  },

  // ─── Match ──────────────────────────────────────────────────────────────

  matchSubsUsed: 0,
  matchSubOut: null,
  matchSubIn: null,
  matchResult: null,
  matchInterval: null,
  matchIsLive: false,

  startMatch: async (homeId, awayId, homeName, awayName, competition) => {
    app.showScreen('match');
    app.matchSubsUsed = 0;
    app.matchSubOut = null;
    app.matchSubIn = null;
    app.matchResult = null;
    app.matchIsLive = true;

    // Reset UI
    document.getElementById('score-home').textContent = '0';
    document.getElementById('score-away').textContent = '0';
    document.getElementById('score-home').classList.remove('goal-flash');
    document.getElementById('score-away').classList.remove('goal-flash');
    document.getElementById('score-home-name').textContent = homeName || 'Casa';
    document.getElementById('score-away-name').textContent = awayName || 'Fora';
    document.getElementById('match-time').textContent = '00:00';
    document.getElementById('commentary-feed').innerHTML = '';
    document.getElementById('btn-finish-match').style.display = 'none';
    document.getElementById('btn-match-subs').style.display = 'inline-block';
    document.getElementById('btn-match-tactics').style.display = 'inline-block';
    document.getElementById('match-status-label').textContent = 'AO VIVO';
    document.getElementById('match-status-label').classList.remove('ended');
    document.getElementById('match-competition').textContent = competition || '';
    document.getElementById('scorers-home').innerHTML = '';
    document.getElementById('scorers-away').innerHTML = '';
    document.getElementById('subs-counter').textContent = 'Substituicoes: 0/3';
    document.getElementById('match-ratings-panel').style.display = 'none';
    document.getElementById('event-ticker').style.display = 'none';

    // Reset stats
    app.resetMatchStats();

    try {
      const result = await invoke('start_match', { homeId: homeId.toString(), awayId: awayId.toString() });
      if (result) {
        app.matchResult = result;
        document.getElementById('score-home-name').textContent = result.home_name;
        document.getElementById('score-away-name').textContent = result.away_name;
        app.playMatch(result);
      }
    } catch (e) {
      console.error("Erro na partida:", e);
    }
  },

  resetMatchStats: () => {
    const ids = ['possession', 'shots', 'sot', 'fouls', 'corners', 'yellows', 'reds'];
    ids.forEach(id => {
      const h = document.getElementById(`stat-${id}-home`);
      const a = document.getElementById(`stat-${id}-away`);
      if (h) h.textContent = id === 'possession' ? '50%' : '0';
      if (a) a.textContent = id === 'possession' ? '50%' : '0';
    });
  },

  playMatch: (result) => {
    let minute = 0;
    const speed = 120; // ms per minute tick
    let homeGoalsShown = 0;
    let awayGoalsShown = 0;

    // Pre-process events by minute for efficient lookup
    const eventsByMinute = {};
    (result.events || []).forEach(ev => {
      if (!eventsByMinute[ev.minute]) eventsByMinute[ev.minute] = [];
      eventsByMinute[ev.minute].push(ev);
    });

    // Also process highlights by minute as fallback
    const highlightsByMinute = {};
    (result.highlights || []).forEach(h => {
      const match = h.match(/^(\d+)'/);
      if (match) {
        const m = parseInt(match[1]);
        if (!highlightsByMinute[m]) highlightsByMinute[m] = [];
        highlightsByMinute[m].push(h);
      }
    });

    app.matchInterval = setInterval(() => {
      minute++;
      document.getElementById('match-time').textContent = `${minute}:00`;

      // Half time
      if (minute === 45) {
        app.addMatchEvent(45, 'HalfTime', 'INTERVALO', null);
      }

      // Process structured events for this minute
      const eventsNow = eventsByMinute[minute] || [];
      eventsNow.forEach(ev => {
        app.addMatchEvent(ev.minute, ev.event_type, ev.description, ev);

        // Update score on goal events
        if (ev.event_type === 'Goal') {
          const descLower = ev.description.toLowerCase();
          // Determine which team scored based on description
          if (descLower.includes('home') || descLower.includes(result.home_name.toLowerCase())) {
            homeGoalsShown++;
            app.flashScore('home', homeGoalsShown);
            app.addScorer('home', ev.minute, ev.description);
          } else {
            awayGoalsShown++;
            app.flashScore('away', awayGoalsShown);
            app.addScorer('away', ev.minute, ev.description);
          }
        }
      });

      // Fallback: process highlights that weren't covered by events
      if (eventsNow.length === 0) {
        const highlights = highlightsByMinute[minute] || [];
        highlights.forEach(h => {
          const isGoal = h.toLowerCase().includes('goal') || h.toLowerCase().includes('gol');
          const isYellow = h.toLowerCase().includes('yellow') || h.toLowerCase().includes('amarelo') || h.toLowerCase().includes('cartao amarelo');
          const isRed = h.toLowerCase().includes('red card') || h.toLowerCase().includes('vermelho') || h.toLowerCase().includes('cartao vermelho');

          let type = 'info';
          if (isGoal) type = 'Goal';
          else if (isRed) type = 'RedCard';
          else if (isYellow) type = 'YellowCard';

          app.addMatchEvent(minute, type, h, null);

          if (isGoal) {
            if (h.toLowerCase().includes('home')) {
              homeGoalsShown++;
              app.flashScore('home', homeGoalsShown);
              app.addScorer('home', minute, h);
            } else if (h.toLowerCase().includes('away')) {
              awayGoalsShown++;
              app.flashScore('away', awayGoalsShown);
              app.addScorer('away', minute, h);
            }
          }
        });
      }

      // Progressively update stats (interpolate toward final values)
      if (result.stats && minute <= 90) {
        const progress = minute / 90;
        app.updateMatchStatsProgressive(result.stats, progress);
      }

      if (minute >= 90) {
        clearInterval(app.matchInterval);
        app.matchInterval = null;
        app.matchIsLive = false;

        // Final whistle
        app.addMatchEvent(90, 'FullTime', 'APITO FINAL!', null);

        // Set final score
        document.getElementById('score-home').textContent = result.home_goals;
        document.getElementById('score-away').textContent = result.away_goals;

        // Update status
        document.getElementById('match-status-label').textContent = 'ENCERRADO';
        document.getElementById('match-status-label').classList.add('ended');

        // Update final stats
        if (result.stats) app.updateMatchStatsFinal(result.stats);

        // Show finish button, hide live controls
        document.getElementById('btn-finish-match').style.display = 'inline-block';
        document.getElementById('btn-match-subs').style.display = 'none';
        document.getElementById('btn-match-tactics').style.display = 'none';

        // Show player ratings if available
        if (result.player_ratings && result.player_ratings.length > 0) {
          app.renderPlayerRatings(result);
        }
      }
    }, speed);
  },

  flashScore: (side, goals) => {
    const el = document.getElementById(`score-${side}`);
    el.textContent = goals;
    el.classList.add('goal-flash');
    setTimeout(() => el.classList.remove('goal-flash'), 1500);
  },

  addScorer: (side, minute, description) => {
    const container = document.getElementById(`scorers-${side}`);
    const entry = document.createElement('div');
    entry.className = 'scorer-entry';
    entry.innerHTML = `<span style="color:#4ade80;">&#9917;</span> <span>${minute}'</span>`;
    entry.style.animation = 'comm-fade-in 0.3s ease-out';
    container.appendChild(entry);
  },

  addMatchEvent: (minute, eventType, description, rawEvent) => {
    // Show ticker for important events
    const isGoal = eventType === 'Goal';
    const isYellow = eventType === 'YellowCard';
    const isRed = eventType === 'RedCard';
    const isHalf = eventType === 'HalfTime';
    const isFull = eventType === 'FullTime';

    if (isGoal || isYellow || isRed) {
      app.showTicker(eventType, description);
    }

    // Determine icon
    let icon = '';
    let cssClass = '';
    if (isGoal) {
      icon = '&#9917;';
      cssClass = 'goal';
    } else if (isYellow) {
      icon = '<span style="display:inline-block;width:10px;height:14px;background:#facc15;border-radius:1px;"></span>';
      cssClass = 'yellow-card';
    } else if (isRed) {
      icon = '<span style="display:inline-block;width:10px;height:14px;background:#f87171;border-radius:1px;"></span>';
      cssClass = 'red-card';
    } else if (isHalf) {
      cssClass = 'halftime';
    } else if (isFull) {
      cssClass = 'fulltime';
    }

    const box = document.getElementById('commentary-feed');
    const div = document.createElement('div');
    div.className = `comm-event ${cssClass}`;

    if (isHalf || isFull) {
      div.innerHTML = `<span class="comm-text">${description}</span>`;
    } else {
      div.innerHTML = `
        <span class="comm-minute">${minute}'</span>
        <span class="comm-icon">${icon}</span>
        <span class="comm-text">${description}</span>
      `;
    }

    box.appendChild(div);
    box.scrollTop = box.scrollHeight;
  },

  showTicker: (eventType, text) => {
    const ticker = document.getElementById('event-ticker');
    const content = document.getElementById('ticker-content');

    ticker.className = 'event-ticker';
    if (eventType === 'Goal') ticker.classList.add('goal-ticker');
    else if (eventType === 'YellowCard') ticker.classList.add('card-ticker');
    else if (eventType === 'RedCard') ticker.classList.add('red-ticker');

    content.textContent = text;
    ticker.style.display = 'block';
    ticker.style.animation = 'none';
    ticker.offsetHeight; // force reflow
    ticker.style.animation = '';

    // Auto-hide after 3 seconds
    clearTimeout(app._tickerTimeout);
    app._tickerTimeout = setTimeout(() => {
      ticker.style.display = 'none';
    }, 3000);
  },

  updateMatchStatsProgressive: (stats, progress) => {
    // Interpolate stats toward final values with some noise
    const lerp = (target) => Math.round(target * progress);
    const poss = stats.home_possession || 50;

    document.getElementById('stat-possession-home').textContent = Math.round(poss) + '%';
    document.getElementById('stat-possession-away').textContent = Math.round(stats.away_possession || (100 - poss)) + '%';
    document.getElementById('stat-shots-home').textContent = lerp(stats.home_shots);
    document.getElementById('stat-shots-away').textContent = lerp(stats.away_shots);
    document.getElementById('stat-sot-home').textContent = lerp(stats.home_shots_on_target);
    document.getElementById('stat-sot-away').textContent = lerp(stats.away_shots_on_target);
    document.getElementById('stat-fouls-home').textContent = lerp(stats.home_fouls);
    document.getElementById('stat-fouls-away').textContent = lerp(stats.away_fouls);
    document.getElementById('stat-corners-home').textContent = lerp(stats.home_corners);
    document.getElementById('stat-corners-away').textContent = lerp(stats.away_corners);
    document.getElementById('stat-yellows-home').textContent = lerp(stats.home_yellow_cards);
    document.getElementById('stat-yellows-away').textContent = lerp(stats.away_yellow_cards);
    document.getElementById('stat-reds-home').textContent = lerp(stats.home_red_cards);
    document.getElementById('stat-reds-away').textContent = lerp(stats.away_red_cards);
  },

  updateMatchStatsFinal: (stats) => {
    document.getElementById('stat-possession-home').textContent = Math.round(stats.home_possession || 50) + '%';
    document.getElementById('stat-possession-away').textContent = Math.round(stats.away_possession || 50) + '%';
    document.getElementById('stat-shots-home').textContent = stats.home_shots;
    document.getElementById('stat-shots-away').textContent = stats.away_shots;
    document.getElementById('stat-sot-home').textContent = stats.home_shots_on_target;
    document.getElementById('stat-sot-away').textContent = stats.away_shots_on_target;
    document.getElementById('stat-fouls-home').textContent = stats.home_fouls;
    document.getElementById('stat-fouls-away').textContent = stats.away_fouls;
    document.getElementById('stat-corners-home').textContent = stats.home_corners;
    document.getElementById('stat-corners-away').textContent = stats.away_corners;
    document.getElementById('stat-yellows-home').textContent = stats.home_yellow_cards;
    document.getElementById('stat-yellows-away').textContent = stats.away_yellow_cards;
    document.getElementById('stat-reds-home').textContent = stats.home_red_cards;
    document.getElementById('stat-reds-away').textContent = stats.away_red_cards;
  },

  renderPlayerRatings: (result) => {
    const panel = document.getElementById('match-ratings-panel');
    const homeCol = document.getElementById('ratings-home');
    const awayCol = document.getElementById('ratings-away');
    homeCol.innerHTML = `<div style="font-size:0.8rem; font-weight:700; color:white; margin-bottom:0.5rem;">${result.home_name}</div>`;
    awayCol.innerHTML = `<div style="font-size:0.8rem; font-weight:700; color:white; margin-bottom:0.5rem;">${result.away_name}</div>`;

    const homeRatings = result.player_ratings.filter(r => r.team === 'Home').sort((a, b) => b.rating - a.rating);
    const awayRatings = result.player_ratings.filter(r => r.team === 'Away').sort((a, b) => b.rating - a.rating);

    const renderRatingRow = (r) => {
      let colorClass = 'avg';
      if (r.rating >= 8.0) colorClass = 'excellent';
      else if (r.rating >= 7.0) colorClass = 'good';
      else if (r.rating < 5.5) colorClass = 'poor';

      let icons = '';
      for (let i = 0; i < r.goals; i++) icons += '<span style="color:#4ade80;">&#9917;</span>';
      if (r.man_of_the_match) icons += '<span class="rating-motm" title="Melhor em campo">&#9733;</span>';

      // Clean player ID to just show surname-like label
      const name = r.player_id.replace('HOME_', '').replace('AWAY_', '');
      const label = `Jogador ${name}`;

      return `<div class="rating-row">
        <span class="rating-name">${label}</span>
        <span class="rating-icons">${icons}</span>
        <span class="rating-val ${colorClass}">${r.rating.toFixed(1)}</span>
      </div>`;
    };

    homeRatings.forEach(r => homeCol.innerHTML += renderRatingRow(r));
    awayRatings.forEach(r => awayCol.innerHTML += renderRatingRow(r));

    panel.style.display = 'block';
  },

  // ─── Substitution Modal ──────────────────────────────────────────────

  openMatchSubs: () => {
    if (!app.matchIsLive) return;
    if (app.matchSubsUsed >= 3) {
      alert('Limite de 3 substituicoes atingido.');
      return;
    }
    const squad = app.state.currentSquad || [];
    if (squad.length <= 11) {
      alert('Nenhum reserva disponivel.');
      return;
    }

    app.matchSubOut = null;
    app.matchSubIn = null;

    const starters = squad.slice(0, 11);
    const subs = squad.slice(11);

    const outList = document.getElementById('subs-out-list');
    outList.innerHTML = '';
    starters.forEach((p, i) => {
      const item = document.createElement('div');
      item.className = 'subs-player-item';
      item.innerHTML = `<div class="player-info"><span class="pos-badge pos-MID">${p.position}</span> <span>${p.name}</span></div><span class="player-ovr">${p.overall}</span>`;
      item.onclick = () => {
        document.querySelectorAll('#subs-out-list .subs-player-item').forEach(el => el.classList.remove('selected'));
        item.classList.add('selected');
        app.matchSubOut = i;
        app.updateSubConfirmBtn();
      };
      outList.appendChild(item);
    });

    const inList = document.getElementById('subs-in-list');
    inList.innerHTML = '';
    subs.forEach((p, i) => {
      const item = document.createElement('div');
      item.className = 'subs-player-item';
      item.innerHTML = `<div class="player-info"><span class="pos-badge pos-MID">${p.position}</span> <span>${p.name}</span></div><span class="player-ovr">${p.overall}</span>`;
      item.onclick = () => {
        document.querySelectorAll('#subs-in-list .subs-player-item').forEach(el => el.classList.remove('selected'));
        item.classList.add('selected');
        app.matchSubIn = i;
        app.updateSubConfirmBtn();
      };
      inList.appendChild(item);
    });

    document.getElementById('btn-confirm-sub').disabled = true;
    document.getElementById('subs-modal').style.display = 'flex';
  },

  updateSubConfirmBtn: () => {
    document.getElementById('btn-confirm-sub').disabled = (app.matchSubOut === null || app.matchSubIn === null);
  },

  confirmSub: () => {
    if (app.matchSubOut === null || app.matchSubIn === null) return;

    const squad = app.state.currentSquad || [];
    const starters = squad.slice(0, 11);
    const subs = squad.slice(11);

    const playerOut = starters[app.matchSubOut];
    const playerIn = subs[app.matchSubIn];

    app.matchSubsUsed++;

    // Get current minute from timer
    const timeText = document.getElementById('match-time').textContent;
    const currentMinute = parseInt(timeText) || 0;

    // Add event to commentary
    app.addMatchEvent(
      currentMinute,
      'Substitution',
      `Substituicao: sai ${playerOut.name}, entra ${playerIn.name} (${app.matchSubsUsed}/3)`,
      null
    );

    // Show ticker
    app.showTicker('info', `Substituicao: ${playerIn.name} entra no lugar de ${playerOut.name}`);

    // Update subs counter
    document.getElementById('subs-counter').textContent = `Substituicoes: ${app.matchSubsUsed}/3`;

    // Close modal
    app.closeSubsModal();

    // Disable subs button if maxed
    if (app.matchSubsUsed >= 3) {
      document.getElementById('btn-match-subs').style.opacity = '0.5';
      document.getElementById('btn-match-subs').style.pointerEvents = 'none';
    }
  },

  closeSubsModal: () => {
    document.getElementById('subs-modal').style.display = 'none';
  },

  openMatchTactics: () => {
    if (!app.matchIsLive) return;
    const options = ['Defensivo', 'Cauteloso', 'Equilibrado', 'Ofensivo', 'Ataque Total'];
    let msg = 'Alterar mentalidade:\n';
    options.forEach((o, i) => { msg += `${i + 1}. ${o}\n`; });
    const choice = prompt(msg + '\nDigite o numero:');
    if (choice && parseInt(choice) > 0 && parseInt(choice) <= options.length) {
      const selected = options[parseInt(choice) - 1];
      const timeText = document.getElementById('match-time').textContent;
      const currentMinute = parseInt(timeText) || 0;
      app.addMatchEvent(currentMinute, 'info', `Mudanca tatica: mentalidade alterada para ${selected}.`, null);
    }
  },

  finishMatch: () => {
    if (app.matchInterval) {
      clearInterval(app.matchInterval);
      app.matchInterval = null;
    }
    app.matchSubsUsed = 0;
    app.matchIsLive = false;
    app.matchResult = null;
    app.renderGameHub();
    app.showScreen('news');
  },

  // ─── Advance Day / Continue ─────────────────────────────────────────────

  advanceGame: async () => {
    try {
      // Advance day — backend processes AI matches, skips user match
      const result = await invoke('advance_day');
      if (!result) return;

      // Update HUD
      app.updateHUD(result.game_state);

      // If user has a match today, prompt to play
      if (result.user_match) {
        const m = result.user_match;
        if (confirm(`${I18N.t('match_today')}: ${m.home_name} vs ${m.away_name} (${m.competition}). ${I18N.t('play_match')}`)) {
          app.startMatch(m.home_id, m.away_id, m.home_name, m.away_name, m.competition);
          return;
        }
      }

      // Refresh inbox to show new messages (round results added by backend)
      app.loadInbox();

    } catch (e) { console.error(e); }
  },

  updateHUD: (gs) => {
    if (!gs) return;
    document.getElementById('hud-game-date').textContent = gs.date;
    document.getElementById('hud-club-name').textContent = gs.club_name;
    const meta = document.getElementById('hud-meta');
    if (meta) {
      meta.innerHTML =
        `<span class="meta-item">${gs.division}</span>
         <span class="meta-item">• Pos: ${gs.position}</span>
         <span class="meta-item">• ${I18N.formatMoney(gs.balance)}</span>`;
    }
  },

  // ─── Save ───────────────────────────────────────────────────────────────

  saveAndExit: async () => {
    try {
      await invoke('save_game');
      alert(I18N.t('game_saved'));
      app.showScreen('start');
    } catch (e) {
      console.error(e);
      alert(I18N.t('save_error') + ': ' + e);
    }
  },

  // ─── Settings Modal ─────────────────────────────────────────────────────

  openSettings: () => {
    const modal = document.getElementById('settings-modal');
    if (!modal) return;

    // Populate language options
    const langSelect = document.getElementById('settings-language');
    langSelect.innerHTML = '';
    Object.entries(I18N.languages).forEach(([code, info]) => {
      const opt = document.createElement('option');
      opt.value = code;
      opt.textContent = `${info.flag} ${info.name}`;
      if (code === I18N.current) opt.selected = true;
      langSelect.appendChild(opt);
    });

    // Populate currency options
    const curSelect = document.getElementById('settings-currency');
    curSelect.innerHTML = '';
    Object.entries(I18N.currencies).forEach(([code, info]) => {
      const opt = document.createElement('option');
      opt.value = code;
      opt.textContent = `${info.symbol} — ${info.name}`;
      if (code === I18N.currency) opt.selected = true;
      curSelect.appendChild(opt);
    });

    // Update labels
    document.getElementById('settings-title').textContent = I18N.t('settings');
    document.getElementById('settings-lang-label').textContent = I18N.t('language');
    document.getElementById('settings-cur-label').textContent = I18N.t('currency');
    document.getElementById('settings-zoom-label').textContent = I18N.t('zoom');
    document.getElementById('settings-apply-btn').textContent = I18N.t('apply');
    document.getElementById('settings-close-btn').textContent = I18N.t('close');

    modal.style.display = 'flex';
  },

  closeSettings: () => {
    document.getElementById('settings-modal').style.display = 'none';
  },

  applySettings: () => {
    const lang = document.getElementById('settings-language').value;
    const cur = document.getElementById('settings-currency').value;
    const zoom = document.getElementById('settings-zoom').value;

    I18N.setLanguage(lang);
    I18N.setCurrency(cur);
    I18N.applyToDOM();
    document.body.style.zoom = zoom;
    app.state.language = lang;

    // Re-render the game hub tabs text
    app.updateTabLabels();

    // Refresh currently active view
    if (app.state.gameState) {
      app.renderGameHub();
    }

    app.closeSettings();
  },

  updateTabLabels: () => {
    document.querySelectorAll('.nav-tab[data-tab]').forEach(tab => {
      const key = tab.getAttribute('data-tab');
      tab.textContent = I18N.t(key);
    });
  },

  applyMenuSettings: () => {
    const lang = document.getElementById('opt-language').value;
    const cur = document.getElementById('opt-currency').value;
    I18N.setLanguage(lang);
    I18N.setCurrency(cur);
    I18N.applyToDOM();
    app.state.language = lang;
    app.renderStartMenuFromI18N();
  },

  setZoom: (scale) => {
    document.body.style.zoom = scale;
  },

  showScreen: (screenId) => {
    document.querySelectorAll('.screen').forEach(el => el.classList.remove('active'));
    document.getElementById(`screen-${screenId}`).classList.add('active');
  }
};

window.app = app;
window.addEventListener('DOMContentLoaded', app.init);
