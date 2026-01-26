const { invoke } = window.__TAURI__.core;

const app = {
  state: {
    language: 'en',
    strings: {},
    newGameData: {
      teamPool: [],
      randomTeams: []
    },
    gameState: null
  },

  init: async () => {
    await app.loadFlags();
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

  // --- 1. Language ---
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

  // --- 2. Start Menu ---
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
      case 'continue_game': alert("Saved games system coming soon."); break;
      case 'options': app.showScreen('options'); break;
      case 'exit': window.close(); break;
    }
  },

  // --- 3. New Game ---
  prepNewGame: async () => {
    const lang = app.state.language;
    const paths = [`assets/JSON/${lang}/times.json`, `assets/JSON/pt-BR/times.json`];
    let teamsData = null;
    for (const p of paths) {
      teamsData = await app.loadJSON(p);
      if (teamsData) break;
    }

    if (!teamsData) { alert("Error loading teams."); return; }

    // Shuffle & Pick 6
    app.state.newGameData.teamPool = teamsData.times;
    const pool = [...teamsData.times];
    for (let i = pool.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [pool[i], pool[j]] = [pool[j], pool[i]];
    }
    app.state.newGameData.randomTeams = pool.slice(0, 6);

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

  loadSquad: async (teamId) => {
    try {
      const players = await invoke('get_team_squad', { teamId: parseInt(teamId) });
      app.state.currentSquad = players; // Cache
      app.filterSquad('ALL');
    } catch (e) {
      console.error("Failed to load squad:", e);
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
            <td>${p.nationality.replace('Nation ', '')}</td>
            <td class="${p.overall > 70 ? 'val-high' : 'val-med'}">${p.overall}</td>
            <td>${p.value}</td>
            <td>${p.condition}%</td>
        `;
      tbody.appendChild(tr);
    });
  },

  openProfile: async (playerId) => {
    try {
      const profile = await invoke('get_player_details', { playerId: parseInt(playerId) || 0 });
      if (profile) {
        document.getElementById('p-name').textContent = profile.display.name;
        document.getElementById('p-meta').textContent = `${profile.display.age} yrs • ${profile.display.nationality} • ${profile.display.position}`;

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

        // Map Rust tuples correctly
        renderCat('Technical', profile.attributes.technical);
        renderCat('Mental', profile.attributes.mental);
        renderCat('Physical', profile.attributes.physical);

        document.getElementById('profile-modal').style.display = 'flex';
      }
    } catch (e) {
      console.error(e);
    }
  },

  closeProfile: () => {
    document.getElementById('profile-modal').style.display = 'none';
  },

  advanceGame: async () => {
    try {
      const newDate = await invoke('advance_day');
      document.getElementById('hud-game-date').textContent = newDate;
      alert(`Simulation: Advanced to ${newDate}`);
      // Here we would also refresh messages, matches, etc.
    } catch (e) {
      console.error(e);
    }
  },

  toggleTab: (tabName) => {
    // UI Logic to switch content in main hub
    document.querySelectorAll('.nav-tab').forEach(el => el.classList.remove('active'));
    // Find tab element (simplified)
    const tabs = document.querySelectorAll('.nav-tab');
    for (let t of tabs) { if (t.textContent === tabName) t.classList.add('active'); }

    if (tabName === 'Squad') {
      document.getElementById('inbox-list').style.display = 'none';
      document.getElementById('reading-pane').style.display = 'none';
      document.getElementById('squad-view').style.display = 'block';

      // Load squad if needed (using current club ID)
      if (app.state.gameState) {
        app.loadSquad(app.state.gameState.meta.clubId);
      }
    } else if (tabName === 'Inbox') {
      document.getElementById('inbox-list').style.display = 'flex';
      document.getElementById('reading-pane').style.display = 'block';
      document.getElementById('squad-view').style.display = 'none';
    }
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
            <div class="team-stripe" style="background: linear-gradient(90deg, ${team.corPrimaria}, ${team.corSecundaria})"></div>
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

  // --- 4. Game Hub (World Generation) ---
  createCareer: async () => {
    const name = document.getElementById('manager-name').value;
    const surname = document.getElementById('manager-surname').value;
    const teamId = document.getElementById('selected-team-id').value;

    if (!name || !surname || !teamId) {
      alert("Please complete the profile.");
      return;
    }

    const selectedTeam = app.state.newGameData.randomTeams.find(t => t.id == teamId);
    const template = await app.loadJSON(`assets/JSON/pt-BR/atual.json`);

    // Mock Game State
    app.state.gameState = {
      ...template,
      meta: {
        managerName: `${name} ${surname}`,
        clubId: teamId,
        clubName: selectedTeam.nome
      },
      messages: [
        {
          id: "msg-welcome",
          type: "system",
          title: "Board Welcome",
          text: `The board welcomes ${name} ${surname} as the new manager of ${selectedTeam.nome}.<br><br>Expectation: <strong>Promotion</strong><br>Transfer Budget: <strong>£500k</strong>`,
          date: "01 Jan",
          time: "09:00",
          unread: true,
          tags: ["Board"]
        },
        ...(template ? template.messages : [])
      ]
    };

    app.renderGameHub();
    app.showScreen('news');
  },

  renderGameHub: () => {
    const state = app.state.gameState;
    // Update Top Bar
    document.getElementById('hud-club-name').textContent = state.meta.clubName;
    document.getElementById('hud-game-date').textContent = state.game ? state.game.dayLabel : "01 Jan 2026";

    // Render Inbox List
    const list = document.getElementById('inbox-list');
    list.innerHTML = '';

    state.messages.forEach((msg, index) => {
      const item = document.createElement('div');
      item.className = `inbox-item ${msg.unread ? 'unread' : ''}`;
      if (index === 0) item.classList.add('active'); // Select first default

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

    if (state.messages.length > 0) {
      app.openMessage(state.messages[0], list.children[0]);
    }
  },

  openMessage: (msg, element) => {
    // Highlight interaction
    document.querySelectorAll('.inbox-item').forEach(el => el.classList.remove('active'));
    if (element) element.classList.add('active');

    // Render Reading Pane
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
        ${msg.type === 'system' ? `
        <div class="status-box">
             <strong>Board Confidence</strong><br>
             The board is pleased with your appointment.
        </div>` : ''}
    `;

    // Mark read (visual)
    if (element) element.classList.remove('unread');
    msg.unread = false;
  },



  toggleTab: (tabName) => {
    document.querySelectorAll('.nav-tab').forEach(el => el.classList.remove('active'));
    const tabs = document.querySelectorAll('.nav-tab');
    for (let t of tabs) { if (t.textContent === tabName) t.classList.add('active'); }

    // Hide all
    document.getElementById('inbox-list').style.display = 'none';
    document.getElementById('reading-pane').style.display = 'none';
    document.getElementById('squad-view').style.display = 'none';
    document.getElementById('tactics-view').style.display = 'none';

    if (tabName === 'Squad') {
      document.getElementById('squad-view').style.display = 'flex';
      // Load squad if needed
      if (app.state.gameState) app.loadSquad(app.state.gameState.meta.clubId);
    } else if (tabName === 'Inbox') {
      document.getElementById('inbox-list').style.display = 'flex';
      document.getElementById('reading-pane').style.display = 'block';
    } else if (tabName === 'Tactics') {
      document.getElementById('tactics-view').style.display = 'block';
      app.renderTactics();
    }
  },

  renderTactics: () => {
    const pitch = document.getElementById('pitch-players');
    pitch.innerHTML = '';

    const positions = [
      { top: '85%', left: '50%', name: 'GK' },
      { top: '70%', left: '20%', name: 'LB' },
      { top: '70%', left: '40%', name: 'CB' },
      { top: '70%', left: '60%', name: 'CB' },
      { top: '70%', left: '80%', name: 'RB' },
      { top: '45%', left: '30%', name: 'CM' },
      { top: '45%', left: '70%', name: 'CM' },
      { top: '25%', left: '20%', name: 'LW' },
      { top: '25%', left: '80%', name: 'RW' },
      { top: '15%', left: '50%', name: 'ST' },
    ];

    positions.forEach(pos => {
      const p = document.createElement('div');
      p.className = 'pitch-player';
      p.style.top = pos.top;
      p.style.left = pos.left;
      p.innerHTML = `<span>${pos.name}</span><span class="p-name">Player</span>`;
      pitch.appendChild(p);
    });

    const bench = document.getElementById('bench-list');
    bench.innerHTML = '';
    for (let i = 0; i < 7; i++) {
      bench.innerHTML += `<div class="bench-item"><span>Bench Warmer ${i + 1}</span> <span>MID</span></div>`;
    }
  },

  startMatch: async () => {
    app.showScreen('match');
    document.getElementById('score-home').textContent = '0';
    document.getElementById('score-away').textContent = '0';
    document.getElementById('match-time').textContent = '00:00';
    document.getElementById('commentary-feed').innerHTML = '';
    document.getElementById('btn-finish-match').style.display = 'none';

    try {
      const myClub = parseInt(app.state.gameState.meta.clubId);
      const oppClub = myClub === 1 ? 2 : 1;
      const result = await invoke('start_match', { homeId: myClub, awayId: oppClub });
      app.playMatch(result);
    } catch (e) {
      console.error("Match failed", e);
    }
  },

  playMatch: (result) => {
    let minute = 0;
    const totalMinutes = 90;
    const speed = 100;
    const tick = setInterval(() => {
      minute++;
      document.getElementById('match-time').textContent = `${minute}:00`;

      result.highlights.forEach(h => {
        if (h.startsWith(`${minute}'`)) {
          app.addCommentary(h, h.includes("GOAL"));
          if (h.includes("Home team scores")) {
            let s = document.getElementById('score-home');
            s.textContent = parseInt(s.textContent) + 1;
          } else if (h.includes("Away team scores")) {
            let s = document.getElementById('score-away');
            s.textContent = parseInt(s.textContent) + 1;
          }
        }
      });

      if (minute >= totalMinutes) {
        clearInterval(tick);
        app.addCommentary("FULL TIME -- Match Ended", true);
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
    app.showScreen('news');
  },

  advanceGame: async () => {
    if (confirm("Play Match Day?")) {
      app.startMatch();
    } else {
      try {
        const newDate = await invoke('advance_day');
        document.getElementById('hud-game-date').textContent = newDate;
      } catch (e) { console.error(e); }
    }
  },

  changeLanguage: async (code) => {
    await app.selectLanguage(code);
    app.showScreen('options');
  },

  showScreen: (screenId) => {
    document.querySelectorAll('.screen').forEach(el => el.classList.remove('active'));
    document.getElementById(`screen-${screenId}`).classList.add('active');
  }
};

window.app = app;
window.addEventListener('DOMContentLoaded', app.init);
