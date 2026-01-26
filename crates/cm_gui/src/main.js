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

  advanceGame: () => {
    alert("Simulation: Advancing to next day...");
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
