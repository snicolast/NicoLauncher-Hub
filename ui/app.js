// NicoLauncher interface. Everything that touches the disk or the network is
// in the Rust side (src-tauri, crates/hub-core); this file only shows state
// and sends the user's choices.
'use strict';

const T = window.__TAURI__;
const invoke = T.core.invoke;

const STR = {
  en: {
    nav_library: 'Library', nav_settings: 'Settings', nav_credits: 'Credits',
    by_author: 'by', refresh: 'Refresh', lib_sub: '{n} projects · {ready} ready to play',
    first_run_title: 'Welcome', about: 'About', screenshots: 'Screenshots', language_help: 'The language of the launcher.',
    credits_projects: 'Projects', show_log: 'Show log', hide_log: 'Hide log', installed_version: 'Installed news',
    first_run: 'Choose the folder where projects will be installed. Each one gets a folder of its own inside it.',
    root_gone_title: 'The install folder is not there', root_gone: '{path} was not found: it was deleted or moved, or its drive is not connected. Choose the folder where projects will be installed.',
    choose_folder: 'Choose folder',
    st_not_installed: 'Not installed', st_needs_input: 'Needs your own file',
    st_needs_build: 'Ready to build', st_ready: 'Ready', st_busy: 'Working...',
    update_available: 'Update available',
    do_install: 'Install', do_choose: 'Choose {file}', choose_title: 'Choose your own copy of {file}',
    ask_title: '{name} needs your own file',
    ask_lead: 'The game is built on this PC from your own copy of {file}. This is what happens next:',
    ask_1: 'A window opens to find {file} on your disk. Choose it.',
    ask_2: 'The launcher reads it and builds the game. It takes several minutes, and the first time it also downloads the build tools.',
    ask_3: 'When it ends, the button turns into Play.',
    ask_go: 'Choose the file', ask_later: 'Not now', ask_cancel: 'Cancel the install', install_cancelled: 'Install of {name} cancelled. What had been downloaded was removed.', do_build: 'Build', do_play: 'Play',
    do_update: 'Update', do_rebuild: 'Rebuild', do_change_file: 'Change file',
    do_open_folder: 'Open folder', do_shortcut: 'Desktop shortcut', shortcut_done: 'Shortcut to {name} added to your desktop.',
    installed_title: 'All installed!', installed_body: 'You have just built {name} on your own PC, and it is ready to play. Press Play whenever you want.',
    installed_ask: 'Do you also want a shortcut on your desktop?', shortcut_yes: 'Yes, add it', shortcut_no: 'No, thanks', do_github: 'View on GitHub', do_existing: 'Use a folder I already have',
    details: 'Details', folder: 'Folder', your_file: 'Your file', source: 'Source',
    not_chosen: 'not chosen', not_found: 'not found', not_installed_yet: 'not installed',
    news: 'What is new', news_new: 'New', no_news: 'No news yet.',
    input_note: 'The launcher never downloads this file. It only reads the copy you choose from your own disk.',
    install_folder: 'Install folder',
    install_folder_help: 'Projects already installed stay where they are if you change it.',
    language: 'Language',
    self_update: 'Update to v{v}', self_updating: 'Updating the launcher...',
    license_title: 'License', license_agreement: 'License agreement', license_read: 'Read the license',
    license_credits: 'NicoLauncher is free and open source, under the MIT License with a few additional terms: keep the credit to snicolast, do not sell it, and never use it to share built games or ROMs.',
    license_intro: 'Before you start, please read and accept the license. In short:',
    license_point_free: 'The launcher is free and not for sale. You may use it, share it and change it, keeping the credit to snicolast.',
    license_point_games: 'It never downloads or shares built games or ROMs, and it must not be changed to do so. You use your own copy of each game.',
    license_point_warranty: 'It is an unofficial fan project, provided as is: no warranty, and no obligation of support or updates.',
    license_full: 'Full text', license_accept: 'I accept', license_scroll: 'Scroll to the end of the text to accept.', license_decline: 'Exit',
    credits_author: 'NicoLauncher and the projects it lists are made by snicolast.',
    credits_rules_title: 'What this launcher does not do',
    credits_rule_roms: 'It does not include, download or link to game ROMs. You use your own copy.',
    credits_rule_builds: 'It does not download or share built games. Each project is built on your PC from its source code.',
    credits_rule_affiliation: 'These are unofficial fan projects, not affiliated with the owners of the original games.',
    support_title: 'Buy me a coffee',
    support_text: 'The launcher and every project are free. A coffee is an optional thank-you for the time behind them, and it unlocks nothing.',
    copy_log: 'Copy log', cancel: 'Cancel', close: 'Close', copied: 'Copied',
    task_running: 'running', task_ok: 'finished', task_failed: 'failed',
    task_setup: 'Installing {name}', task_build: 'Building {name}', task_update: 'Updating {name}',
    do_uninstall: 'Uninstall', un_title: 'Uninstall {name}?',
    un_delete: 'This folder is deleted with everything in it, including your saved games and settings of the game:',
    un_rom_out: 'Your {file} is not touched: it stays where you keep it.',
    un_rom_in: 'Careful: your {file} is inside that folder and would be deleted too. Move it somewhere else first if you want to keep it.',
    un_again: 'You can install it again whenever you want.',
    un_clone: 'This folder is your own copy of the project, so the launcher does not delete it. It only stops using it:',
    un_go: 'Delete and uninstall', un_go_clone: 'Stop using it', un_done: '{name} was uninstalled.',
    ph_source: 'Downloading the source code', ph_unpack: 'Unpacking what was downloaded',
    ph_tools: 'Getting the tools to build the game (only the first time)', ph_libs: 'Downloading a few extra pieces',
    ph_rom: 'Reading your game file', ph_recomp: 'Translating the original game for PC',
    ph_configure: 'Preparing the build', ph_compile: 'Building the game. This is the long part: it takes a few minutes',
    ph_place: 'Putting the game in its place', ph_tests: 'Checking that everything came out right',
    ph_check_update: 'Looking for the new version', ph_pc_tools: 'Installing what this PC needs to build the game',
    nav_home: 'Home', home_news: 'Latest news', side_games: 'Games', details: 'Details',
    home_more: 'Show earlier news', home_library: 'Your library', home_open_library: 'Open Library', home_video: 'Latest video on snicolast', home_video_watch: 'Watch on YouTube',
    count_ready: '{n} ready to play', count_updates: '{n} with something new', count_todo: '{n} still to set up', count_missing: '{n} not installed',
    home_empty: 'No news could be loaded. Check your connection and press Refresh.',
    today: 'Today', yesterday: 'Yesterday',
    settings_damaged: 'The settings file could not be read:',
  },
  es: {
    nav_library: 'Biblioteca', nav_settings: 'Ajustes', nav_credits: 'Créditos',
    by_author: 'por', refresh: 'Actualizar lista', lib_sub: '{n} proyectos · {ready} listos para jugar',
    first_run_title: 'Bienvenido', about: 'Acerca de', screenshots: 'Capturas', language_help: 'El idioma del launcher.',
    credits_projects: 'Proyectos', show_log: 'Ver log', hide_log: 'Ocultar log', installed_version: 'Novedades instaladas',
    first_run: 'Elige la carpeta donde se instalarán los proyectos. Cada uno tendrá su propia carpeta dentro.',
    root_gone_title: 'La carpeta de instalación no está', root_gone: 'No se encontró {path}: se borró o se movió, o su unidad no está conectada. Elige la carpeta donde se instalarán los proyectos.',
    choose_folder: 'Elegir carpeta',
    st_not_installed: 'No instalado', st_needs_input: 'Falta tu archivo',
    st_needs_build: 'Listo para compilar', st_ready: 'Listo', st_busy: 'Trabajando...',
    update_available: 'Hay novedades',
    do_install: 'Instalar', do_choose: 'Elegir {file}', choose_title: 'Elige tu propia copia de {file}',
    ask_title: '{name} necesita tu propio archivo',
    ask_lead: 'El juego se compila en este PC a partir de tu propia copia de {file}. Esto es lo que pasa ahora:',
    ask_1: 'Se abre una ventana para buscar {file} en tu disco. Elígelo.',
    ask_2: 'El launcher lo lee y compila el juego. Tarda varios minutos, y la primera vez además descarga las herramientas de compilación.',
    ask_3: 'Cuando termine, el botón pasa a ser Jugar.',
    ask_go: 'Elegir el archivo', ask_later: 'Ahora no', ask_cancel: 'Cancelar la instalación', install_cancelled: 'Instalación de {name} cancelada. Se ha borrado lo que se había descargado.', do_build: 'Compilar', do_play: 'Jugar',
    do_update: 'Actualizar', do_rebuild: 'Recompilar', do_change_file: 'Cambiar archivo',
    do_open_folder: 'Abrir carpeta', do_shortcut: 'Acceso directo', shortcut_done: 'Acceso directo a {name} añadido a tu escritorio.',
    installed_title: '¡Todo instalado!', installed_body: 'Acabas de montar {name} en tu propio PC, y ya está listo para jugar. Pulsa Jugar cuando quieras.',
    installed_ask: '¿Quieres también un acceso directo en el escritorio?', shortcut_yes: 'Sí, añádelo', shortcut_no: 'No, gracias', do_github: 'Ver en GitHub', do_existing: 'Usar una carpeta que ya tengo',
    details: 'Detalles', folder: 'Carpeta', your_file: 'Tu archivo', source: 'Código',
    not_chosen: 'sin elegir', not_found: 'no encontrada', not_installed_yet: 'sin instalar',
    news: 'Novedades', news_new: 'Nuevo', no_news: 'Aún no hay novedades.',
    input_note: 'El launcher nunca descarga este archivo. Solo lee la copia que elijas de tu propio disco.',
    install_folder: 'Carpeta de instalación',
    install_folder_help: 'Los proyectos ya instalados se quedan donde están si la cambias.',
    language: 'Idioma',
    self_update: 'Actualizar a v{v}', self_updating: 'Actualizando el launcher...',
    license_title: 'Licencia', license_agreement: 'Contrato de licencia', license_read: 'Leer la licencia',
    license_credits: 'NicoLauncher es gratuito y de código abierto, bajo la licencia MIT con unas cláusulas adicionales: mantener el crédito a snicolast, no venderlo y no usarlo nunca para compartir juegos compilados ni ROMs.',
    license_intro: 'Antes de empezar, lee y acepta la licencia. En resumen:',
    license_point_free: 'El launcher es gratuito y no se puede vender. Puedes usarlo, compartirlo y modificarlo, manteniendo el crédito a snicolast.',
    license_point_games: 'Nunca descarga ni comparte juegos compilados ni ROMs, y no se puede modificar para que lo haga. Usas tu propia copia de cada juego.',
    license_point_warranty: 'Es un proyecto de fans no oficial, que se ofrece tal cual: sin garantía y sin obligación de soporte ni de actualizaciones.',
    license_full: 'Texto completo (en inglés, que es el que vale)', license_accept: 'Acepto', license_scroll: 'Baja hasta el final del texto para poder aceptar.', license_decline: 'Salir',
    credits_author: 'NicoLauncher y los proyectos que lista son obra de snicolast.',
    credits_rules_title: 'Lo que este launcher no hace',
    credits_rule_roms: 'No incluye, descarga ni enlaza ROMs de juegos. Usas tu propia copia.',
    credits_rule_builds: 'No descarga ni comparte juegos compilados. Cada proyecto se compila en tu PC desde su código fuente.',
    credits_rule_affiliation: 'Son proyectos de fans no oficiales, sin relación con los dueños de los juegos originales.',
    support_title: 'Invítame a un café',
    support_text: 'El launcher y todos los proyectos son gratis. El café es una forma opcional de dar las gracias por el tiempo que llevan, y no desbloquea nada.',
    copy_log: 'Copiar log', cancel: 'Cancelar', close: 'Cerrar', copied: 'Copiado',
    task_running: 'en curso', task_ok: 'terminado', task_failed: 'con errores',
    task_setup: 'Instalando {name}', task_build: 'Compilando {name}', task_update: 'Actualizando {name}',
    do_uninstall: 'Desinstalar', un_title: '¿Desinstalar {name}?',
    un_delete: 'Se borra esta carpeta con todo lo que tiene dentro, también tus partidas guardadas y los ajustes del juego:',
    un_rom_out: 'Tu {file} no se toca: se queda donde lo tienes.',
    un_rom_in: 'Cuidado: tu {file} está dentro de esa carpeta y también se borraría. Muévelo antes a otro sitio si quieres conservarlo.',
    un_again: 'Puedes volver a instalarlo cuando quieras.',
    un_clone: 'Esta carpeta es tu propia copia del proyecto, así que el launcher no la borra. Solo deja de usarla:',
    un_go: 'Borrar y desinstalar', un_go_clone: 'Dejar de usarla', un_done: '{name} se ha desinstalado.',
    ph_source: 'Descargando el código fuente', ph_unpack: 'Descomprimiendo lo descargado',
    ph_tools: 'Bajando las herramientas para montar el juego (solo la primera vez)', ph_libs: 'Descargando unas piezas más',
    ph_rom: 'Leyendo tu archivo del juego', ph_recomp: 'Traduciendo el juego original para PC',
    ph_configure: 'Preparando el montaje', ph_compile: 'Montando el juego. Es la parte larga: tarda unos minutos',
    ph_place: 'Colocando el juego en su sitio', ph_tests: 'Comprobando que todo ha salido bien',
    ph_check_update: 'Buscando la versión nueva', ph_pc_tools: 'Instalando lo que este PC necesita para montar el juego',
    nav_home: 'Inicio', home_news: 'Últimas noticias', side_games: 'Juegos', details: 'Ver detalles',
    home_more: 'Ver noticias anteriores', home_library: 'Tu biblioteca', home_open_library: 'Abrir la Biblioteca', home_video: 'Último vídeo en snicolast', home_video_watch: 'Ver en YouTube',
    count_ready: '{n} listos para jugar', count_updates: '{n} con novedades', count_todo: '{n} a medio preparar', count_missing: '{n} sin instalar',
    home_empty: 'No se han podido cargar las noticias. Comprueba la conexión y pulsa Actualizar lista.',
    today: 'Hoy', yesterday: 'Ayer',
    settings_damaged: 'No se pudo leer el archivo de ajustes:',
  },
};

