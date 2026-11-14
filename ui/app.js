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
    bad_folder: '{path} has [ ] or ; in its name, and the build tools cannot work in a folder like that. Choose a folder without them (Settings, Install folder).',
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
    version: 'Version', version_short: 'Version {n}', version_out: 'version {n} is out', commit_title: 'The exact code installed, on GitHub',
    link_failed: 'Could not open {url}: {error}',
    not_chosen: 'not chosen', not_found: 'not found', not_installed_yet: 'not installed',
    news: 'What is new', news_new: 'New', news_installed: 'Already installed', no_news: 'No news yet.',
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
    ach_title: 'Achievements', ach_view_all: 'View all', ach_count: '{n} of {total} unlocked', ach_unlocked_on: 'Unlocked {date}', ach_locked: 'Locked',
    ach_toast_one: '{name}: achievement unlocked, {ach}', ach_toast_many: '{name}: {n} achievements unlocked',
    act_setup: 'install', act_build: 'build', act_update: 'update',
    nav_feedback: 'Feedback', fb_sub: 'Something broken, an idea, or a game you would love to see next? Every message is read.',
    fb_see_all: 'See what others sent',
    fb_bug_title: 'Report a problem', fb_bug_text: 'A game or the launcher does not work as it should: it closes, looks wrong, or an install fails.',
    fb_idea_title: 'Suggest an improvement', fb_idea_text: 'A change you would like in a game or in the launcher: a setting, a control, a new feature.',
    fb_recomp_title: 'Propose a recomp', fb_recomp_text: 'Which arcade or console game would you love to see running natively next? Others can vote for it.',
    fb_recomp_warn: 'Name the game only: no ROMs, game files or links to them.',
    fb_talk_title: 'Ask or comment', fb_talk_text: 'A question, a thank-you, or anything else that is not a problem or an idea.',
    fb_where_issue: 'Opens a report on GitHub', fb_where_disc: 'Opens GitHub Discussions', fb_where_vote: 'Public, with votes',
    fb_sent_title: 'What goes with your message', fb_sent_text: 'Added for you, so nobody has to ask. You see all of it on GitHub before sending, and can remove anything.',
    fb_launcher: 'Launcher', fb_system: 'Windows', fb_gpu: 'Graphics',
    fb_rules_title: 'Before you write',
    fb_rule_roms: 'Never share ROMs, game files or links to them. Messages that do are deleted.',
    fb_rule_search: 'Look at what others sent first: maybe it is already there, and a vote helps more than a copy.',
    fb_rule_one: 'One problem or idea per message, so each one can be followed and closed.',
    fb_rule_account: 'Sending needs a free GitHub account. Without one, you can still read everything.',
    ann_title: 'Announcements', ann_new: 'New announcement', ann_new_many: '{n} new announcements', ann_label: 'Announcement',
    ann_important: 'Important', ann_from: 'from snicolast', ann_of: '{i} of {n}', ann_prev: 'Previous', ann_next: 'Next',
    cl_badge: 'Updated', cl_title: 'What is new in {name}', cl_from: 'From version {a} to version {b}', cl_one: '1 update', cl_many: '{n} updates',
    cl_latest: 'Latest', cl_all: 'All the news', cl_title_launcher: 'What is new in NicoLauncher',
    fb_report_this: 'Report this problem', fb_report_game: 'Report a problem',
    fb_bug_lead: 'Tell what happened in your own words. The versions and the log are added for you.',
    fb_idea_lead: 'Tell what you would change, and why it would be better.',
    fb_talk_lead: 'Ask or tell whatever you want.',
    fb_recomp_lead: 'Tell which game you would like to see recompiled for PC.',
    fb_where: 'Where is it?', fb_title_label: 'Title', fb_title_ph: 'In a few words, what goes wrong',
    fb_what: 'What happened?', fb_what_ph: 'What you did, what you expected, and what happened instead.',
    fb_idea_ph: 'In a few words, your idea', fb_idea_what: 'Your idea', fb_idea_what_ph: 'What it would do, and how you would use it.',
    fb_attach: 'Added for you', fb_attach_log: 'Log of the last {action}, failed {when}', fb_attach_shot: 'Add a screenshot (you can paste it on GitHub)',
    fb_go: 'Continue on GitHub',
    fb_go_note: 'GitHub opens in your browser with all of this filled in. Nothing is sent until you press Submit there.',
    fb_recomp_name: 'Game', fb_recomp_name_ph: 'Name of the game', fb_recomp_system: 'Original system', fb_recomp_system_ph: 'Arcade board or console, e.g. Sega Model 2',
    fb_recomp_why: 'Why this one?', fb_recomp_why_ph: 'What makes it special, and what you would like it to have on PC.',
    fb_recomp_public: 'Proposals are public and others can vote for them. The most voted are looked at first, but no game is promised.',
    fb_launcher_name: 'The launcher', fb_not_installed: 'not installed', fb_general: 'General',
  },
  es: {
    nav_library: 'Biblioteca', nav_settings: 'Ajustes', nav_credits: 'Créditos',
    by_author: 'por', refresh: 'Actualizar lista', lib_sub: '{n} proyectos · {ready} listos para jugar',
    first_run_title: 'Bienvenido', about: 'Acerca de', screenshots: 'Capturas', language_help: 'El idioma del launcher.',
    credits_projects: 'Proyectos', show_log: 'Ver log', hide_log: 'Ocultar log', installed_version: 'Novedades instaladas',
    first_run: 'Elige la carpeta donde se instalarán los proyectos. Cada uno tendrá su propia carpeta dentro.',
    root_gone_title: 'La carpeta de instalación no está', root_gone: 'No se encontró {path}: se borró o se movió, o su unidad no está conectada. Elige la carpeta donde se instalarán los proyectos.',
    choose_folder: 'Elegir carpeta',
    bad_folder: '{path} tiene [ ] o ; en el nombre, y las herramientas de compilación no funcionan en una carpeta así. Elige una carpeta sin esos caracteres (Ajustes, Carpeta de instalación).',
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
    version: 'Versión', version_short: 'Versión {n}', version_out: 'ya está la versión {n}', commit_title: 'El código exacto instalado, en GitHub',
    link_failed: 'No se pudo abrir {url}: {error}',
    not_chosen: 'sin elegir', not_found: 'no encontrada', not_installed_yet: 'sin instalar',
    news: 'Novedades', news_new: 'Nuevo', news_installed: 'Ya instalado', no_news: 'Aún no hay novedades.',
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
    ach_title: 'Logros', ach_view_all: 'Ver todos', ach_count: '{n} de {total} desbloqueados', ach_unlocked_on: 'Desbloqueado el {date}', ach_locked: 'Bloqueado',
    ach_toast_one: '{name}: logro desbloqueado, {ach}', ach_toast_many: '{name}: {n} logros desbloqueados',
    act_setup: 'instalación', act_build: 'compilación', act_update: 'actualización',
    nav_feedback: 'Opiniones', fb_sub: '¿Algo falla, tienes una idea o un juego que te encantaría ver? Se leen todos los mensajes.',
    fb_see_all: 'Ver lo que otros enviaron',
    fb_bug_title: 'Informar de un fallo', fb_bug_text: 'Un juego o el launcher no funciona como debe: se cierra, se ve mal o falla una instalación.',
    fb_idea_title: 'Proponer una mejora', fb_idea_text: 'Un cambio que te gustaría en un juego o en el launcher: un ajuste, un control, algo nuevo.',
    fb_recomp_title: 'Proponer un recomp', fb_recomp_text: '¿Qué juego de recreativa o consola te gustaría ver funcionando de forma nativa? Otros pueden votarlo.',
    fb_recomp_warn: 'Solo el nombre del juego: nada de ROMs, archivos del juego ni enlaces a ellos.',
    fb_talk_title: 'Preguntar o comentar', fb_talk_text: 'Una pregunta, un agradecimiento o cualquier otra cosa que no sea un fallo ni una idea.',
    fb_where_issue: 'Abre un informe en GitHub', fb_where_disc: 'Abre GitHub Discussions', fb_where_vote: 'Público, con votos',
    fb_sent_title: 'Lo que va con tu mensaje', fb_sent_text: 'Se añade solo, para que nadie tenga que preguntarlo. Lo ves todo en GitHub antes de enviarlo y puedes quitar lo que quieras.',
    fb_launcher: 'Launcher', fb_system: 'Windows', fb_gpu: 'Gráfica',
    fb_rules_title: 'Antes de escribir',
    fb_rule_roms: 'Nunca compartas ROMs, archivos del juego ni enlaces a ellos. Los mensajes que lo hagan se borran.',
    fb_rule_search: 'Mira primero lo que otros enviaron: quizá ya está, y un voto ayuda más que una copia.',
    fb_rule_one: 'Un fallo o una idea por mensaje, para poder seguir y cerrar cada uno.',
    fb_rule_account: 'Para enviar hace falta una cuenta gratuita de GitHub. Sin ella puedes leerlo todo igualmente.',
    ann_title: 'Avisos', ann_new: 'Aviso nuevo', ann_new_many: '{n} avisos nuevos', ann_label: 'Aviso',
    ann_important: 'Importante', ann_from: 'de snicolast', ann_of: '{i} de {n}', ann_prev: 'Anterior', ann_next: 'Siguiente',
    cl_badge: 'Actualizado', cl_title: 'Novedades de {name}', cl_from: 'De la versión {a} a la versión {b}', cl_one: '1 actualización', cl_many: '{n} actualizaciones',
    cl_latest: 'Última', cl_all: 'Todas las novedades', cl_title_launcher: 'Novedades de NicoLauncher',
    fb_report_this: 'Informar de este fallo', fb_report_game: 'Informar de un fallo',
    fb_bug_lead: 'Cuenta con tus palabras qué pasó. Las versiones y el registro se añaden solos.',
    fb_idea_lead: 'Cuenta qué cambiarías y por qué sería mejor.',
    fb_talk_lead: 'Pregunta o cuenta lo que quieras.',
    fb_recomp_lead: 'Di qué juego te gustaría ver recompilado para PC.',
    fb_where: '¿Dónde está?', fb_title_label: 'Título', fb_title_ph: 'En pocas palabras, qué falla',
    fb_what: '¿Qué pasó?', fb_what_ph: 'Qué hiciste, qué esperabas y qué pasó en cambio.',
    fb_idea_ph: 'En pocas palabras, tu idea', fb_idea_what: 'Tu idea', fb_idea_what_ph: 'Qué haría y cómo la usarías.',
    fb_attach: 'Se añade solo', fb_attach_log: 'Registro de la última {action}, falló {when}', fb_attach_shot: 'Añadir una captura (puedes pegarla en GitHub)',
    fb_go: 'Seguir en GitHub',
    fb_go_note: 'GitHub se abre en tu navegador con todo esto rellenado. No se envía nada hasta que pulses Submit allí.',
    fb_recomp_name: 'Juego', fb_recomp_name_ph: 'Nombre del juego', fb_recomp_system: 'Sistema original', fb_recomp_system_ph: 'Placa de recreativa o consola, p. ej. Sega Model 2',
    fb_recomp_why: '¿Por qué este?', fb_recomp_why_ph: 'Qué lo hace especial y qué te gustaría que tuviera en PC.',
    fb_recomp_public: 'Las propuestas son públicas y otros pueden votarlas. Las más votadas se miran antes, pero no se promete ningún juego.',
    fb_launcher_name: 'El launcher', fb_not_installed: 'sin instalar', fb_general: 'General',
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
  check: '<path d="M5 12.5l4.500 4.500L19 7"/>',
  trash: '<path d="M4 7h16M10 11v6M14 11v6M6 7l1 13h10l1-13M9 7V4h6v3"/>',
  trophy: '<path d="M8 4h8v5a4 4 0 0 1-8 0z"/><path d="M8 6H5a3 3 0 0 0 3 4.500M16 6h3a3 3 0 0 1-3 4.500"/><path d="M12 13v4M9 20h6M10 17h4"/>',
  lock: '<rect x="5" y="11" width="14" height="9" rx="2"/><path d="M8 11V8a4 4 0 0 1 8 0v3"/>',
  chat: '<path d="M4 5.5A1.500 1.500 0 0 1 5.500 4h13A1.500 1.500 0 0 1 20 5.500v9a1.500 1.500 0 0 1-1.500 1.500H10l-4.500 4v-4h0A1.500 1.500 0 0 1 4 14.500z"/><path d="M8.500 9h7M8.500 12h4"/>',
  bug: '<rect x="7" y="8" width="10" height="12" rx="5"/><path d="M9 8a3 3 0 0 1 6 0M12 12v8M3 13h4M17 13h4M4 7l3 2.500M20 7l-3 2.500M4 19l3-2.500M20 19l-3-2.500"/>',
  bulb: '<path d="M9 18h6M10 21h4"/><path d="M12 3a6 6 0 0 0-3.500 10.900c.6.5 1 1.200 1 2.100h5c0-.9.4-1.600 1-2.100A6 6 0 0 0 12 3z"/>',
  joystick: '<rect x="3" y="15" width="18" height="6" rx="2"/><path d="M12 15V9"/><circle cx="12" cy="6" r="3"/><path d="M7 18h2"/>',
  ban: '<circle cx="12" cy="12" r="8.500"/><path d="M6 6l12 12"/>',
  search: '<circle cx="11" cy="11" r="6.500"/><path d="M16 16l4.500 4.500"/>',
  megaphone: '<path d="M3.5 10.5v3a1.500 1.500 0 0 0 1.500 1.500H7l9 4.500V4.500L7 9H5a1.500 1.500 0 0 0-1.500 1.500z"/><path d="M7.500 15l1.300 4.500h2.400l-1-4"/><path d="M19 9.500a3.500 3.500 0 0 1 0 5"/>',
  chevl: '<path d="M15 6l-6 6 6 6"/>',
  chevr: '<path d="M9 6l6 6-6 6"/>',
  external: '<path d="M14 4h6v6M20 4l-9 9"/><path d="M18 14v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4"/>',
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
// A web page in the user's browser. A failure says so instead of leaving the
// click without any effect
function openLink(url) {
  T.opener.openUrl(url).catch((e) => toast(t('link_failed', { url, error: e })));
}

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

// CMake reads [ ] as wildcards and ; as a list separator: nothing builds in a
// folder with them (the engine refuses it too, with the same check)
const badFolder = (path) => /[[\];]/.test(path || '');

async function pickInstallRoot() {
  const path = await T.dialog.open({ directory: true, multiple: false });
  if (!path) return false;
  if (badFolder(path)) { toast(t('bad_folder', { path })); return false; }
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
  if (badFolder(p.dir)) { toast(t('bad_folder', { path: p.dir })); return; }
  newTask(p, action);
  task.own = true;
  task.fresh = action === 'setup' && p.state === 'not_installed';
  // Not playable before this task: if it ends playable, the install is over
  task.wasReady = p.state === 'ready';
  // The version before an update: what is newer afterwards is shown
  if (action === 'update') task.fromVersion = Math.max(0, ...p.news.map((e) => e.number));
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
  if (path && badFolder(path)) toast(t('bad_folder', { path }));
  else if (path) update('set_project_dir', { id: p.id, path });
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

// Something new is waiting for an installed project
const hasUpdate = (p) => p.can_update && p.new_entries.length > 0;

function stateLine(p) {
  const working = snap.busy === p.id;
  return [el('span', { class: 'dot ' + (working ? 'busy' : p.state) }), working ? t('st_busy') : t('st_' + p.state)];
}

// The installed version: the number of the newest news entry the installed
// copy has (each entry is a version), its date, and the commit it was made
// from. "Version 14 · 14 Nov 2026"
function dateLabel(title) {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(dayKey(title || ''));
  return m ? new Date(+m[1], +m[2] - 1, +m[3]).toLocaleDateString(lang, { day: 'numeric', month: 'short', year: 'numeric' }) : (title || '');
}

function versionText(p) {
  const v = p.version;
  if (!v) return '';
  const parts = [];
  if (v.number != null) parts.push(t('version_short', { n: v.number }));
  if (v.title) parts.push(dateLabel(v.title));
  return parts.join(' · ');
}

// The Details row: the version, a link to its commit, and the number waiting
// on GitHub when there is a newer one
function versionDetail(p) {
  const v = p.version;
  const dd = el('dd', {}, versionText(p));
  if (v.commit) {
    if (dd.textContent) dd.append(' · ');
    dd.append(el('a', { class: 'commit', href: '#', title: t('commit_title'),
      onclick: (e) => { e.preventDefault(); openLink(`${p.url}/commit/${v.commit}`); } }, v.commit.slice(0, 7)));
  }
  const newest = Math.max(0, ...p.new_entries.map((e) => e.number));
  if (newest > (v.number ?? 0)) dd.append(el('span', { class: 'version-out' }, ' · ' + t('version_out', { n: newest })));
  return dd;
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
    if (!block) day.blocks.push(block = { from: e.from, lines: [], had: [], isNew: false });
    // What the waiting update brings is kept apart from what the installed
    // copy already has, so a day half installed does not look all new
    // (entries come newest first, so the new ones are always met first)
    (block.isNew && !e.isNew ? block.had : block.lines).push(...e.lines);
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
    el('ul', {}, ...b.lines.map((l) => el('li', {}, l))),
    b.had.length ? el('div', { class: 'feed-had' }, el('span', { class: 'feed-had-label' }, icon('check'), t('news_installed'))) : null,
    b.had.length ? el('ul', { class: 'had' }, ...b.had.map((l) => el('li', {}, l))) : null);
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
      el('button', { class: 'btn small attn', disabled: !!snap.busy, onclick: () => startTask(p, 'update') }, icon('up'), t('do_update')))),
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
      el('span', { class: 'dot ' + (snap.busy === p.id ? 'busy' : hasUpdate(p) ? 'fresh' : p.state) }))));
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
        el('div', {}, el('h3', {}, p.name), el('div', { class: 'state' }, ...stateLine(p))),
        achBadge(p))));
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
    achSeen = markAchievementsSeen(p);
  }
  $('p-screens-block').hidden = !p.screens.length;
  $('p-name').textContent = p.name;
  $('p-name').hidden = !!p.logo && !!$('p-logo').children.length;
  $('p-tagline').textContent = p.tagline;
  $('p-desc').textContent = p.description;
  $('p-tags').replaceChildren(...p.tags.map((tag) => el('span', { class: 'chip' }, tag)));
  // Beside the state, the number only; the date and commit are in Details
  const vn = p.version?.number;
  $('p-status').replaceChildren(...stateLine(p), ...(vn != null ? [el('span', { class: 'sep' }, '·'), el('span', { class: 'nowrap' }, t('version_short', { n: vn }))] : []));

  const btn = (ic, label, fn, cls = '') => el('button', { class: 'btn ' + cls, disabled: busy, onclick: fn }, ic ? icon(ic) : null, label);
  const actions = [primaryButton(p, true)];
  if (p.can_update) actions.push(btn('up', t('do_update'), () => startTask(p, 'update'), 'lg' + (hasUpdate(p) ? ' attn' : ' idle')));
  // In sight beside Play, but quiet: it is not what one comes here for
  if (p.state !== 'not_installed') actions.push(btn('trash', t('do_uninstall'), () => uninstall(p), 'ghost danger'));
  $('p-actions').replaceChildren(...actions);

  const installed = p.state !== 'not_installed';
  const details = $('p-details');
  details.replaceChildren(
    el('dt', {}, t('folder')), el('dd', {}, installed ? p.dir : t('not_installed_yet')),
    el('dt', {}, t('source')), el('dd', {}, p.url.replace('https://', '')));
  if (installed && p.version) details.append(el('dt', {}, t('version')), versionDetail(p));
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
  more.push(el('button', { class: 'btn', onclick: () => openLink(p.url) }, icon('github'), 'GitHub'));
  more.push(btn('bug', t('fb_report_game'), () => feedback('bug', p.id)));
  $('p-more').replaceChildren(...more);

  $('p-news-title').textContent = t('news');
  const news = $('p-news');
  const entry = (e) => el('div', { class: 'entry' + (e.isNew ? ' new' : '') },
    el('h3', {}, dayLabel(e.title), e.isNew ? el('span', { class: 'chip new' }, t('news_new')) : null),
    el('ul', {}, ...e.lines.map((l) => el('li', {}, l))),
    e.had.length ? el('div', { class: 'feed-had' }, el('span', { class: 'feed-had-label' }, icon('check'), t('news_installed'))) : null,
    e.had.length ? el('ul', { class: 'had' }, ...e.had.map((l) => el('li', {}, l))) : null);
  // Entries with the same title (several on one date) show as one day. On a
  // day that mixes both, what the waiting update brings goes first and what
  // is already installed below it, dimmed (as on the home page)
  const groups = [];
  for (const e of [...p.new_entries.map((n) => ({ ...n, isNew: true })), ...p.news.slice(0, 10).map((n) => ({ ...n, isNew: false }))]) {
    let g = groups.find((x) => x.title === e.title);
    if (!g) groups.push(g = { title: e.title, isNew: false, lines: [], had: [] });
    (g.isNew && !e.isNew ? g.had : g.lines).push(...e.lines);
    g.isNew = g.isNew || e.isNew;
  }
  // The three latest show (and everything not installed yet); the rest
  // waits behind a button, so the page does not open as a wall of text
  const shown = projectNewsAll ? groups.length : Math.max(PROJECT_NEWS, groups.filter((g) => g.isNew).length);
  const rows = groups.slice(0, shown).map(entry);
  if (groups.length > shown) rows.push(el('button', { class: 'btn ghost small news-more', onclick: () => { projectNewsAll = true; renderProject(); } }, t('home_more')));
  news.replaceChildren(...(rows.length ? rows : [el('p', { class: 'dim' }, t('no_news'))]));
  news.classList.toggle('timeline', rows.length > 0);
  renderAchievements(p);
}

