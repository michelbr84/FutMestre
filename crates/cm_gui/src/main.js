const { invoke } = window.__TAURI__.core;

const app = {
  state: {
    language: 'en',
    strings: {},
  },

  init: async () => {
    await app.loadFlags();
  },

  loadFlags: async () => {
    try {
      const response = await fetch('assets/JSON/flags.json');
      const data = await response.json();
      app.renderFlags(data.paises);
    } catch (e) {
      console.error("Failed to load flags:", e);
    }
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
    let data = null;
    const paths = [
      `assets/JSON/${code}/start.json`,
      `assets/JSON/${code.split('-')[0]}/start.json`,
      `assets/JSON/en/start.json`
    ];
    for (const path of paths) {
      try {
        const res = await fetch(path);
        if (res.ok) {
          data = await res.json();
          break;
        }
      } catch (e) { }
    }
    if (data) {
      app.state.strings = data;
      app.renderStartMenu(data);
    }
  },

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
    console.log("Action:", actionId);
    if (actionId === 'start_game') {
      app.populateNewGameForm();
      app.showScreen('newgame');
    } else if (actionId === 'continue_game') {
      alert("Searching for saved games...");
    } else if (actionId === 'options') {
      app.showScreen('options');
    } else if (actionId === 'exit') {
      // window.__TAURI__.process.exit(0);
    }
  },

  populateNewGameForm: async () => {
    const select = document.getElementById('manager-nationality');
    if (select.children.length === 0) {
      try {
        const response = await fetch('assets/JSON/flags.json');
        const data = await response.json();
        data.paises.sort((a, b) => a.nome.localeCompare(b.nome));
        data.paises.forEach(p => {
          const opt = document.createElement('option');
          opt.value = p.id;
          opt.textContent = `${p.flag} ${p.nome}`;
          select.appendChild(opt);
        });
      } catch (e) {
        console.error("Error populating nationalities", e);
      }
    }
  },

  startGame: async () => {
    const name = document.getElementById('manager-name').value;
    const surname = document.getElementById('manager-surname').value;
    const nation = document.getElementById('manager-nationality').value;
    const team = document.getElementById('manager-team').value;

    if (!name || !surname) {
      alert("Please enter your name.");
      return;
    }

    console.log("Starting game with:", { name, surname, nation, team });
    try {
      await invoke('start_new_game', { name, surname, nationId: parseInt(nation), teamId: team });
      alert("Game Started! (Backend integration pending)");
    } catch (e) {
      console.error("Failed to start game:", e);
      // Fallback to show it works even without backend
      alert("Simulation: Game Started!\nName: " + name + " " + surname);
    }
  },

  changeLanguage: async (code) => {
    await app.selectLanguage(code);
  },

  showScreen: (screenId) => {
    document.querySelectorAll('.screen').forEach(el => el.classList.remove('active'));
    document.getElementById(`screen-${screenId}`).classList.add('active');
  }
};

window.app = app;
window.addEventListener('DOMContentLoaded', app.init);