let snap = null;          // last snapshot from the Rust side
let lang = 'en';
let view = 'home';
let current = null;       // project id shown in the project view
let task = null;          // { project, name, action, done, ok, step, total, label, started }
let logOpen = false;
let timer = null;

const ICONS = {
  home: '<path d="M4 11l8-7 8 7v8a1 1 0 0 1-1 1h-4v-6h-6v6H5a1 1 0 0 1-1-1z"/>',
  grid: '<rect x="3" y="3" width="7" height="7" rx="1.5"/><rect x="14" y="3" width="7" height="7" rx="1.5"/><rect x="3" y="14" width="7" height="7" rx="1.5"/><rect x="14" y="14" width="7" height="7" rx="1.5"/>',
  sliders: '<path d="M4 6h10M18 6h2M4 12h4M12 12h8M4 18h12M20 18h0"/><circle cx="16" cy="6" r="2"/><circle cx="10" cy="12" r="2"/><circle cx="18" cy="18" r="2"/>',
  heart: '<path d="M12 20s-7-4.4-7-10a4 4 0 0 1 7-2.6A4 4 0 0 1 19 10c0 5.600-7 10-7 10z"/>',
  refresh: '<path d="M20 11a8 8 0 1 0-2.3 5.700"/><path d="M20 4v7h-7"/>',
  folder: '<path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>',
  back: '<path d="M15 5l-7 7 7 7"/>',
  github: '<path d="M9 19c-4 1.300-4-2-6-2.500M15 22v-3.500a3 3 0 0 0-.8-2.300c2.800-.3 5.800-1.400 5.800-6.200a4.800 4.800 0 0 0-1.300-3.300 4.500 4.500 0 0 0-.1-3.300s-1.100-.3-3.500 1.300a12 12 0 0 0-6.200 0C6.500 1.100 5.400 1.400 5.400 1.400a4.500 4.500 0 0 0-.1 3.300A4.800 4.800 0 0 0 4 8c0 4.800 3 5.900 5.800 6.200A3 3 0 0 0 9 16.500V22"/>',
  shield: '<path d="M12 3l8 3v6c0 5-3.500 8-8 9-4.500-1-8-4-8-9V6z"/><path d="M9 12l2 2 4-4"/>',
  coffee: '<path d="M5 9h11v5a5 5 0 0 1-5 5h-1a5 5 0 0 1-5-5z"/><path d="M16 10h1.500a2.500 2.500 0 0 1 0 5H16"/><path d="M8 3v2.500M11.500 3v2.500"/>',
  youtube: '<rect x="2.5" y="5.5" width="19" height="13" rx="4"/><path d="M10 9.200v5.600l5-2.800z" fill="currentColor" stroke="none"/>',
  play: '<path d="M7 4.500v15l13-7.500z" fill="currentColor" stroke="none"/>',
  download: '<path d="M12 4v11M7 11l5 5 5-5M5 20h14"/>',
  file: '<path d="M14 3H7a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V8z"/><path d="M14 3v5h5"/>',
  hammer: '<path d="M14 6l4 4M3 21l9-9M12 4l8 8-3 3-8-8z"/>',
  up: '<path d="M12 20V8M7 13l5-5 5 5M5 4h14"/>',
  trash: '<path d="M4 7h16M10 11v6M14 11v6M6 7l1 13h10l1-13M9 7V4h6v3"/>',
};
const icon = (name) => {
  const s = document.createElement('span');
  s.className = 'ico';
  s.innerHTML = `<svg viewBox="0 0 24 24">${ICONS[name]}</svg>`;
  return s;
};