// ------------------------------------------------------------ achievements
// Each project writes its own list (save/achievements.json in its folder);
// the launcher only shows it. Achievements unlocked since the page was last
// opened are marked as new for that visit.
let achSeen = 0;
const achKnown = new Map();   // project id -> ids unlocked, while the launcher runs
const unlockedOf = (p) => p.achievements.filter((a) => a.unlocked > 0);

function achBadge(p) {
  if (!p.achievements.length) return null;
  return el('span', { class: 'ach-badge', title: t('ach_title') }, icon('trophy'), `${unlockedOf(p).length}/${p.achievements.length}`);
}

function markAchievementsSeen(p) {
  const key = 'ach-seen-' + p.id;
  let seen = 0;
  try { seen = Number(localStorage.getItem(key)) || 0; } catch (e) { /* no storage: nothing is marked */ }
  const newest = Math.max(0, ...p.achievements.map((a) => a.unlocked));
  try { if (newest > seen) localStorage.setItem(key, String(newest)); } catch (e) { /* ignore */ }
  // The first visit marks nothing: everything would be new
  return seen;
}

function achDate(unix) {
  return new Date(unix * 1000).toLocaleDateString(lang === 'es' ? 'es-ES' : 'en-GB', { day: 'numeric', month: 'short', year: 'numeric' });
}

