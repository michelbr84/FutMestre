const { invoke } = window.__TAURI__.core;

const app = {
  state: {
    language: 'en',
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
    app.state.language = code;
    await app.loadStrings(code);
    app.showScreen('start');
  },

  loadStrings: async (code) => {
    const paths = [
      `assets/JSON/${code}/start.json`,
      `assets/JSON/${code.split('-')[0]}/start.json`,
      `assets/JSON/en/start.json`
    ];
    let data = null;
    for (const p of paths) {
      data = await app.loadJSON(p);
      if (data) break;
    }
    if (data) {
      app.state.strings = data;
      app.renderStartMenu(data);
    }
  },

  // ─── Start Menu ─────────────────────────────────────────────────────────

  renderStartMenu: (data) => {
    const container = document.getElementById('menu-buttons');
    container.innerHTML = '';
    if (data.menu_inicial) {
      data.menu_inicial.forEach(item => {
        const btn = document.createElement('button');
        btn.className = 'menu-btn';
        btn.textContent = item.label;
        btn.onclick = () => app.handleMenuAction(item.id);
        container.appendChild(btn);
      });
    }
  },

  handleMenuAction: (actionId) => {
    switch (actionId) {
      case 'start_game': app.prepNewGame(); break;
      case 'continue_game': app.showLoadScreen(); break;
      case 'options': app.showScreen('options'); break;
      case 'exit': window.close(); break;
    }
  },

  // ─── Load Game ──────────────────────────────────────────────────────────

  showLoadScreen: async () => {
    app.showScreen('load');
    const list = document.getElementById('save-list');
    list.innerHTML = '<div style="text-align:center; color:#888;">Carregando...</div>';

    try {
      const saves = await invoke('get_saved_games');
      list.innerHTML = '';
      if (saves.length === 0) {
        list.innerHTML = '<div style="text-align:center;">Nenhum jogo salvo encontrado.</div>';
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
    if (!confirm('Carregar este save? Progresso nao salvo sera perdido.')) return;
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
      alert("Falha ao carregar jogo: " + e);
    }
  },

  // ─── New Game ───────────────────────────────────────────────────────────

  prepNewGame: async () => {
    // Load clubs from backend (real data)
    try {
      const clubs = await invoke('get_available_clubs');
      if (!clubs || clubs.length === 0) {
        alert("Erro: nenhum clube encontrado. Verifique os dados em assets/data/.");
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
      alert("Erro ao carregar clubes.");
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
      alert("Preencha todos os campos.");
      return;
    }

    try {
      const gameState = await invoke("start_new_game", {
        name,
        surname,
        teamId: teamId.toString()
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
        list.innerHTML = '<div style="padding:1rem; color:#666;">Nenhuma mensagem.</div>';
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

  toggleTab: (tabName) => {
    document.querySelectorAll('.nav-tab').forEach(el => el.classList.remove('active'));
    const tabs = document.querySelectorAll('.nav-tab');
    for (let t of tabs) { if (t.textContent === tabName) t.classList.add('active'); }

    // Hide all views
    const views = ['inbox-list', 'reading-pane', 'squad-view', 'tactics-view', 'comps-view', 'transfers-view', 'finance-view', 'fixtures-view'];
    views.forEach(v => {
      const el = document.getElementById(v);
      if (el) el.style.display = 'none';
    });

    if (tabName === 'Inbox') {
      document.getElementById('inbox-list').style.display = 'flex';
      document.getElementById('reading-pane').style.display = 'block';
      app.loadInbox();
    } else if (tabName === 'Squad') {
      document.getElementById('squad-view').style.display = 'flex';
      if (app.state.gameState) app.loadSquad(app.state.gameState.meta.clubId);
    } else if (tabName === 'Tactics') {
      document.getElementById('tactics-view').style.display = 'block';
      app.renderTactics();
    } else if (tabName === 'Competitions') {
      document.getElementById('comps-view').style.display = 'block';
      app.loadCompetitions();
    } else if (tabName === 'Transfers') {
      document.getElementById('transfers-view').style.display = 'block';
    } else if (tabName === 'Finance') {
      document.getElementById('finance-view').style.display = 'block';
      app.loadFinances();
    } else if (tabName === 'Fixtures') {
      document.getElementById('fixtures-view').style.display = 'block';
      app.loadFixtures();
    }
  },

  // ─── Squad ──────────────────────────────────────────────────────────────

  loadSquad: async (teamId) => {
    try {
      const players = await invoke('get_team_squad', { teamId: teamId.toString() });
      app.state.currentSquad = players;
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
    app.renderSquadTable(filtered);
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
        <td>${p.value}</td>
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
        `${profile.display.age} anos • ${profile.display.nationality} • ${profile.display.position}`;

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
        tr.innerHTML = `
          <td style="font-weight:bold; cursor:pointer;" onclick="app.openProfile('${p.id}')">${p.name}</td>
          <td>${p.age}</td>
          <td>${p.position}</td>
          <td>${p.club_name || '-'}</td>
          <td>${p.value}</td>
        `;
        tbody.appendChild(tr);
      });
    } else {
      table.style.display = 'none';
      empty.style.display = 'block';
      empty.textContent = 'Nenhum jogador encontrado.';
    }
  },

  // ─── Tactics ────────────────────────────────────────────────────────────

  renderTactics: () => {
    const pitch = document.getElementById('pitch-players');
    pitch.innerHTML = '';

    const positions = [
      { top: '85%', left: '50%', name: 'GK' },
      { top: '70%', left: '20%', name: 'LE' },
      { top: '70%', left: '40%', name: 'ZC' },
      { top: '70%', left: '60%', name: 'ZC' },
      { top: '70%', left: '80%', name: 'LD' },
      { top: '45%', left: '30%', name: 'MC' },
      { top: '45%', left: '70%', name: 'MC' },
      { top: '25%', left: '20%', name: 'PE' },
      { top: '25%', left: '80%', name: 'PD' },
      { top: '15%', left: '50%', name: 'CA' },
    ];

    positions.forEach((pos, idx) => {
      const p = document.createElement('div');
      p.className = 'pitch-player';
      p.style.top = pos.top;
      p.style.left = pos.left;
      p.draggable = true;
      p.dataset.idx = idx;

      p.ondragstart = (e) => {
        e.dataTransfer.setData("text/plain", idx);
        e.target.style.opacity = '0.5';
      };
      p.ondragend = (e) => e.target.style.opacity = '1';
      p.innerHTML = `<span>${pos.name}</span>`;
      pitch.appendChild(p);
    });

    pitch.ondragover = (e) => e.preventDefault();
    pitch.ondrop = (e) => {
      e.preventDefault();
      const idx = e.dataTransfer.getData("text/plain");
      const playerEl = document.querySelector(`.pitch-player[data-idx='${idx}']`);
      const rect = pitch.getBoundingClientRect();
      playerEl.style.left = ((e.clientX - rect.left) / rect.width * 100).toFixed(0) + '%';
      playerEl.style.top = ((e.clientY - rect.top) / rect.height * 100).toFixed(0) + '%';
    };

    // Formation selector
    const formSelect = document.getElementById('formation-select');
    if (formSelect) {
      formSelect.onchange = () => app.saveTactics();
    }
  },

  saveTactics: async () => {
    const formSelect = document.getElementById('formation-select');
    const formation = formSelect ? formSelect.value : '4-4-2';
    try {
      await invoke('update_tactics', {
        formation,
        mentality: 'Balanced',
        tempo: 'Normal',
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
      const fixtures = await invoke('get_fixtures');
      const container = document.getElementById('fixtures-list');
      if (!container) return;
      container.innerHTML = '';

      if (!fixtures || fixtures.length === 0) {
        container.innerHTML = '<div style="text-align:center; color:#666; padding:2rem;">Nenhum jogo agendado.</div>';
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

      container.innerHTML = `
        <div style="display:grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
          <div class="glass-container" style="padding:1.5rem;">
            <div class="panel-header">Saldo</div>
            <div style="font-size:2rem; font-weight:bold; color: #4ade80; margin:1rem 0;">${fin.balance}</div>
          </div>
          <div class="glass-container" style="padding:1.5rem;">
            <div class="panel-header">Orcamento de Transferencias</div>
            <div style="font-size:2rem; font-weight:bold; color: #60a5fa; margin:1rem 0;">${fin.transfer_budget}</div>
          </div>
        </div>
        <div class="glass-container" style="margin-top: 2rem; padding: 1.5rem;">
          <div class="panel-header" style="margin-bottom:1rem;">Folha Salarial</div>
          <table style="width:100%; text-align:left; border-collapse:collapse; color:#ccc;">
            <tr style="border-bottom:1px solid #333;">
              <th style="padding:0.5rem;">Categoria</th>
              <th style="padding:0.5rem; text-align:right;">Valor</th>
            </tr>
            <tr>
              <td style="padding:0.5rem;">Orcamento Salarial</td>
              <td style="padding:0.5rem; text-align:right; color:#60a5fa;">${fin.wage_budget}</td>
            </tr>
            <tr>
              <td style="padding:0.5rem;">Folha Atual</td>
              <td style="padding:0.5rem; text-align:right; color:#f87171;">${fin.wage_bill}</td>
            </tr>
            <tr>
              <td style="padding:0.5rem;">Espaco Disponivel</td>
              <td style="padding:0.5rem; text-align:right; color:#4ade80;">${fin.wage_room}</td>
            </tr>
          </table>
        </div>
      `;
    } catch (e) { console.error(e); }
  },

  // ─── Match ──────────────────────────────────────────────────────────────

  startMatch: async (homeId, awayId, homeName, awayName) => {
    app.showScreen('match');
    document.getElementById('score-home').textContent = '0';
    document.getElementById('score-away').textContent = '0';
    document.getElementById('score-home-name').textContent = homeName || 'Home';
    document.getElementById('score-away-name').textContent = awayName || 'Away';
    document.getElementById('match-time').textContent = '00:00';
    document.getElementById('commentary-feed').innerHTML = '';
    document.getElementById('btn-finish-match').style.display = 'none';

    try {
      const result = await invoke('start_match', { homeId: homeId.toString(), awayId: awayId.toString() });
      if (result) {
        document.getElementById('score-home-name').textContent = result.home_name;
        document.getElementById('score-away-name').textContent = result.away_name;
        app.playMatch(result);
      }
    } catch (e) {
      console.error("Erro na partida:", e);
    }
  },

  playMatch: (result) => {
    let minute = 0;
    const speed = 100;
    const tick = setInterval(() => {
      minute++;
      document.getElementById('match-time').textContent = `${minute}:00`;

      result.highlights.forEach(h => {
        if (h.startsWith(`${minute}'`)) {
          app.addCommentary(h, h.toLowerCase().includes("goal") || h.toLowerCase().includes("gol"));
          if (h.includes("Home") || h.includes("home")) {
            let s = document.getElementById('score-home');
            s.textContent = parseInt(s.textContent) + 1;
          } else if (h.includes("Away") || h.includes("away")) {
            let s = document.getElementById('score-away');
            s.textContent = parseInt(s.textContent) + 1;
          }
        }
      });

      if (minute >= 90) {
        clearInterval(tick);
        app.addCommentary("FINAL DE JOGO!", true);
        document.getElementById('score-home').textContent = result.home_goals;
        document.getElementById('score-away').textContent = result.away_goals;
        document.getElementById('btn-finish-match').style.display = 'block';
      }
    }, speed);
  },

  addCommentary: (text, important = false) => {
    const box = document.getElementById('commentary-feed');
    const div = document.createElement('div');
    div.className = `comm-event ${important ? 'goal' : ''}`;
    div.textContent = text;
    box.appendChild(div);
    box.scrollTop = box.scrollHeight;
  },

  finishMatch: () => {
    app.renderGameHub();
    app.showScreen('news');
  },

  // ─── Advance Day / Continue ─────────────────────────────────────────────

  advanceGame: async () => {
    // Check if there's a match today
    try {
      const match = await invoke('check_match_today');
      if (match) {
        if (confirm(`Jogo hoje: ${match.home_name} vs ${match.away_name} (${match.competition}). Jogar?`)) {
          app.startMatch(match.home_id, match.away_id, match.home_name, match.away_name);
          return;
        }
      }
    } catch (e) {
      console.error(e);
    }

    // No match or skipped - advance day
    try {
      const gs = await invoke('advance_day');
      if (gs) {
        document.getElementById('hud-game-date').textContent = gs.date;
        document.getElementById('hud-club-name').textContent = gs.club_name;
        if (document.getElementById('hud-meta')) {
          document.getElementById('hud-meta').innerHTML =
            `<span class="meta-item">${gs.division}</span>
             <span class="meta-item">• Pos: ${gs.position}</span>
             <span class="meta-item">• ${gs.balance}</span>`;
        }
      }
    } catch (e) { console.error(e); }
  },

  // ─── Save ───────────────────────────────────────────────────────────────

  saveAndExit: async () => {
    try {
      await invoke('save_game');
      alert('Jogo salvo com sucesso!');
      app.showScreen('start');
    } catch (e) {
      console.error(e);
      alert('Erro ao salvar: ' + e);
    }
  },

  // ─── Settings ───────────────────────────────────────────────────────────

  changeLanguage: async (code) => {
    await app.selectLanguage(code);
    app.showScreen('options');
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