const $ = (id) => document.getElementById(id);
const t = (key, vars = {}) =>
  (STR[lang][key] ?? STR.en[key] ?? key).replace(/\{(\w+)\}/g, (_, k) => vars[k] ?? '');

function el(tag, props = {}, ...children) {
  const node = document.createElement(tag);
  for (const [k, v] of Object.entries(props)) {
    if (k === 'class') node.className = v;
    else if (k === 'onclick') node.addEventListener('click', v);
    else if (v !== false && v != null) node.setAttribute(k, v === true ? '' : v);
  }
  for (const c of children) if (c != null) node.append(c);
  return node;
}

// The project's own pictures come from its repository on GitHub. Until one
// loads (or with no connection) a gradient made from its id stands in.
function gradient(p) {
  let h = 0;
  for (const ch of p.id) h = (h * 31 + ch.charCodeAt(0)) % 360;
  return `linear-gradient(135deg, hsl(${h} 50% 26%), hsl(${(h + 50) % 360} 55% 12%))`;
}
function picture(src, cls) {
  const img = el('img', { src, alt: '', class: cls || '', draggable: 'false' });
  img.addEventListener('load', () => img.classList.add('in'));
  img.addEventListener('error', () => img.remove());
  return img;
}

// A message in the corner: something went wrong, or (good) something the
// user asked for is done
function toast(text, good) {
  const node = el('div', { class: 'toast' + (good ? ' good' : ''), onclick: () => node.remove() }, text);
  $('toasts').append(node);
  setTimeout(() => node.remove(), 9000);
}

async function call(cmd, args) {
  try {
    return await invoke(cmd, args);
  } catch (e) {
    toast(String(e));
    return undefined;
  }
}

async function update(cmd, args) {
  const s = await call(cmd, args);
  if (s) { snap = s; render(); }
  return s;
}

async function pickInstallRoot() {
  const path = await T.dialog.open({ directory: true, multiple: false });
  if (!path) return false;
  return !!(await update('set_install_root', { path }));
}

function newTask(p, action) {
  task = { project: p.id, name: p.name, action, done: false, ok: false, step: 0, total: 0, label: '', started: Date.now() };
  $('dock-log').textContent = '';
  clearInterval(timer);
  timer = setInterval(renderDock, 1000);
}

async function startTask(p, action) {
  if (action === 'setup' && !snap.install_root && !p.dir) {
    if (!(await pickInstallRoot())) return;
  }
  newTask(p, action);
  task.own = true;
  task.fresh = action === 'setup' && p.state === 'not_installed';
  // Not playable before this task: if it ends playable, the install is over
  task.wasReady = p.state === 'ready';
  const started = await call('start_task', { id: p.id, action });
  if (started === undefined) { task = null; clearInterval(timer); }
  await update('snapshot');
}

// The file dialog for a project's own file: titled with what is asked for and
// showing that kind of file first ("swtrilgy.zip (Rev. A)" -> .zip)
async function addShortcut(p) {
  if ((await call('desktop_shortcut', { id: p.id })) !== undefined) toast(t('shortcut_done', { name: p.name }), true);
}

async function celebrate(p) {
  const yes = await ask({
    title: t('installed_title'),
    body: [el('p', {}, t('installed_body', { name: p.name })), el('p', { class: 'dim' }, t('installed_ask'))],
    go: [t('shortcut_yes')], later: t('shortcut_no'),
  });
  if (yes) addShortcut(p);
}