// Card of one achievement, in the full list
function achCard(a) {
  const on = a.unlocked > 0;
  const fresh = on && achSeen > 0 && a.unlocked > achSeen;
  let foot;
  if (on) foot = [icon('check'), t('ach_unlocked_on', { date: achDate(a.unlocked) })];
  else if (a.goal > 1) foot = [el('div', { class: 'ach-mini' }, el('div', { style: `width:${achPct(a)}%` })), el('span', {}, achProgress(a))];
  else foot = [t('ach_locked')];
  return el('div', { class: 'ach' + (on ? ' on' : '') + (fresh ? ' fresh' : '') },
    el('div', { class: 'ach-icon' }, icon(on ? 'trophy' : 'lock')),
    el('div', { class: 'ach-body' },
      el('div', { class: 'ach-name' }, a.name, fresh ? el('span', { class: 'chip new' }, t('news_new')) : null),
      a.description ? el('div', { class: 'ach-desc' }, a.description) : null,
      el('div', { class: 'ach-foot' }, ...foot)));
}
const achPct = (a) => (100 * a.progress) / a.goal;
const achProgress = (a) => a.progress_text || `${a.progress.toLocaleString('en-US')} / ${a.goal.toLocaleString('en-US')}`;
// Unlocked first, newest first; then the rest in the project's own order
const achOrder = (p) => [...unlockedOf(p)].sort((a, b) => b.unlocked - a.unlocked).concat(p.achievements.filter((a) => !a.unlocked));

