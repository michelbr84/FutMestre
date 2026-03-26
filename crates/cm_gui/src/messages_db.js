// ─── Messages Database — Multilingual ────────────────────────────────────────
// Each category has an array of message templates per language.
// Placeholders: {club}, {player}, {opponent}, {competition}, {position}, {score}, {date}
// Users can add more messages by appending to the arrays.

const MESSAGES_DB = {

  // ─── Welcome Messages (shown at game start) ──────────────────────────
  welcome: {
    'en': [
      { title: "Welcome!", text: "The board of {club} welcomes you. Show us what you can do!" },
      { title: "New Era", text: "A new chapter begins at {club}. The fans are hopeful." },
    ],
    'pt-BR': [
      { title: "Bem-vindo!", text: "A diretoria do {club} lhe da as boas-vindas. Mostre do que voce e capaz!" },
      { title: "Nova Era", text: "Um novo capitulo comeca no {club}. A torcida esta esperancosa." },
    ],
    'es': [
      { title: "Bienvenido!", text: "La directiva del {club} le da la bienvenida. Demuestre lo que puede hacer!" },
      { title: "Nueva Era", text: "Un nuevo capitulo comienza en {club}. La aficion tiene esperanza." },
    ],
    'fr': [
      { title: "Bienvenue!", text: "La direction du {club} vous souhaite la bienvenue. Montrez-nous ce que vous savez faire!" },
      { title: "Nouvelle Ere", text: "Un nouveau chapitre commence au {club}. Les supporters sont pleins d'espoir." },
    ],
  },

  // ─── Daily Tips / Random Messages (shown when advancing day) ──────────
  daily: {
    'en': [
      { title: "Training Report", text: "The squad trained well today. Fitness levels are improving." },
      { title: "Scout Report", text: "Our scouts are monitoring several promising players in the region." },
      { title: "Fan Mood", text: "The fans are excited about the upcoming fixtures." },
      { title: "Press Conference", text: "The press is asking about your plans for the transfer window." },
      { title: "Board Meeting", text: "The board reviewed the club's financial situation. Everything is stable." },
      { title: "Youth Academy", text: "The youth academy is producing some interesting talent." },
      { title: "Stadium News", text: "Stadium maintenance is on schedule. Capacity remains unchanged." },
      { title: "Injury Update", text: "The medical staff reports no new injuries from today's training session." },
      { title: "Tactical Analysis", text: "Your assistant has prepared an analysis of the next opponent." },
      { title: "Transfer Rumor", text: "Rumors suggest a rival club is interested in one of your players." },
      { title: "Community Event", text: "The club organized a community event. Fan engagement is growing." },
      { title: "Weather Forecast", text: "Good weather expected for the upcoming match day." },
    ],
    'pt-BR': [
      { title: "Relatorio de Treino", text: "O elenco treinou bem hoje. Os niveis de forma estao melhorando." },
      { title: "Relatorio do Olheiro", text: "Nossos olheiros estao monitorando varios jogadores promissores na regiao." },
      { title: "Humor da Torcida", text: "A torcida esta animada com os proximos jogos." },
      { title: "Coletiva de Imprensa", text: "A imprensa pergunta sobre seus planos para a janela de transferencias." },
      { title: "Reuniao da Diretoria", text: "A diretoria revisou a situacao financeira do clube. Tudo estavel." },
      { title: "Base do Clube", text: "A base esta produzindo alguns talentos interessantes." },
      { title: "Noticias do Estadio", text: "A manutencao do estadio esta em dia. A capacidade permanece inalterada." },
      { title: "Boletim Medico", text: "O departamento medico nao reporta novas lesoes do treino de hoje." },
      { title: "Analise Tatica", text: "Seu assistente preparou uma analise do proximo adversario." },
      { title: "Rumor de Transferencia", text: "Rumores indicam que um clube rival esta interessado em um de seus jogadores." },
      { title: "Evento Comunitario", text: "O clube organizou um evento comunitario. O engajamento da torcida cresce." },
      { title: "Previsao do Tempo", text: "Bom tempo esperado para o proximo dia de jogos." },
    ],
    'es': [
      { title: "Informe de Entrenamiento", text: "La plantilla entreno bien hoy. Los niveles de forma estan mejorando." },
      { title: "Informe del Ojeador", text: "Nuestros ojeadores estan monitoreando varios jugadores prometedores en la region." },
      { title: "Animo de la Aficion", text: "La aficion esta emocionada con los proximos partidos." },
      { title: "Rueda de Prensa", text: "La prensa pregunta sobre sus planes para la ventana de fichajes." },
      { title: "Reunion de la Directiva", text: "La directiva reviso la situacion financiera del club. Todo estable." },
      { title: "Cantera", text: "La cantera esta produciendo algunos talentos interesantes." },
      { title: "Noticias del Estadio", text: "El mantenimiento del estadio esta al dia." },
      { title: "Parte Medico", text: "El departamento medico no reporta nuevas lesiones del entrenamiento de hoy." },
      { title: "Analisis Tactico", text: "Su asistente preparo un analisis del proximo rival." },
      { title: "Rumor de Fichaje", text: "Rumores sugieren que un club rival esta interesado en uno de sus jugadores." },
      { title: "Evento Comunitario", text: "El club organizo un evento comunitario. La participacion de los aficionados crece." },
      { title: "Pronostico del Tiempo", text: "Buen tiempo esperado para el proximo dia de partidos." },
    ],
    'fr': [
      { title: "Rapport d'Entrainement", text: "L'equipe s'est bien entrainee aujourd'hui. La forme s'ameliore." },
      { title: "Rapport du Recruteur", text: "Nos recruteurs surveillent plusieurs joueurs prometteurs dans la region." },
      { title: "Moral des Supporters", text: "Les supporters sont enthousiasmes par les prochains matchs." },
      { title: "Conference de Presse", text: "La presse s'interroge sur vos plans pour le mercato." },
      { title: "Reunion du Conseil", text: "Le conseil a examine la situation financiere du club. Tout est stable." },
      { title: "Centre de Formation", text: "Le centre de formation produit quelques talents interessants." },
      { title: "Nouvelles du Stade", text: "L'entretien du stade est en bonne voie." },
      { title: "Bulletin Medical", text: "Le staff medical ne signale aucune nouvelle blessure apres l'entrainement." },
      { title: "Analyse Tactique", text: "Votre adjoint a prepare une analyse du prochain adversaire." },
      { title: "Rumeur de Transfert", text: "Des rumeurs suggerent qu'un club rival est interesse par l'un de vos joueurs." },
      { title: "Evenement Communautaire", text: "Le club a organise un evenement communautaire." },
      { title: "Previsions Meteo", text: "Beau temps prevu pour le prochain jour de match." },
    ],
  },

  // ─── Match Day Messages ───────────────────────────────────────────────
  match_day: {
    'en': [
      { title: "Match Day!", text: "Today is match day! {club} faces {opponent} in the {competition}." },
      { title: "Kick Off Soon", text: "The stadium is filling up. {club} vs {opponent} kicks off shortly." },
    ],
    'pt-BR': [
      { title: "Dia de Jogo!", text: "Hoje e dia de jogo! {club} enfrenta {opponent} pelo {competition}." },
      { title: "Inicio em Breve", text: "O estadio esta lotando. {club} x {opponent} comeca em breve." },
    ],
    'es': [
      { title: "Dia de Partido!", text: "Hoy es dia de partido! {club} enfrenta a {opponent} en la {competition}." },
      { title: "Inicio Pronto", text: "El estadio se esta llenando. {club} vs {opponent} comienza pronto." },
    ],
    'fr': [
      { title: "Jour de Match!", text: "Aujourd'hui c'est jour de match! {club} affronte {opponent} en {competition}." },
      { title: "Coup d'Envoi Imminent", text: "Le stade se remplit. {club} vs {opponent} va bientot commencer." },
    ],
  },

  // ─── Round Results Summary ────────────────────────────────────────────
  round_results: {
    'en': [
      { title: "Round Results", text: "Results from today's matches:\n{results}" },
    ],
    'pt-BR': [
      { title: "Resultados da Rodada", text: "Resultados dos jogos de hoje:\n{results}" },
    ],
    'es': [
      { title: "Resultados de la Jornada", text: "Resultados de los partidos de hoy:\n{results}" },
    ],
    'fr': [
      { title: "Resultats de la Journee", text: "Resultats des matchs d'aujourd'hui:\n{results}" },
    ],
  },

  // ─── Board Messages ───────────────────────────────────────────────────
  board: {
    'en': [
      { title: "Board Expectations", text: "The board expects a top-half finish this season." },
      { title: "Board Confidence", text: "The board is satisfied with recent results. Keep it up!" },
      { title: "Budget Update", text: "The board has reviewed the transfer budget. No changes at this time." },
    ],
    'pt-BR': [
      { title: "Expectativas da Diretoria", text: "A diretoria espera uma classificacao na metade superior nesta temporada." },
      { title: "Confianca da Diretoria", text: "A diretoria esta satisfeita com os resultados recentes. Continue assim!" },
      { title: "Atualizacao de Orcamento", text: "A diretoria revisou o orcamento de transferencias. Sem alteracoes no momento." },
    ],
    'es': [
      { title: "Expectativas de la Directiva", text: "La directiva espera un final en la mitad superior esta temporada." },
      { title: "Confianza de la Directiva", text: "La directiva esta satisfecha con los resultados recientes. Siga asi!" },
      { title: "Actualizacion de Presupuesto", text: "La directiva reviso el presupuesto de fichajes. Sin cambios por ahora." },
    ],
    'fr': [
      { title: "Attentes du Conseil", text: "Le conseil attend une place en premiere moitie de classement cette saison." },
      { title: "Confiance du Conseil", text: "Le conseil est satisfait des resultats recents. Continuez!" },
      { title: "Mise a Jour Budget", text: "Le conseil a revu le budget transferts. Pas de changement pour le moment." },
    ],
  },

  // ─── Transfer Messages ────────────────────────────────────────────────
  transfer: {
    'en': [
      { title: "Transfer Interest", text: "A club has enquired about {player}. No formal bid yet." },
      { title: "Market Movement", text: "The transfer window is active. Several deals are being negotiated across the league." },
    ],
    'pt-BR': [
      { title: "Interesse em Transferencia", text: "Um clube perguntou sobre {player}. Nenhuma proposta formal ainda." },
      { title: "Movimentacao do Mercado", text: "A janela de transferencias esta ativa. Varios negocios estao sendo negociados na liga." },
    ],
    'es': [
      { title: "Interes de Fichaje", text: "Un club ha preguntado por {player}. Sin oferta formal aun." },
      { title: "Movimiento del Mercado", text: "La ventana de fichajes esta activa. Varios acuerdos se estan negociando." },
    ],
    'fr': [
      { title: "Interet pour un Transfert", text: "Un club s'est renseigne sur {player}. Pas d'offre formelle pour le moment." },
      { title: "Mouvement du Marche", text: "Le mercato est actif. Plusieurs transferts sont en negociation." },
    ],
  },
};

// ─── Helper Functions ─────────────────────────────────────────────────────

/**
 * Get a random message from a category for the given language.
 * Falls back to 'en' if language not found.
 */
function getRandomMessage(category, lang, placeholders = {}) {
  const cat = MESSAGES_DB[category];
  if (!cat) return null;

  const messages = cat[lang] || cat['en'];
  if (!messages || messages.length === 0) return null;

  const template = messages[Math.floor(Math.random() * messages.length)];
  return {
    title: replacePlaceholders(template.title, placeholders),
    text: replacePlaceholders(template.text, placeholders),
  };
}

/**
 * Get ALL messages from a category (for displaying lists).
 */
function getMessagesForCategory(category, lang) {
  const cat = MESSAGES_DB[category];
  if (!cat) return [];
  return cat[lang] || cat['en'] || [];
}

/**
 * Replace {placeholders} in a string.
 */
function replacePlaceholders(str, data) {
  return str.replace(/\{(\w+)\}/g, (match, key) => data[key] || match);
}

export { MESSAGES_DB, getRandomMessage, getMessagesForCategory, replacePlaceholders };
export default MESSAGES_DB;