// A question over the whole window. Resolves to true when the user goes on;
// Escape and the quiet button resolve to false.
let closeAsk = null;
function ask({ title, body, go, later, danger }) {
  return new Promise((resolve) => {
    const done = (yes) => { closeAsk = null; box.remove(); resolve(yes); };
    const yes = el('button', { class: 'btn ' + (danger ? 'danger' : 'primary'), onclick: () => done(true) }, ...go);
    const no = el('button', { class: 'btn ghost', onclick: () => done(false) }, later);
    const box = el('div', { class: 'modal', role: 'dialog', 'aria-modal': 'true' },
      el('div', { class: 'modal-box' }, el('h2', {}, title), ...body, el('div', { class: 'modal-actions' }, no, yes)));
    closeAsk = () => done(false);
    document.body.append(box);
    // What deletes is never one Enter away
    (danger ? no : yes).focus();
  });
}

// The license over the whole window. On first run it cannot be skipped: the
// user accepts it or the launcher closes. From Credits it is only read.
let licenseBox = null;
async function showLicense(mustAccept) {
  if (licenseBox) licenseBox.remove();
  const text = (await call('license_text')) || '';
  const close = () => { licenseBox.remove(); licenseBox = null; };
  const actions = mustAccept
    ? [el('button', { class: 'btn ghost', onclick: () => invoke('quit') }, t('license_decline')),
       el('button', { class: 'btn primary', disabled: true, onclick: async () => { if (await update('accept_license')) close(); } }, t('license_accept'))]
    : [el('button', { class: 'btn primary', onclick: close }, t('close'))];
  licenseBox = el('div', { class: 'modal license', role: 'dialog', 'aria-modal': 'true' },
    el('div', { class: 'modal-box' },
      el('h2', {}, t('license_agreement')),
      mustAccept ? el('p', { class: 'dim' }, t('license_intro')) : null,
      el('ul', { class: 'rules' }, ...['free', 'games', 'warranty'].map((k) =>
        el('li', {}, icon('shield'), el('span', {}, t('license_point_' + k))))),
      el('h3', {}, t('license_full')),
      el('pre', { class: 'license-text', tabindex: '0' }, text),
      mustAccept ? el('p', { class: 'dim small license-hint' }, t('license_scroll')) : null,
      el('div', { class: 'modal-actions' }, ...actions)));
  licenseBox.dataset.must = mustAccept ? '1' : '';
  document.body.append(licenseBox);
  const pre = licenseBox.querySelector('.license-text');
  const accept = licenseBox.querySelector('.modal-actions .btn.primary');
  if (mustAccept) {
    // "I accept" lights up once the text has been scrolled to its end (or
    // when it fits whole and there is nothing to scroll)
    const read = () => {
      if (pre.scrollTop + pre.clientHeight < pre.scrollHeight - 8) return;
      accept.disabled = false;
      const hint = licenseBox.querySelector('.license-hint');
      if (hint) hint.remove();
    };
    pre.addEventListener('scroll', read);
    read();
    pre.focus();
  } else accept.focus();
}

// Before the file dialog of an install: what the file is for and what
// happens after it, so the dialog does not come out of nowhere
function explainInput(p, fresh) {
  const file = { file: p.input_label };
  return ask({
    title: t('ask_title', { name: p.name }),
    body: [
      el('p', { class: 'dim' }, t('ask_lead', file)),
      el('ol', {}, el('li', {}, t('ask_1', file)), el('li', {}, t('ask_2')), el('li', {}, t('ask_3'))),
      el('div', { class: 'note' }, icon('shield'), t('input_note')),
    ],
    go: [icon('file'), t('ask_go')],
    later: t(fresh ? 'ask_cancel' : 'ask_later'),
  });
}

// Uninstall: says exactly which folder goes and what happens to the user's
// own file before anything is deleted
async function uninstall(p) {
  const file = { file: p.input_label };
  const inside = !!p.input && p.input.toLowerCase().startsWith(p.dir.toLowerCase().replace(/[\\/]*$/, '\\'));
  const body = [el('p', { class: 'dim' }, t(p.is_clone ? 'un_clone' : 'un_delete')), el('p', { class: 'path' }, p.dir)];
  if (!p.is_clone) {
    if (p.input_label) body.push(el('div', { class: 'note' + (inside ? ' warn' : '') }, icon('shield'), t(inside ? 'un_rom_in' : 'un_rom_out', file)));
    body.push(el('p', { class: 'dim' }, t('un_again')));
  }
  const yes = await ask({
    title: t('un_title', { name: p.name }),
    body,
    go: [icon('trash'), t(p.is_clone ? 'un_go_clone' : 'un_go')],
    later: t('cancel'),
    danger: !p.is_clone,
  });
  if (!yes) return;
  if (await update('uninstall', { id: p.id })) toast(t('un_done', { name: p.name }), true);
  else await update('snapshot');
}

// Backing out of an install before the user's file was chosen: what was
// downloaded for it is taken off the disk again, so nothing is left half done.
// Only for an install this window has just started; the launcher's own rules
// for deleting (never somebody's own clone, never another folder) still hold.
async function abandon(p) {
  try {
    snap = await invoke('uninstall', { id: p.id });
    render();
    toast(t('install_cancelled', { name: p.name }), true);
  } catch (e) { await update('snapshot'); }
}

async function chooseInput(p, thenBuild, fresh) {
  if (thenBuild && !(await explainInput(p, fresh))) { if (fresh) await abandon(p); return; }
  const ext = (/\.([a-z0-9]+)\b/i.exec(p.input_label || '') || [])[1];
  const filters = ext ? [{ name: p.input_label, extensions: [ext.toLowerCase()] }, { name: '*', extensions: ['*'] }] : undefined;
  const path = await T.dialog.open({ directory: false, multiple: false, title: t('choose_title', { file: p.input_label }), filters });
  if (!path) { if (fresh) await abandon(p); return; }
  const s = await update('set_input', { id: p.id, path });
  if (s && thenBuild) {
    const now = s.projects.find((x) => x.id === p.id);
    if (now && now.state === 'needs_build') startTask(now, 'setup');
  }
}

async function chooseExisting(p) {
  const path = await T.dialog.open({ directory: true, multiple: false });
  if (path) update('set_project_dir', { id: p.id, path });
}

function primaryButton(p, large) {
  const busy = !!snap.busy;
  const make = (ic, label, fn, cls = 'primary') =>
    el('button', { class: `btn ${cls}${large ? ' lg' : ''}`, disabled: busy, onclick: (e) => { e.stopPropagation(); fn(); } }, icon(ic), label);
  switch (p.state) {
    case 'not_installed': return make('download', t('do_install'), () => startTask(p, 'setup'));
    case 'needs_input': return make('file', t('do_choose', { file: p.input_label }), () => chooseInput(p, true));
    case 'needs_build': return make('hammer', t('do_build'), () => startTask(p, 'setup'));
    default: return make('play', t('do_play'), () => call('launch', { id: p.id }), 'play');
  }
}

function stateLine(p) {
  const working = snap.busy === p.id;
  return [el('span', { class: 'dot ' + (working ? 'busy' : p.state) }), working ? t('st_busy') : t('st_' + p.state)];
}