// One row under the page's header: a tile per achievement, as many as fit,
// each with its details on hover; the whole list is one click away
const TILE = 44, TILE_GAP = 8;
function renderAchievements(p) {
  const list = p.achievements;
  $('p-ach-strip').hidden = !list.length;
  if (!list.length) return;
  const got = unlockedOf(p).length;
  $('p-ach-got').textContent = got;
  $('p-ach-total').textContent = ` / ${list.length}`;
  $('p-ach-all').textContent = t('ach_view_all');
  $('p-ach-all').onclick = () => openAchievements(p);
  const row = $('p-ach-row');
  const order = achOrder(p);
  const fits = Math.max(1, Math.floor((row.clientWidth + TILE_GAP) / (TILE + TILE_GAP)));
  const shown = order.length > fits ? order.slice(0, fits - 1) : order;
  const tiles = shown.map((a) => {
    const on = a.unlocked > 0;
    const fresh = on && achSeen > 0 && a.unlocked > achSeen;
    const detail = on ? t('ach_unlocked_on', { date: achDate(a.unlocked) }) : a.goal > 1 ? achProgress(a) : t('ach_locked');
    return el('div', { class: 'ach-tile' + (on ? ' on' : '') + (fresh ? ' fresh' : ''), tabindex: '0' },
      icon(on ? 'trophy' : 'lock'),
      !on && a.goal > 1 ? el('div', { class: 'ach-tile-bar' }, el('div', { style: `width:${achPct(a)}%` })) : null,
      el('div', { class: 'ach-tip' },
        el('strong', {}, a.name),
        a.description ? el('span', {}, a.description) : null,
        el('em', { class: on ? 'on' : '' }, detail)));
  });
  if (shown.length < order.length) {
    tiles.push(el('button', { class: 'ach-tile more', onclick: () => openAchievements(p) }, `+${order.length - shown.length}`));
  }
  row.replaceChildren(...tiles);
}