// Home: everything that is new, from the launcher itself and from every
// project, as one page to read. A date is shown once, with what each
// project brought that day under it.
// A news title is usually its date, written by hand: 2026-11-12 in some
// projects, "12 November 2026" in others. As YYYY-MM-DD, or '' when the title
// is something else
const MONTHS = ['jan', 'feb', 'mar', 'apr', 'may', 'jun', 'jul', 'aug', 'sep', 'oct', 'nov', 'dec'];
function dayKey(title) {
  const text = title.trim();
  if (/^\d{4}-\d{2}-\d{2}$/.test(text)) return text;
  const two = (n) => String(n).padStart(2, '0');
  let m = /^(\d{1,2})(?:st|nd|rd|th)?\s+([A-Za-z]+)\.?,?\s+(\d{4})$/.exec(text);
  if (m) m = [m[1], m[2], m[3]];
  else if ((m = /^([A-Za-z]+)\.?\s+(\d{1,2})(?:st|nd|rd|th)?,?\s+(\d{4})$/.exec(text))) m = [m[2], m[1], m[3]];
  if (!m) return '';
  const month = MONTHS.indexOf(m[1].slice(0, 3).toLowerCase());
  return month < 0 || +m[0] < 1 || +m[0] > 31 ? '' : `${m[2]}-${two(month + 1)}-${two(m[0])}`;
}

function dayLabel(title) {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(dayKey(title));
  if (!m) return title;
  const day = new Date(+m[1], +m[2] - 1, +m[3]);
  const now = new Date();
  const ago = Math.round((new Date(now.getFullYear(), now.getMonth(), now.getDate()) - day) / 86400000);
  if (ago === 0) return t('today');
  if (ago === 1) return t('yesterday');
  const opts = { weekday: 'long', day: 'numeric', month: 'long' };
  if (day.getFullYear() !== now.getFullYear()) opts.year = 'numeric';
  const text = day.toLocaleDateString(lang, opts);
  return text.charAt(0).toUpperCase() + text.slice(1);
}

// A picture that fills `box`, changed only when it is another one: the page
// is drawn again at every change of state, and the picture must not blink
// The new picture fades in over the one before it, which then goes away.
function backdrop(box, src) {
  if (box.dataset.src === (src || '')) return false;
  box.dataset.src = src || '';
  const old = [...box.children];
  if (src) box.append(picture(src));
  setTimeout(() => old.forEach((n) => n.remove()), src ? 1100 : 0);
  return true;
}

// The project Home puts forward: the one picked in its strip, or else the one
// with the newest news. Left alone, Home goes through all of them in turn
let spot = null;
// Days of news Home shows before "Show earlier news"
const HOME_DAYS = 3, HOME_DAYS_ALL = 10;
let homeDays = HOME_DAYS;
const SPOT_EVERY = 9000;
let spotTimer = null;
function spotNext() {
  const list = snap ? snap.projects : [];
  // Not while something else has the eye: another page, a question on screen,
  // the pointer on the spotlight, or the window out of sight
  if (list.length < 2 || view !== 'home' || document.hidden || document.querySelector('.modal') || $('view-home').querySelector('.spot:hover')) return;
  const at = list.findIndex((p) => p.id === $('spot-bg').dataset.id);
  spot = list[(at + 1) % list.length].id;
  renderHome();
}
function spotRestart() {
  clearInterval(spotTimer);
  spotTimer = setInterval(spotNext, SPOT_EVERY);
}
spotRestart();

function renderHome() {
  const items = [];
  for (const e of snap.launcher_news || []) items.push({ ...e, day: dayKey(e.title), order: -1, from: null });
  snap.projects.forEach((p, order) => {
    const fresh = new Set(p.new_entries.map((e) => e.number));
    for (const e of p.latest_news || []) items.push({ ...e, day: dayKey(e.title), order, from: p, isNew: fresh.has(e.number) });
  });
  // By day, newest first (a title that is not a date goes last); within a day
  // the launcher's own news go first, then the projects in their order
  items.sort((a, b) => (a.day < b.day ? 1 : a.day > b.day ? -1 : a.order - b.order || b.number - a.number));
  const days = [];
  for (const e of items) {
    let day = days[days.length - 1];
    const key = e.day || e.title;
    if (!day || day.key !== key) {
      if (days.length === homeDays + 1) break;
      days.push(day = { key, title: e.title, blocks: [] });
    }
    let block = day.blocks.find((b) => b.from === e.from);
    if (!block) day.blocks.push(block = { from: e.from, lines: [], isNew: false });
    block.lines.push(...e.lines);
    block.isNew = block.isNew || !!e.isNew;
  }
  // One day more than shown was gathered, to know whether there are earlier ones
  const earlier = days.length > homeDays;
  if (earlier) days.pop();
  $('home-more').hidden = !earlier || homeDays >= HOME_DAYS_ALL;

  // The spotlight: one project across the top, with its picture, what it
  // brought last and the one thing to do with it
  const newest = items.find((e) => e.from);
  const lead = snap.projects.find((p) => p.id === spot) || (newest && newest.from) || snap.projects[0];
  const open = (p) => (!snap.install_root && !p.dir ? show('library') : openProject(p.id));
  backdrop($('spot-bg'), lead && lead.hero);
  const swapped = $('spot-bg').dataset.id !== (lead ? lead.id : '');
  $('spot-bg').dataset.id = lead ? lead.id : '';
  $('spot-content').classList.remove('swap');
  if (swapped) { void $('spot-content').offsetWidth; $('spot-content').classList.add('swap'); }
  if (lead) {
    $('spot-bg').style.backgroundImage = gradient(lead);
    const last = items.find((e) => e.from === lead);
    $('spot-content').replaceChildren(
      el('div', { class: 'hero-logo' }, lead.logo ? picture(lead.logo) : el('h1', {}, lead.name)),
      el('p', { class: 'tagline' }, lead.tagline),
      last ? el('p', { class: 'spot-news' }, el('b', {}, dayLabel(last.title)), last.lines[0] || '') : null,
      el('div', { class: 'hero-actions' },
        !snap.install_root && !lead.dir
          ? el('button', { class: 'btn primary lg', onclick: () => show('library') }, icon('download'), t('do_install'))
          : primaryButton(lead, true),
        el('button', { class: 'btn lg glass', onclick: () => open(lead) }, t('details')),
        el('span', { class: 'state' }, ...stateLine(lead)),
        lead.new_entries.length ? el('span', { class: 'chip new' }, t('update_available')) : null));
  } else $('spot-content').replaceChildren();
  $('spot-picks').classList.toggle('dots', snap.projects.length > 5);
  $('spot-picks').replaceChildren(...(snap.projects.length > 1 ? snap.projects.map((p) =>
    el('button', { class: 'pick' + (p === lead ? ' on' : ''), title: p.name, style: `background-image:${gradient(p)}`, onclick: () => { spot = p.id; spotRestart(); render(); } },
      p.hero ? picture(p.hero) : null)) : []));

  const block = (b) => el('div', { class: 'feed-block' + (b.from ? ' link' : ''), onclick: b.from ? () => open(b.from) : null },
    el('div', { class: 'feed-from' },
      b.from && b.from.logo ? picture(b.from.logo, 'feed-logo') : el('span', { class: 'mark', 'aria-hidden': 'true' }),
      el('strong', {}, b.from ? b.from.name : 'NicoLauncher'),
      b.isNew ? el('span', { class: 'chip new' }, t('update_available')) : null),
    el('ul', {}, ...b.lines.map((l) => el('li', {}, l))));
  $('home-feed').replaceChildren(...(days.length
    ? days.map((d) => el('div', { class: 'entry' }, el('h3', {}, dayLabel(d.title)), ...d.blocks.map(block)))
    : [el('p', { class: 'dim' }, t('home_empty'))]));
  $('home-feed').classList.toggle('timeline', days.length > 0);

  // Beside the news: the pictures of the project in the spotlight (changed
  // only with it, so they do not blink), the library in numbers and the links
  let shots = lead ? (lead.screens && lead.screens.length ? lead.screens : lead.hero ? [lead.hero] : []).slice(0, 4) : [];
  if (shots.length === 3) shots.pop();
  if ($('home-shots').dataset.id !== (lead ? lead.id : '')) {
    $('home-shots').dataset.id = lead ? lead.id : '';
    $('shots-title').textContent = lead ? lead.name : '';
    $('home-shots').replaceChildren(...shots.map((src) => {
      const img = picture(src);
      img.addEventListener('click', () => { $('lightbox-img').src = src; $('lightbox').hidden = false; });
      return img;
    }));
  }
  const count = (state) => snap.projects.filter((p) => p.state === state).length;
  const fresh = snap.projects.filter((p) => p.new_entries.length && p.can_update);
  const todo = count('needs_input') + count('needs_build');
  const line = (dot, key, n) => (n ? el('div', { class: 'state' }, el('span', { class: 'dot ' + dot }), t(key, { n })) : null);
  $('home-library').replaceChildren(...[
    line('ready', 'count_ready', count('ready')),
    line('needs_build', 'count_todo', todo),
    line('', 'count_missing', count('not_installed')),
    ...fresh.map((p) => el('div', { class: 'fresh' },
      el('span', {}, el('strong', {}, p.name.replace(/ Native$/, '')), el('span', { class: 'chip new' }, t('update_available'))),
      el('button', { class: 'btn small', disabled: !!snap.busy, onclick: () => startTask(p, 'update') }, icon('up'), t('do_update')))),
    el('button', { class: 'btn ghost small', onclick: () => show('library') }, t('home_open_library')),
  ].filter(Boolean));

  // The author's newest video: rebuilt only when it is another one
  const video = snap.latest_video;
  $('home-video-panel').hidden = !video;
  if (video && $('home-video').dataset.url !== video.url) {
    $('home-video').dataset.url = video.url;
    $('home-video').title = t('home_video_watch');
    const img = el('img', { src: video.thumbnail, alt: '', draggable: 'false' });
    img.addEventListener('error', () => img.remove());
    $('home-video').replaceChildren(
      el('span', { class: 'video-art' }, img, el('span', { class: 'video-play' }, icon('play'))),
      el('span', { class: 'video-text' }, el('strong', {}, video.title),
        el('span', { class: 'dim small' }, [video.date && dayLabel(video.date), 'YouTube'].filter(Boolean).join(' · '))));
  }

  // The sidebar's shortcuts, and the picture far behind everything
  $('side-games').replaceChildren(...snap.projects.map((p) =>
    el('button', { class: 'side-game' + (view === 'project' && current === p.id ? ' on' : ''), title: p.name, onclick: () => open(p) },
      el('span', { class: 'side-art', style: `background-image:${gradient(p)}` }, p.hero ? picture(p.hero) : null),
      el('span', { class: 'side-name' }, p.name.replace(/ Native$/, '')),
      el('span', { class: 'dot ' + (snap.busy === p.id ? 'busy' : p.state) }))));
  const behind = view === 'project' ? snap.projects.find((p) => p.id === current) : lead;
  backdrop($('ambient'), behind && behind.hero);
}