function openAchievements(p) {
  const list = p.achievements, got = unlockedOf(p).length;
  $('ach-modal-title').textContent = `${t('ach_title')} · ${p.name}`;
  $('ach-modal-count').textContent = t('ach_count', { n: got, total: list.length });
  $('ach-modal-bar').style.width = `${(100 * got) / list.length}%`;
  $('ach-modal-close').textContent = t('close');
  $('ach-modal-grid').replaceChildren(...achOrder(p).map(achCard));
  $('ach-modal').hidden = false;
}
$('ach-modal-close').addEventListener('click', () => { $('ach-modal').hidden = true; });
$('ach-modal').addEventListener('click', (e) => { if (e.target === $('ach-modal')) $('ach-modal').hidden = true; });
window.addEventListener('keydown', (e) => { if (e.key === 'Escape') $('ach-modal').hidden = true; });
window.addEventListener('resize', () => {
  const p = view === 'project' && snap && snap.projects.find((x) => x.id === current);
  if (p) renderAchievements(p);
});

// A toast when a refresh finds achievements unlocked since the last one (the
// launcher refreshes when its window comes back, as after closing a game)
function noticeAchievements() {
  for (const p of snap.projects) {
    if (!p.achievements.length) continue;
    const got = unlockedOf(p);
    const before = achKnown.get(p.id);
    achKnown.set(p.id, new Set(got.map((a) => a.id)));
    if (!before) continue;
    const fresh = got.filter((a) => !before.has(a.id));
    if (fresh.length === 1) toast(t('ach_toast_one', { name: p.name, ach: fresh[0].name }), true);
    else if (fresh.length > 1) toast(t('ach_toast_many', { name: p.name, n: fresh.length }), true);
  }
}

function renderSettings() {
  $('s-root').textContent = snap.install_root || (snap.missing_root ? `${snap.missing_root} · ${t('not_found')}` : t('not_chosen'));
  for (const b of $('s-lang').children) b.classList.toggle('on', b.dataset.lang === lang);
}

function renderCredits() {
  $('c-repos').replaceChildren(...snap.projects.map((p) =>
    el('button', { onclick: () => openLink(p.url) }, icon('github'), p.name,
      el('span', { class: 'dim' }, p.url.replace('https://github.com/', '')))));
}

// ---------------------------------------------------------------- feedback
// Nothing is sent from the launcher: each kind of message opens GitHub in the
// browser with a form already filled in (the versions, and the log when an
// install or build failed). The user reads it there and sends it.
const HUB = 'https://github.com/snicolast/NicoLauncher-Hub';
const FB_KINDS = [
  { id: 'bug', icon: 'bug', where: 'fb_where_issue' },
  { id: 'idea', icon: 'bulb', where: 'fb_where_disc' },
  { id: 'recomp', icon: 'joystick', where: 'fb_where_vote', warn: 'fb_recomp_warn' },
  { id: 'talk', icon: 'chat', where: 'fb_where_disc' },
];
let sysInfo = null;      // Windows and graphics card, asked once
let lastFail = null;     // { project, action, when, log } of the last task that failed

function versionLine(p, say = t) {
  if (p.state === 'not_installed') return say('fb_not_installed');
  const v = p.version || {};
  return [v.number != null ? say('version_short', { n: v.number }) : null, v.commit ? v.commit.slice(0, 7) : null].filter(Boolean).join(' · ');
}

function renderFeedback() {
  $('fb-cards').replaceChildren(...FB_KINDS.map((k) =>
    el('button', { class: 'fb-card ' + k.id, onclick: () => feedback(k.id) },
      el('span', { class: 'fb-badge' }, icon(k.icon)),
      el('h3', {}, t(`fb_${k.id}_title`)),
      el('p', { class: 'dim' }, t(`fb_${k.id}_text`)),
      k.warn ? el('div', { class: 'note warn' }, icon('ban'), t(k.warn)) : null,
      el('div', { class: 'fb-foot' }, el('span', {}, icon('github'), t(k.where)), el('span', { class: 'fb-go' }, icon('external'))))));
  const sys = $('fb-system');
  sys.replaceChildren(
    el('dt', {}, t('fb_launcher')), el('dd', {}, 'v' + snap.version),
    el('dt', {}, t('fb_system')), el('dd', {}, sysInfo ? sysInfo.os : '...'),
    el('dt', {}, t('fb_gpu')), el('dd', {}, sysInfo ? sysInfo.gpu : '...'));
  for (const p of snap.projects) sys.append(el('dt', {}, shortName(p)), el('dd', { class: p.state === 'not_installed' ? 'dim' : '' }, versionLine(p)));
}

function field(label, control) {
  return el('label', { class: 'field' }, el('span', {}, label), control);
}

// The form of one kind of message, then GitHub with it filled in
function feedback(kind, projectId, withLog) {
  const box = el('div', { class: 'modal', role: 'dialog', 'aria-modal': 'true' });
  const close = () => { closeAsk = null; box.remove(); };
  const bug = kind === 'bug';
  const body = [];
  let where = projectId || (bug ? (snap.projects.find((p) => p.state !== 'not_installed') || {}).id : null) || 'launcher';
  const targets = [{ id: 'launcher', name: t('fb_launcher_name') }, ...snap.projects.map((p) => ({ id: p.id, name: shortName(p) }))];
  const seg = el('div', { class: 'seg wide' });
  const drawSeg = () => seg.replaceChildren(...targets.map((x) => el('button', { type: 'button', class: x.id === where ? 'on' : '', onclick: () => { where = x.id; drawSeg(); drawAttach(); } }, x.name)));
  const attach = el('div', { class: 'fb-attach' });
  const logBox = el('input', { type: 'checkbox' });
  logBox.checked = withLog !== false;
  const failHere = () => bug && lastFail && lastFail.project === where;
  const drawAttach = () => {
    const items = details(where).map(([k, v]) => el('div', { class: 'fb-attach-row' }, icon('check'), el('span', {}, k), el('span', { class: 'dim' }, v)));
    if (failHere()) items.push(el('label', { class: 'fb-attach-row toggle' }, logBox, icon('file'),
      el('span', {}, t('fb_attach_log', { action: t('act_' + lastFail.action), when: lastFail.when }))));
    attach.replaceChildren(el('h3', {}, t('fb_attach')), ...items);
  };
  const input = (ph) => el('input', { type: 'text', placeholder: t(ph), maxlength: 140 });
  const title = input(kind === 'recomp' ? 'fb_recomp_name_ph' : bug ? 'fb_title_ph' : 'fb_idea_ph');
  const system = input('fb_recomp_system_ph');
  const text = el('textarea', { rows: bug ? 4 : 5, placeholder: t(kind === 'recomp' ? 'fb_recomp_why_ph' : bug ? 'fb_what_ph' : 'fb_idea_what_ph') });
  if (kind === 'recomp') {
    body.push(el('p', { class: 'dim' }, t('fb_recomp_lead')),
      el('div', { class: 'fb-two' }, field(t('fb_recomp_name'), title), field(t('fb_recomp_system'), system)),
      field(t('fb_recomp_why'), text),
      el('div', { class: 'fb-warn' }, icon('ban'), el('span', {}, t('fb_rule_roms'))),
      el('div', { class: 'note' }, icon('chat'), t('fb_recomp_public')));
  } else {
    body.push(el('p', { class: 'dim' }, t(`fb_${kind}_lead`)),
      field(t('fb_where'), seg),
      field(t('fb_title_label'), title),
      field(t(bug ? 'fb_what' : 'fb_idea_what'), text));
    if (bug) body.push(attach);
    drawSeg(); drawAttach();
  }
  const go = el('button', { class: 'btn primary', disabled: true, onclick: () => {
    const log = failHere() && logBox.checked ? lastFail.log : null;
    openLink(feedbackUrl(kind, where, { title: title.value.trim(), system: system.value.trim(), text: text.value.trim(), log }));
    close();
  } }, icon('external'), t('fb_go'));
  // Nothing to send without a title (or the game's name)
  title.addEventListener('input', () => { go.disabled = !title.value.trim(); });
  box.append(el('div', { class: 'modal-box fb-box' },
    el('div', { class: 'fb-box-head' }, el('span', { class: 'fb-badge ' + kind }, icon(FB_KINDS.find((k) => k.id === kind).icon)), el('h2', {}, t(`fb_${kind}_title`))),
    ...body,
    el('p', { class: 'fb-go-note dim small' }, t('fb_go_note')),
    el('div', { class: 'modal-actions' }, el('button', { class: 'btn ghost', onclick: close }, t('cancel')), go)));
  closeAsk = close;
  document.body.append(box);
  title.focus();
}

const shortName = (p) => p.name.replace(/ Native$/, '');

// What goes with a message about `where` (a project id, or 'launcher'), as
// [label, value] rows. Always in English: it is read on GitHub.
function details(where, english) {
  const say = (key, vars) => (english ? STR.en[key].replace(/\{(\w+)\}/g, (_, k) => vars?.[k] ?? '') : t(key, vars));
  const p = snap.projects.find((x) => x.id === where);
  const rows = [[say('fb_launcher'), 'v' + snap.version]];
  if (p) {
    rows.push([shortName(p), versionLine(p, say)]);
  }
  if (sysInfo) rows.push([say('fb_system'), sysInfo.os], [say('fb_gpu'), sysInfo.gpu]);
  return rows;
}