function renderLibrary() {
  $('first-run').hidden = !!snap.install_root;
  $('first-run-title').textContent = snap.missing_root ? t('root_gone_title') : t('first_run_title');
  $('first-run-text').textContent = snap.missing_root ? t('root_gone', { path: snap.missing_root }) : t('first_run');
  const ready = snap.projects.filter((p) => p.state === 'ready').length;
  $('lib-sub').textContent = t('lib_sub', { n: snap.projects.length, ready });
  const cards = $('cards');
  cards.replaceChildren();
  for (const p of snap.projects) {
    const art = el('div', { class: 'art', style: `background-image:${gradient(p)}` });
    if (p.hero) art.append(picture(p.hero));
    const top = el('div', { class: 'card-top' },
      p.logo ? picture(p.logo, 'card-logo') : el('span'),
      p.new_entries.length ? el('span', { class: 'chip new' }, t('update_available')) : null);
    // Until there is a folder to install into, the projects are only shown:
    // the welcome box above is the one thing to do. A project that already
    // has a folder of its own stays in use. A card has no button of its own:
    // it opens the project's page, where everything is done
    const locked = !snap.install_root && !p.dir;
    const card = locked
      ? { class: 'card locked', 'aria-disabled': 'true', title: t('first_run') }
      : { class: 'card', tabindex: '0', onclick: () => openProject(p.id) };
    cards.append(el('div', card,
      art, top,
      el('div', { class: 'card-foot' },
        el('div', {}, el('h3', {}, p.name), el('div', { class: 'state' }, ...stateLine(p))))));
  }
}