// Problems go to the issues of the repository they are about; ideas, recomp
// proposals and the rest to the launcher's Discussions, where they get votes.
// GitHub fills its form from the address: the user reads it there and sends it.
const URL_MAX = 7500;
function feedbackUrl(kind, where, { title, system, text, log }) {
  const p = snap.projects.find((x) => x.id === where);
  const base = kind === 'bug' ? `${p ? p.url : HUB}/issues/new`
    : `${HUB}/discussions/new?category=${{ idea: 'ideas', recomp: 'recomp-proposals', talk: 'general' }[kind]}`;
  const sep = base.includes('?') ? '&' : '?';
  const facts = details(where, true).map(([k, v]) => `- **${k}:** ${v}`).join('\n');
  // The user's name in their own paths stays out of a public page
  let lines = log ? log.split('\n').filter((l) => l.trim()).map((l) => l.replace(/([A-Za-z]:\\Users\\)[^\\\r\n]+/gi, '$1<user>')) : [];
  const make = () => {
    let md;
    if (kind === 'recomp') {
      md = `**Game:** ${title}\n**Original system:** ${system || '?'}\n\n**Why this one?**\n\n${text}\n\n---\n_Sent from NicoLauncher v${snap.version}_`;
    } else if (kind === 'bug') {
      md = `**What happened?**\n\n${text}\n\n**Details**\n\n${facts}`;
      if (lines.length) md += `\n\n<details><summary>Log of the last ${STR.en['act_' + lastFail.action]} (failed)</summary>\n\n\`\`\`\n${lines.join('\n')}\n\`\`\`\n</details>`;
    } else {
      md = `${text}\n\n---\n**About:** ${p ? shortName(p) : 'the launcher'}\n\n${facts}`;
    }
    const name = kind === 'recomp' && system ? `${title} (${system})` : title;
    return `${base}${sep}title=${encodeURIComponent(name)}&body=${encodeURIComponent(md)}`;
  };
  // A long log does not fit in an address: its last lines, where the error is
  let url = make();
  while (url.length > URL_MAX && lines.length > 1) {
    lines = lines.slice(Math.ceil(lines.length / 4));
    url = make();
  }
  return url;
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
  $('dock-report').hidden = !(task.done && !task.ok);
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

// Announcements: notes from the author that are not news of a game (help
// wanted, a call for testers, something important). They come from
// announcements.json in the Hub; the Rust side leaves out those that have
// expired or are for a game that is not installed, and says which ones the
// user has already closed. One not seen yet makes the megaphone in the title
// bar beat; closing the window marks what it showed as seen, so it stays
// quiet until there is a new one. An important one opens by itself.
let annClose = null;
const annAll = () => (snap && snap.announcements) || [];
const annUnseen = () => annAll().filter((a) => !a.seen);

function renderAnnounce() {
  const unseen = annUnseen();
  const b = $('announce');
  b.hidden = !annAll().length;
  b.classList.toggle('unread', unseen.length > 0);
  b.classList.toggle('important', unseen.some((a) => a.level === 'important'));
  b.title = t('ann_title');
  $('announce-label').textContent = unseen.length === 1 ? (unseen[0].label || t('ann_new'))
    : unseen.length ? t('ann_new_many', { n: unseen.length }) : '';
}

// The text is plain: a line is a paragraph, and lines that start with "- "
// make a list. Nothing in it is read as HTML.
function annBody(lines) {
  const out = [];
  let list = null;
  for (const line of lines) {
    if (line.startsWith('- ')) {
      if (!list) { list = el('ul', { class: 'ann-list' }); out.push(list); }
      list.append(el('li', {}, line.slice(2)));
    } else { list = null; out.push(el('p', {}, line)); }
  }
  return out;
}

function openAnnouncements(startId) {
  const all = annAll();
  if (!all.length || annClose) return;
  let i = Math.max(0, all.findIndex((a) => a.id === (startId ?? annUnseen()[0]?.id)));
  const shown = new Set();
  const box = el('div', { class: 'modal ann', role: 'dialog', 'aria-modal': 'true' });
  annClose = async () => {
    annClose = null;
    box.remove();
    const ids = all.filter((a) => shown.has(a.id) && !a.seen).map((a) => a.id);
    if (ids.length) await update('mark_announcements_seen', { ids });
    else renderAnnounce();
  };
  box.addEventListener('click', (e) => { if (e.target === box) annClose(); });
  const draw = () => {
    const a = all[i];
    shown.add(a.id);
    const p = a.game && snap.projects.find((x) => x.id === a.game);
    const head = el('div', { class: 'ann-head' + (p ? ' game' : '') });
    if (p) {
      head.style.backgroundImage = gradient(p);
      if (p.hero) head.append(picture(p.hero, 'ann-hero'));
      if (p.logo) head.append(picture(p.logo, 'ann-logo'));
    } else head.append(el('span', { class: 'ann-art', 'aria-hidden': 'true' }, icon('megaphone')));
    head.append(
      el('div', { class: 'ann-tags' },
        el('span', { class: 'ann-badge' }, icon('megaphone'), a.label || t('ann_label')),
        a.level === 'important' ? el('span', { class: 'ann-badge important' }, t('ann_important')) : null),
      el('button', { class: 'ann-x', 'aria-label': t('close'), onclick: () => annClose() }),
    );
    head.querySelector('.ann-x').innerHTML = '<svg viewBox="0 0 10 10"><path d="M1 1l8 8M9 1L1 9"/></svg>';
    const meta = el('div', { class: 'ann-meta' },
      el('img', { src: 'img/snicolast.png', alt: 'snicolast', class: 'ann-by', draggable: 'false' }),
      el('span', { class: 'dim' }, [a.date ? dateLabel(a.date) : null, p ? p.name : null].filter(Boolean).join(' · ')));
    const links = (a.buttons || []).slice(0, 2).map((btn, n) =>
      el('button', { class: 'btn ' + (n === 0 ? 'primary' : ''), onclick: () => { openLink(btn.url); annClose(); } },
        el('span', {}, btn.label), icon('external')));
    const pager = all.length > 1 ? el('div', { class: 'ann-pager' },
      el('button', { class: 'btn ghost small', 'aria-label': t('ann_prev'), disabled: i === 0, onclick: () => { i--; draw(); } }, icon('chevl')),
      el('span', { class: 'dim small' }, t('ann_of', { i: i + 1, n: all.length })),
      el('button', { class: 'btn ghost small', 'aria-label': t('ann_next'), disabled: i === all.length - 1, onclick: () => { i++; draw(); } }, icon('chevr'))) : el('span');
    box.replaceChildren(el('div', { class: 'modal-box ann-box' + (a.level === 'important' ? ' important' : '') },
      head,
      el('div', { class: 'ann-main' }, meta, el('h2', {}, a.title), el('div', { class: 'ann-text' }, ...annBody(a.body || []))),
      el('div', { class: 'ann-foot' }, pager,
        el('div', { class: 'modal-actions' }, el('button', { class: 'btn ghost', onclick: () => annClose() }, t('close')), ...links.reverse()))));
    (box.querySelector('.btn.primary') || box.querySelector('.ann-foot .btn.ghost:last-child'))?.focus();
  };
  draw();
  document.body.append(box);
}

// After an update: the new version and what it brought. Every news entry
// newer than the version the user had, newest first, so skipping a few
// updates loses nothing. The same window for a game and for the launcher.
let clClose = null;
const CL_MAX = 12;
function changelog({ head, title, from, to, entries, actions, more }) {
  if (clClose) clClose();
  const box = el('div', { class: 'modal ann cl', role: 'dialog', 'aria-modal': 'true' });
  clClose = () => { clClose = null; box.remove(); };
  box.addEventListener('click', (e) => { if (e.target === box) clClose(); });
  const x = el('button', { class: 'ann-x', 'aria-label': t('close'), onclick: () => clClose() });
  x.innerHTML = '<svg viewBox="0 0 10 10"><path d="M1 1l8 8M9 1L1 9"/></svg>';
  head.append(el('div', { class: 'ann-tags' }, el('span', { class: 'ann-badge good' }, icon('check'), t('cl_badge'))), x);
  const count = entries.length === 1 ? t('cl_one') : t('cl_many', { n: entries.length });
  const meta = el('div', { class: 'cl-meta' },
    el('span', { class: 'cl-ver' }, from ? el('span', { class: 'cl-old' }, from) : null,
      from ? icon('chevr') : null, el('span', { class: 'cl-new' }, to)),
    el('span', { class: 'dim' }, count));
  const list = entries.slice(0, CL_MAX).map((e, n) => el('section', { class: 'cl-entry' + (n === 0 ? ' latest' : '') },
    el('h3', {}, el('span', { class: 'cl-num' }, e.label), el('span', { class: 'dim' }, dateLabel(e.title)),
      n === 0 ? el('span', { class: 'chip new' }, t('cl_latest')) : null),
    el('ul', { class: 'ann-list' }, ...e.lines.map((line) => el('li', {}, line)))));
  box.append(el('div', { class: 'modal-box ann-box cl-box' },
    head,
    el('div', { class: 'ann-main' }, el('h2', {}, title), meta, el('div', { class: 'cl-list' }, ...list)),
    el('div', { class: 'ann-foot' },
      el('button', { class: 'btn ghost small', onclick: () => { clClose(); more(); } }, t('cl_all')),
      el('div', { class: 'modal-actions' }, el('button', { class: 'btn ghost', onclick: () => clClose() }, t('close')), ...(actions || [])))));
  document.body.append(box);
  (box.querySelector('.btn.primary') || box.querySelector('.modal-actions .btn'))?.focus();
}

// A game updated by this window: its news since the version it had
function showChangelog(p, from) {
  const entries = p.news.filter((e) => e.number > from);
  const head = el('div', { class: 'ann-head game' });
  head.style.backgroundImage = gradient(p);
  if (p.hero) head.append(picture(p.hero, 'ann-hero'));
  if (p.logo) head.append(picture(p.logo, 'ann-logo'));
  const v = (n) => t('version_short', { n });
  changelog({
    head, title: t('cl_title', { name: p.name }),
    from: from ? v(from) : null, to: v(entries[0].number),
    entries: entries.map((e) => ({ ...e, label: v(e.number) })),
    actions: p.state === 'ready'
      ? [el('button', { class: 'btn primary', onclick: () => { clClose(); call('launch', { id: p.id }); } }, icon('play'), el('span', {}, t('do_play')))]
      : [],
    more: () => openProject(p.id),
  });
}

// The launcher updated itself: on the first start of the new version, what
// it brought (its news entries that start "Launcher 0.3.6:"), once. With
// nothing to show once the news are in, it is marked seen quietly.
let launcherShown = false;
function noticeLauncherUpdate() {
  const up = snap.launcher_update;
  if (!up || launcherShown || !snap.license_accepted || clClose || annClose || closeAsk || licenseBox) return;
  if (!up.changes.length) { if ((snap.launcher_news || []).length) { launcherShown = true; update('mark_launcher_seen'); } return; }
  launcherShown = true;
  const head = el('div', { class: 'ann-head cl-launcher' },
    el('img', { src: 'img/logo.png', alt: 'nico launcher', class: 'cl-mark', draggable: 'false' }));
  const v = (n) => t('version_short', { n });
  changelog({
    head, title: t('cl_title_launcher'),
    from: up.from ? v(up.from) : null, to: v(snap.version),
    entries: up.changes.map((e) => ({ ...e, label: v(e.version) })),
    more: () => show('home'),
  });
  update('mark_launcher_seen');
}

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
  for (const v of ['home', 'library', 'project', 'settings', 'credits', 'feedback']) $('view-' + v).hidden = v !== view;
  $('version').textContent = 'v' + snap.version;
  $('self-update').hidden = !selfUpdate;
  if (selfUpdate) {
    $('self-update').textContent = selfUpdate.installing ? t('self_updating') : t('self_update', { v: selfUpdate.version });
    $('self-update').disabled = !!selfUpdate.installing || !!snap.busy;
    $('self-update').title = selfUpdate.notes || '';
  }
  noticeAchievements();
  renderAnnounce();
  noticeLauncherUpdate();
  renderHome();
  renderLibrary();
  if (view === 'project') renderProject();
  renderSettings();
  renderCredits();
  renderFeedback();
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
for (const n of document.querySelectorAll('[data-url]')) n.addEventListener('click', () => openLink(n.dataset.url));
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
$('announce').addEventListener('click', () => openAnnouncements());
$('lib-refresh').addEventListener('click', () => refresh());
$('home-refresh').addEventListener('click', () => refresh());
$('home-video').addEventListener('click', () => { const u = $('home-video').dataset.url; if (u) openLink(u); });
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
$('dock-report').addEventListener('click', () => feedback('bug', task.project, true));
$('fb-open-list').addEventListener('click', () => openLink(HUB + '/discussions'));
$('lightbox').addEventListener('click', () => { $('lightbox').hidden = true; });
document.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') {
    if (licenseBox) { if (!licenseBox.dataset.must) { licenseBox.remove(); licenseBox = null; } }
    else if (closeAsk) closeAsk();
    else if (clClose) clClose();
    else if (annClose) annClose();
    else if (!$('lightbox').hidden) $('lightbox').hidden = true;
    else if (view === 'project') goBack();
  }
  if (e.key === 'Enter' && document.activeElement.classList.contains('card')) document.activeElement.click();
});
// No browser context menu or image dragging: it is an app, not a page
document.addEventListener('contextmenu', (e) => { if (!e.target.closest('pre, code, dd, .toast, input, textarea')) e.preventDefault(); });

// Project list, news and launcher updates from GitHub. Left open, the
// launcher asks again every so often, so new versions and news show up
// without pressing Refresh; never while something is installing or building.
const AUTO_REFRESH_MS = 20 * 60 * 1000;
let lastRefresh = 0;
let refreshing = false;
const working = () => (task && !task.done) || (snap && snap.busy);

async function refresh() {
  if (refreshing) return;
  refreshing = true;
  try {
    await update('refresh');
    lastRefresh = Date.now();
  } finally { refreshing = false; }
  if (selfUpdate && selfUpdate.installing) return;
  try { selfUpdate = await invoke('check_launcher_update'); render(); } catch (e) { /* offline, or no release yet */ }
}

const refreshDue = () => snap && !working() && Date.now() - lastRefresh >= AUTO_REFRESH_MS;
setInterval(() => { if (refreshDue()) refresh(); }, 60 * 1000);

// Back in the launcher after doing something else (deleting or moving a
// folder in Explorer, say): look at the disk again, and at GitHub too if
// the last look is old
window.addEventListener('focus', () => {
  if (refreshDue()) refresh();
  else if (snap && !working()) update('snapshot');
});

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
    // For a problem report: which task, when, and what it wrote
    if (!payload.ok) lastFail = { project: payload.project, action: task.action, when: new Date().toLocaleTimeString(lang, { hour: '2-digit', minute: '2-digit' }), log: $('dock-log').textContent };
    if (!payload.ok) logOpen = true;
    const finished = task;
    await update('snapshot');
    const l = $('dock-log'); l.scrollTop = l.scrollHeight;
    // An install this window started that now waits for the user's own file:
    // ask for it right away, and build once it is chosen
    const p = snap && snap.projects.find((x) => x.id === payload.project);
    if (payload.ok && finished.own && finished.action === 'setup' && p && p.state === 'needs_input') chooseInput(p, true, finished.fresh);
    // An update with news: what it brought, everything since the version the
    // user had
    else if (payload.ok && finished.own && finished.action === 'update' && p && p.news.some((e) => e.number > finished.fromVersion)) showChangelog(p, finished.fromVersion);
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
  // Windows and graphics card, for the feedback forms
  invoke('system_info').then((x) => { sysInfo = x; render(); }).catch(() => {});
  // Nothing can be done until there is a folder to install into: start where
  // that is asked for
  if (snap && !snap.install_root && view === 'home') show('library');
  if (snap && snap.config_error) toast(`${t('settings_damaged')} ${snap.config_error}`);
  // Project list and news from GitHub; the built-in list shows meanwhile
  await refresh();
  const important = annUnseen().find((a) => a.level === 'important');
  if (important) openAnnouncements(important.id);
})();