const PROJECT_NEWS = 3;
let projectNewsAll = false;
let shownProject = null;  // so the hero pictures are not reloaded on every render
function renderProject() {
  const p = snap.projects.find((x) => x.id === current);
  if (!p) { show('library'); return; }
  const busy = !!snap.busy;
  $('back-label').textContent = t(cameFrom === 'home' ? 'nav_home' : 'nav_library');
  if (shownProject !== p.id) {
    shownProject = p.id;
    projectNewsAll = false;
    $('p-bg').style.backgroundImage = gradient(p);
    $('p-bg').replaceChildren(...(p.hero ? [picture(p.hero)] : []));
    $('p-logo').replaceChildren(...(p.logo ? [picture(p.logo)] : []));
    $('p-screens').replaceChildren(...p.screens.map((src) => {
      const img = picture(src);
      img.classList.add('in');
      img.addEventListener('click', () => { $('lightbox-img').src = src; $('lightbox').hidden = false; });
      return img;
    }));
    $('view-project').scrollTop = 0;
  }
  $('p-screens-block').hidden = !p.screens.length;
  $('p-name').textContent = p.name;
  $('p-name').hidden = !!p.logo && !!$('p-logo').children.length;
  $('p-tagline').textContent = p.tagline;
  $('p-desc').textContent = p.description;
  $('p-tags').replaceChildren(...p.tags.map((tag) => el('span', { class: 'chip' }, tag)));
  $('p-status').replaceChildren(...stateLine(p));

  const btn = (ic, label, fn, cls = '') => el('button', { class: 'btn ' + cls, disabled: busy, onclick: fn }, ic ? icon(ic) : null, label);
  const actions = [primaryButton(p, true)];
  if (p.can_update) actions.push(btn('up', t('do_update'), () => startTask(p, 'update'), 'lg' + (p.new_entries.length ? ' attn' : '')));
  // In sight beside Play, but quiet: it is not what one comes here for
  if (p.state !== 'not_installed') actions.push(btn('trash', t('do_uninstall'), () => uninstall(p), 'ghost danger'));
  $('p-actions').replaceChildren(...actions);

  const installed = p.state !== 'not_installed';
  const details = $('p-details');
  details.replaceChildren(
    el('dt', {}, t('folder')), el('dd', {}, installed ? p.dir : t('not_installed_yet')),
    el('dt', {}, t('source')), el('dd', {}, p.url.replace('https://', '')));
  if (p.input_label) {
    details.append(el('dt', {}, t('your_file')), el('dd', {}, p.input || `${p.input_label} · ${t('not_chosen')}`));
    details.append(el('div', { class: 'note' }, icon('shield'), t('input_note')));
  }
  const more = [];
  if (p.can_rebuild) more.push(btn('hammer', t('do_rebuild'), () => startTask(p, 'build')));
  if (p.input_label) more.push(btn('file', t('do_change_file'), () => chooseInput(p, false)));
  if (installed) more.push(btn('folder', t('do_open_folder'), () => call('open_folder', { id: p.id })));
  if (p.state === 'ready') more.push(btn('play', t('do_shortcut'), () => addShortcut(p)));
  else more.push(btn('folder', t('do_existing'), () => chooseExisting(p)));
  more.push(el('button', { class: 'btn', onclick: () => T.opener.openUrl(p.url) }, icon('github'), 'GitHub'));
  $('p-more').replaceChildren(...more);

  $('p-news-title').textContent = t('news');
  const news = $('p-news');
  const entry = (e) => el('div', { class: 'entry' + (e.isNew ? ' new' : '') },
    el('h3', {}, dayLabel(e.title), e.isNew ? el('span', { class: 'chip new' }, t('news_new')) : null),
    el('ul', {}, ...e.lines.map((l) => el('li', {}, l))));
  // Entries with the same title (several on one date) show as one, and what
  // is new is never mixed with what is already installed
  const groups = [];
  for (const e of [...p.new_entries.map((n) => ({ ...n, isNew: true })), ...p.news.slice(0, 10).map((n) => ({ ...n, isNew: false }))]) {
    const last = groups[groups.length - 1];
    if (last && last.title === e.title && last.isNew === e.isNew) last.lines.push(...e.lines);
    else groups.push({ title: e.title, isNew: e.isNew, lines: [...e.lines] });
  }
  // The three latest show (and everything not installed yet); the rest
  // waits behind a button, so the page does not open as a wall of text
  const shown = projectNewsAll ? groups.length : Math.max(PROJECT_NEWS, groups.filter((g) => g.isNew).length);
  const rows = groups.slice(0, shown).map(entry);
  if (groups.length > shown) rows.push(el('button', { class: 'btn ghost small news-more', onclick: () => { projectNewsAll = true; renderProject(); } }, t('home_more')));
  news.replaceChildren(...(rows.length ? rows : [el('p', { class: 'dim' }, t('no_news'))]));
  news.classList.toggle('timeline', rows.length > 0);
}

function renderSettings() {
  $('s-root').textContent = snap.install_root || (snap.missing_root ? `${snap.missing_root} · ${t('not_found')}` : t('not_chosen'));
  for (const b of $('s-lang').children) b.classList.toggle('on', b.dataset.lang === lang);
}

function renderCredits() {
  $('c-repos').replaceChildren(...snap.projects.map((p) =>
    el('button', { onclick: () => T.opener.openUrl(p.url) }, icon('github'), p.name,
      el('span', { class: 'dim' }, p.url.replace('https://github.com/', '')))));
}

function clock(ms) {
  const s = Math.floor(ms / 1000);
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, '0')}`;
}

function renderDock() {
  const dock = $('dock');
  dock.hidden = !task;
  if (!task) return;
  dock.classList.toggle('collapsed', !logOpen);
  $('dock-title').textContent = t('task_' + task.action, { name: task.name });
  const cls = task.done ? (task.ok ? 'ok' : 'fail') : '';
  $('dock-spin').className = 'spinner ' + cls;
  // In plain words what is going on; the technical lines are in the log
  $('dock-step').textContent = task.done ? t(task.ok ? 'task_ok' : 'task_failed')
    : (task.phase ? t(task.phase) : task.label) + (task.amount ? ` · ${task.amount}` : '');
  $('dock-time').textContent = clock((task.ended || Date.now()) - task.started);
  const bar = $('dock-fill').parentElement;
  bar.className = 'progress ' + (cls || (task.total ? '' : 'busy'));
  $('dock-fill').style.width = task.done ? '' : (task.total ? `${Math.round(((task.step - 0.5) / task.total) * 100)}%` : '');
  $('dock-toggle').textContent = t(logOpen ? 'hide_log' : 'show_log');
  $('dock-cancel').hidden = task.done;
  $('dock-close').hidden = !task.done;
}

function appendLog(text) {
  const log = $('dock-log');
  const stick = log.scrollTop + log.clientHeight >= log.scrollHeight - 30;
  log.append(text + '\n');
  if (stick) log.scrollTop = log.scrollHeight;
  // The steps are numbered: "[2/5] Recompiling..."
  const m = /^\[(\d+)\/(\d+)\]\s*(.*)$/.exec(text);
  if (m) { task.step = +m[1]; task.total = +m[2]; task.label = m[3]; }
  else if (text.trim() && !task.total) task.label = text.trim();
  // A download's progress ("  32 of 120 MB") goes beside the phase
  const mb = /^\s+(\d+(?: of \d+)? MB)$/.exec(text);
  task.amount = mb ? mb[1] : (/^\s/.test(text) ? task.amount : '');
  const phase = PHASES.find(([re]) => re.test(m ? m[3] : text));
  if (phase) task.phase = phase[1];
  else if (m) task.phase = '';   // a step with no plain wording: its own name
}

// What the engine's and the projects' lines mean, for someone who has never
// built a program. First match wins.
const PHASES = [
  [/^Downloading the source of /, 'ph_source'],
  [/^Unpacking\.\.\.$/, 'ph_unpack'],
  [/^(Getting the build tools|Downloading (llvm-mingw|cmake|ninja|python)\b|Unpacking )/i, 'ph_tools'],
  [/^(Up to date|New version)/, 'ph_check_update'],
  [/winget|Installing (Python|Build Tools|the C\+\+)|Visual Studio Installer/i, 'ph_pc_tools'],
  [/^(Getting the libraries|Downloading )/, 'ph_libs'],
  [/^Extracting the ROM/, 'ph_rom'],
  [/^Recompiling/, 'ph_recomp'],
  [/^(Configuring|Setting up Visual Studio)/, 'ph_configure'],
  [/^(Compiling the tests|Testing)/, 'ph_tests'],
  [/^(Compiling|Building)/, 'ph_compile'],
  [/^Copying/, 'ph_place'],
];

function render() {
  if (!snap) return;
  lang = STR[snap.language] ? snap.language : lang;
  document.documentElement.lang = lang;
  for (const n of document.querySelectorAll('[data-i18n]')) n.textContent = t(n.dataset.i18n);
  for (const n of document.querySelectorAll('.nav')) n.classList.toggle('on', n.dataset.view === view || (view === 'project' && n.dataset.view === 'library'));
  // The highlight of the sidebar slides to the page that is open
  const on = document.querySelector('.nav.on');
  $('nav-pill').hidden = !on;
  if (on) { $('nav-pill').style.transform = `translateY(${on.offsetTop}px)`; $('nav-pill').style.height = on.offsetHeight + 'px'; }
  for (const v of ['home', 'library', 'project', 'settings', 'credits']) $('view-' + v).hidden = v !== view;
  $('version').textContent = 'v' + snap.version;
  $('self-update').hidden = !selfUpdate;
  if (selfUpdate) {
    $('self-update').textContent = selfUpdate.installing ? t('self_updating') : t('self_update', { v: selfUpdate.version });
    $('self-update').disabled = !!selfUpdate.installing || !!snap.busy;
    $('self-update').title = selfUpdate.notes || '';
  }
  renderHome();
  renderLibrary();
  if (view === 'project') renderProject();
  renderSettings();
  renderCredits();
  renderDock();
}

// Changing page: what the new page holds arrives one piece after another
let entering = null;
function show(v) {
  const go = () => {
    view = v;
    if (v !== 'project') shownProject = null;
    render();
    const page = $('view-' + view);
    page.classList.add('entering');
    clearTimeout(entering);
    entering = setTimeout(() => { for (const n of document.querySelectorAll('.view.entering')) n.classList.remove('entering'); }, 900);
  };
  go();
}
// Back from a game's page returns to where it was opened from: Home or Library
let cameFrom = 'library';
function openProject(id) { if (view === 'home' || view === 'library') cameFrom = view; current = id; show('project'); }
function goBack() { show(cameFrom); }

for (const n of document.querySelectorAll('[data-icon]')) n.innerHTML = `<svg viewBox="0 0 24 24">${ICONS[n.dataset.icon]}</svg>`;
for (const n of document.querySelectorAll('.nav')) n.addEventListener('click', () => show(n.dataset.view));
for (const n of document.querySelectorAll('[data-url]')) n.addEventListener('click', () => T.opener.openUrl(n.dataset.url));
$('back').addEventListener('click', goBack);
$('first-run-pick').addEventListener('click', pickInstallRoot);
$('s-pick').addEventListener('click', pickInstallRoot);

// A newer launcher, when its releases have one. Asked once at start; a failed
// check (no connection) is not worth a message.
let selfUpdate = null;
$('self-update').addEventListener('click', async () => {
  selfUpdate.installing = true;
  render();
  // On success the launcher is replaced and started again: nothing comes back
  try { await invoke('install_launcher_update'); } catch (e) { toast(String(e)); }
  selfUpdate.installing = false;
  render();
});
$('lib-refresh').addEventListener('click', () => update('refresh'));
$('home-refresh').addEventListener('click', () => update('refresh'));
$('home-video').addEventListener('click', () => { const u = $('home-video').dataset.url; if (u) T.opener.openUrl(u); });
$('home-more').addEventListener('click', () => { homeDays = HOME_DAYS_ALL; renderHome(); });
// The launcher draws its own title bar: these are its three buttons. Dragging
// and the double click to maximize come from data-tauri-drag-region.
const win = T.window && T.window.getCurrentWindow ? T.window.getCurrentWindow() : null;
if (win) {
  $('win-min').addEventListener('click', () => win.minimize());
  $('win-max').addEventListener('click', () => win.toggleMaximize());
  $('win-close').addEventListener('click', () => win.close());
  const shape = async () => {
    const full = await win.isMaximized();
    $('win-max-icon').setAttribute('d', full ? 'M.5 2.5h7v7h-7zM2.500 2.500v-2h7v7h-2' : 'M.5.5h9v9h-9z');
    document.body.classList.toggle('maximized', full);
  };
  win.onResized(shape);
  shape();
} else document.querySelector('.titlebar').hidden = true;

$('c-license').addEventListener('click', () => showLicense(false));
for (const b of $('s-lang').children) b.addEventListener('click', () => { lang = b.dataset.lang; update('set_language', { language: lang }); });
$('dock-toggle').addEventListener('click', () => { logOpen = !logOpen; renderDock(); const l = $('dock-log'); l.scrollTop = l.scrollHeight; });
$('dock-cancel').addEventListener('click', () => { if (task) task.cancelled = true; call('cancel_task'); });
$('dock-close').addEventListener('click', () => { task = null; clearInterval(timer); renderDock(); });
$('dock-copy').addEventListener('click', async () => {
  await navigator.clipboard.writeText($('dock-log').textContent);
  const b = $('dock-copy');
  b.textContent = t('copied');
  setTimeout(() => { b.textContent = t('copy_log'); }, 1200);
});
$('lightbox').addEventListener('click', () => { $('lightbox').hidden = true; });
document.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') {
    if (licenseBox) { if (!licenseBox.dataset.must) { licenseBox.remove(); licenseBox = null; } }
    else if (closeAsk) closeAsk();
    else if (!$('lightbox').hidden) $('lightbox').hidden = true;
    else if (view === 'project') goBack();
  }
  if (e.key === 'Enter' && document.activeElement.classList.contains('card')) document.activeElement.click();
});
// No browser context menu or image dragging: it is an app, not a page
document.addEventListener('contextmenu', (e) => { if (!e.target.closest('pre, code, dd, .toast')) e.preventDefault(); });

// Back in the launcher after doing something else (deleting or moving a
// folder in Explorer, say): look at the disk again
window.addEventListener('focus', () => { if (snap && !(task && !task.done)) update('snapshot'); });

T.event.listen('task', async ({ payload }) => {
  if (!task || task.project !== payload.project || task.done) {
    // A task this window did not start itself (after a reload)
    const p = snap && snap.projects.find((x) => x.id === payload.project);
    newTask({ id: payload.project, name: p ? p.name : payload.project }, 'setup');
  }
  if (payload.kind === 'line') {
    appendLog(payload.text);
    renderDock();
  } else {
    task.done = true;
    task.ok = payload.ok;
    task.ended = Date.now();
    clearInterval(timer);
    if (payload.message) appendLog(payload.message);
    if (!payload.ok) logOpen = true;
    const finished = task;
    await update('snapshot');
    const l = $('dock-log'); l.scrollTop = l.scrollHeight;
    // An install this window started that now waits for the user's own file:
    // ask for it right away, and build once it is chosen
    const p = snap && snap.projects.find((x) => x.id === payload.project);
    if (payload.ok && finished.own && finished.action === 'setup' && p && p.state === 'needs_input') chooseInput(p, true, finished.fresh);
    // Cancelled while it was still downloading: the same clean way out
    // The game can be played for the first time: say so, and offer a shortcut
    else if (payload.ok && finished.own && !finished.wasReady && p && p.state === 'ready') celebrate(p);
    else if (!payload.ok && finished.own && finished.fresh && finished.cancelled && p && p.state !== 'not_installed' && p.state !== 'ready') abandon(p);
  }
});

(async () => {
  // English is the base language; another one is the user's choice in Settings
  await update('snapshot');
  if (snap && !snap.license_accepted) await showLicense(true);
  // Nothing can be done until there is a folder to install into: start where
  // that is asked for
  if (snap && !snap.install_root && view === 'home') show('library');
  if (snap && snap.config_error) toast(`${t('settings_damaged')} ${snap.config_error}`);
  // Project list and news from GitHub; the built-in list shows meanwhile
  await update('refresh');
  try { selfUpdate = await invoke('check_launcher_update'); render(); } catch (e) { /* offline, or no release yet */ }
})();
