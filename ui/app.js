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
    update_available: 'Update available', update_unfinished: 'Update not finished',
    unfinished_note: 'The last update did not finish, so {name} is still the previous version. Press Update to finish it.',
    do_install: 'Install', do_choose: 'Choose {file}', choose_title: 'Choose your own copy of {file}',
    ask_title: '{name} needs your own file',
    ask_lead: 'The game is built on this PC from your own copy of {file}. This is what happens next:',
    ask_1: 'A window opens to find {file} on your disk. Choose it.',
    ask_2: 'The launcher reads it and builds the game. It takes several minutes, and the first time it also downloads the build tools.',
    ask_3: 'When it ends, the button turns into Play.',
    ask_go: 'Choose the file', ask_later: 'Not now', ask_cancel: 'Cancel the install', install_cancelled: 'Install of {name} cancelled. What had been downloaded was removed.', do_build: 'Build', do_play: 'Play',
    do_update: 'Update',
    shortcut_done: 'Shortcut to {name} added to your desktop.',
    rename_title: 'Rename', rename_label: 'Name', rename_save: 'Save',
    rename_lead: 'Give it the name you like: the launcher shows it everywhere, and new desktop shortcuts use it. The game and its folder do not change.',
    rename_hint: 'Leave it empty to go back to {name}.', renamed: 'Now called {name}.',
    rename_text: 'Show this name instead of the logo',
    m_text_title: 'Show name instead of logo', m_logo: 'Show logo instead of name',
    m_artwork: 'Change artwork…', art_title: 'Artwork', art_save: 'Save', art_saved: 'Artwork saved.',
    art_cover: 'Cover', art_cover_sub: 'Library, Home strip', art_bg: 'Background', art_bg_sub: 'Game page, Home',
    art_drag: 'Drag to choose what shows', art_preparing: 'Preparing the picture…',
    art_side: 'Sidebar', art_strip: 'Home strip', art_home: 'Home', art_blur: 'Also far behind the whole window, blurred, while this game is open.',
    art_pick: 'Pick a picture', art_pick_hint: 'One of the game\'s screenshots, or any JPG, PNG or WebP from your PC. You can also drop one here.',
    art_default: 'Default', art_yours: 'Your picture', art_from_pc: 'From your PC…', art_files: 'Pictures',
    art_same_cover: 'Use it for the background too', art_same_bg: 'Same as the cover', art_reset: 'Back to default',
    art_logo: 'Logo', art_logo_sub: 'Instead of the name', art_logo_pick: 'Pick a logo',
    art_logo_hint: 'A PNG with a see-through background looks best. Pick one from your PC, or drop it here.',
    art_logo_name: 'The name', art_card: 'Library', art_yours_logo: 'Your logo',
    art_icon: 'Icon', art_icon_sub: 'Sidebar, desktop shortcut', art_icon_pick: 'Pick an icon',
    art_icon_hint: 'A screenshot, or any JPG, PNG, WebP or ICO from your PC. A square picture fits best. You can also drop one here.',
    art_icon_game: 'Game icon', art_yours_icon: 'Your icon', art_desk: 'Desktop shortcut', art_sizes: 'Small sizes',
    art_desk_note: 'The shortcut on your desktop changes too. The game\'s own window keeps its icon.',
    installed_title: 'All installed!', installed_body: 'You have just built {name} on your own PC, and it is ready to play. Press Play whenever you want.',
    installed_ask: 'Do you also want a shortcut on your desktop?', shortcut_yes: 'Yes, add it', shortcut_no: 'No, thanks', do_github: 'View on GitHub', do_existing: 'Use a folder I already have',
    details: 'Details', folder: 'Folder', your_file: 'Your file', source: 'Source',
    manage: 'More options', m_open_folder: 'Open game folder', m_shortcut: 'Add desktop shortcut', m_rename: 'Rename…',
    m_change_file: 'Change game file…', m_rebuild: 'Rebuild game', m_rebuild_sub: 'Builds it again from the code', m_uninstall: 'Uninstall…',
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
    un_title: 'Uninstall {name}?',
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
    home_more: 'Show earlier news', home_library: 'Your library', home_open_library: 'Open Library', home_video: 'Latest video on snicolast', home_video_watch: 'Watch on YouTube', home_video_play: 'Play here',
    video_blocked: 'This video can\'t be played inside the launcher.',
    count_ready: '{n} ready to play', count_updates: '{n} with something new', count_todo: '{n} still to set up', count_missing: '{n} not installed',
    home_empty: 'No news could be loaded. Check your connection and press Refresh.',
    today: 'Today', yesterday: 'Yesterday',
    sort_label: 'Order of the games', sort_custom: 'My order', sort_name: 'A–Z', sort_recent: 'Recently played',
    lib_drag: 'Drag a game to move it.', show_hidden: 'Show hidden ({n})',
    m_hide: 'Hide from library', m_hide_sub: 'Also from the sidebar and Home', m_unhide: 'Show in library',
    do_hide: 'Hide', do_unhide: 'Show again', chip_hidden: 'Hidden', hidden_done: '{name} is hidden. Show hidden games from the Library.', undo: 'Undo',
    all_hidden: 'All your games are hidden.',
    played_today: 'Played today', played_yesterday: 'Played yesterday', played_days: 'Played {n} days ago', played_never: 'Not played yet',
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
    fb_launcher: 'Launcher', fb_system: 'Windows', fb_gpu: 'Graphics', portable: 'portable',
    fb_installed: 'Installed', fb_portable: 'Portable',
    fb_rules_title: 'Before you write',
    fb_rule_roms: 'Never share ROMs, game files or links to them. Messages that do are deleted.',
    fb_rule_search: 'Look at what others sent first: maybe it is already there, and a vote helps more than a copy.',
    fb_rule_one: 'One problem or idea per message, so each one can be followed and closed.',
    fb_rule_account: 'Sending needs a free GitHub account. Without one, you can still read everything.',
    ann_title: 'Announcements', ann_new: 'New announcement', ann_new_many: '{n} new announcements', ann_label: 'Announcement',
    ann_important: 'Important', ann_from: 'from snicolast', ann_of: '{i} of {n}', ann_prev: 'Previous', ann_next: 'Next',
    cl_badge: 'Updated', cl_title: 'What is new in {name}', cl_from: 'From version {a} to version {b}', cl_one: '1 update', cl_many: '{n} updates',
    cl_latest: 'Latest', cl_all: 'All the news', cl_title_launcher: 'What is new in NicoLauncher',
    cl_ready: 'Update available', cl_go: 'Update now', cl_later: 'Not now', updated_toast: '{name} is up to date.',
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
    update_available: 'Hay novedades', update_unfinished: 'Actualización a medias',
    unfinished_note: 'La última actualización no terminó, así que {name} sigue siendo la versión anterior. Pulsa Actualizar para terminarla.',
    do_install: 'Instalar', do_choose: 'Elegir {file}', choose_title: 'Elige tu propia copia de {file}',
    ask_title: '{name} necesita tu propio archivo',
    ask_lead: 'El juego se compila en este PC a partir de tu propia copia de {file}. Esto es lo que pasa ahora:',
    ask_1: 'Se abre una ventana para buscar {file} en tu disco. Elígelo.',
    ask_2: 'El launcher lo lee y compila el juego. Tarda varios minutos, y la primera vez además descarga las herramientas de compilación.',
    ask_3: 'Cuando termine, el botón pasa a ser Jugar.',
    ask_go: 'Elegir el archivo', ask_later: 'Ahora no', ask_cancel: 'Cancelar la instalación', install_cancelled: 'Instalación de {name} cancelada. Se ha borrado lo que se había descargado.', do_build: 'Compilar', do_play: 'Jugar',
    do_update: 'Actualizar',
    shortcut_done: 'Acceso directo a {name} añadido a tu escritorio.',
    rename_title: 'Renombrar', rename_label: 'Nombre', rename_save: 'Guardar',
    rename_lead: 'Ponle el nombre que quieras: el launcher lo muestra en todas partes y los accesos directos nuevos lo usan. El juego y su carpeta no cambian.',
    rename_hint: 'Déjalo vacío para volver a {name}.', renamed: 'Ahora se llama {name}.',
    rename_text: 'Mostrar este nombre en vez del logo',
    m_text_title: 'Mostrar el nombre en vez del logo', m_logo: 'Mostrar el logo en vez del nombre',
    m_artwork: 'Cambiar imágenes…', art_title: 'Imágenes', art_save: 'Guardar', art_saved: 'Imágenes guardadas.',
    art_cover: 'Portada', art_cover_sub: 'Biblioteca, tira de Inicio', art_bg: 'Fondo', art_bg_sub: 'Ficha del juego, Inicio',
    art_drag: 'Arrastra para elegir qué se ve', art_preparing: 'Preparando la imagen…',
    art_side: 'Barra lateral', art_strip: 'Tira de Inicio', art_home: 'Inicio', art_blur: 'También detrás de toda la ventana, difuminada, mientras este juego está abierto.',
    art_pick: 'Elige una imagen', art_pick_hint: 'Una captura del juego, o cualquier JPG, PNG o WebP de tu PC. También puedes soltarla aquí.',
    art_default: 'De serie', art_yours: 'Tu imagen', art_from_pc: 'Desde tu PC…', art_files: 'Imágenes',
    art_same_cover: 'Usarla también de fondo', art_same_bg: 'La misma que la portada', art_reset: 'Volver a la de serie',
    art_logo: 'Logo', art_logo_sub: 'En vez del nombre', art_logo_pick: 'Elige un logo',
    art_logo_hint: 'Queda mejor un PNG con el fondo transparente. Elígelo desde tu PC o suéltalo aquí.',
    art_logo_name: 'El nombre', art_card: 'Biblioteca', art_yours_logo: 'Tu logo',
    art_icon: 'Icono', art_icon_sub: 'Barra lateral, acceso directo', art_icon_pick: 'Elige un icono',
    art_icon_hint: 'Una captura, o cualquier JPG, PNG, WebP o ICO de tu PC. Queda mejor una imagen cuadrada. También puedes soltarla aquí.',
    art_icon_game: 'Icono del juego', art_yours_icon: 'Tu icono', art_desk: 'Acceso directo', art_sizes: 'En pequeño',
    art_desk_note: 'El acceso directo del escritorio también cambia. La ventana del juego mantiene su icono.',
    installed_title: '¡Todo instalado!', installed_body: 'Acabas de montar {name} en tu propio PC, y ya está listo para jugar. Pulsa Jugar cuando quieras.',
    installed_ask: '¿Quieres también un acceso directo en el escritorio?', shortcut_yes: 'Sí, añádelo', shortcut_no: 'No, gracias', do_github: 'Ver en GitHub', do_existing: 'Usar una carpeta que ya tengo',
    details: 'Detalles', folder: 'Carpeta', your_file: 'Tu archivo', source: 'Código',
    manage: 'Más opciones', m_open_folder: 'Abrir la carpeta del juego', m_shortcut: 'Crear acceso directo', m_rename: 'Renombrar…',
    m_change_file: 'Cambiar el archivo del juego…', m_rebuild: 'Recompilar el juego', m_rebuild_sub: 'Lo vuelve a compilar desde el código', m_uninstall: 'Desinstalar…',
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
    un_title: '¿Desinstalar {name}?',
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
    home_more: 'Ver noticias anteriores', home_library: 'Tu biblioteca', home_open_library: 'Abrir la Biblioteca', home_video: 'Último vídeo en snicolast', home_video_watch: 'Ver en YouTube', home_video_play: 'Reproducir aquí',
    video_blocked: 'Este vídeo no se puede ver dentro del launcher.',
    count_ready: '{n} listos para jugar', count_updates: '{n} con novedades', count_todo: '{n} a medio preparar', count_missing: '{n} sin instalar',
    home_empty: 'No se han podido cargar las noticias. Comprueba la conexión y pulsa Actualizar lista.',
    today: 'Hoy', yesterday: 'Ayer',
    sort_label: 'Orden de los juegos', sort_custom: 'Mi orden', sort_name: 'A–Z', sort_recent: 'Jugados hace poco',
    lib_drag: 'Arrastra un juego para moverlo.', show_hidden: 'Ver ocultos ({n})',
    m_hide: 'Ocultar de la biblioteca', m_hide_sub: 'También de la barra lateral y de Inicio', m_unhide: 'Mostrar en la biblioteca',
    do_hide: 'Ocultar', do_unhide: 'Volver a mostrar', chip_hidden: 'Oculto', hidden_done: '{name} está oculto. Los ocultos se ven desde la Biblioteca.', undo: 'Deshacer',
    all_hidden: 'Todos tus juegos están ocultos.',
    played_today: 'Jugado hoy', played_yesterday: 'Jugado ayer', played_days: 'Jugado hace {n} días', played_never: 'Sin jugar todavía',
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
    fb_launcher: 'Launcher', fb_system: 'Windows', fb_gpu: 'Gráfica', portable: 'portátil',
    fb_installed: 'Instalado', fb_portable: 'Portátil',
    fb_rules_title: 'Antes de escribir',
    fb_rule_roms: 'Nunca compartas ROMs, archivos del juego ni enlaces a ellos. Los mensajes que lo hagan se borran.',
    fb_rule_search: 'Mira primero lo que otros enviaron: quizá ya está, y un voto ayuda más que una copia.',
    fb_rule_one: 'Un fallo o una idea por mensaje, para poder seguir y cerrar cada uno.',
    fb_rule_account: 'Para enviar hace falta una cuenta gratuita de GitHub. Sin ella puedes leerlo todo igualmente.',
    ann_title: 'Avisos', ann_new: 'Aviso nuevo', ann_new_many: '{n} avisos nuevos', ann_label: 'Aviso',
    ann_important: 'Importante', ann_from: 'de snicolast', ann_of: '{i} de {n}', ann_prev: 'Anterior', ann_next: 'Siguiente',
    cl_badge: 'Actualizado', cl_title: 'Novedades de {name}', cl_from: 'De la versión {a} a la versión {b}', cl_one: '1 actualización', cl_many: '{n} actualizaciones',
    cl_latest: 'Última', cl_all: 'Todas las novedades', cl_title_launcher: 'Novedades de NicoLauncher',
    cl_ready: 'Actualización disponible', cl_go: 'Actualizar ahora', cl_later: 'Ahora no', updated_toast: '{name} está al día.',
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
  type: '<path d="M5 7V5h14v2M12 5v14M9 19h6"/>',
  image: '<rect x="3" y="4" width="18" height="16" rx="2.5"/><circle cx="9" cy="10" r="1.8"/><path d="M21 16.5l-5.5-5.5L5 20"/>',
  move: '<path d="M12 3v18M3 12h18M12 3l-3 3M12 3l3 3M12 21l-3-3M12 21l3-3M3 12l3-3M3 12l3 3M21 12l-3-3M21 12l-3 3"/>',
  pencil: '<path d="M4 20h4L19 9a2.100 2.100 0 0 0-4-4L4 16z"/><path d="M13.500 6.500l4 4"/>',
  gear: '<circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1.1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.5-1.1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3H9a1.7 1.7 0 0 0 1-1.5V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8V9a1.7 1.7 0 0 0 1.5 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1z"/>',
  shortcut: '<path d="M8 18v-5a4 4 0 0 1 4-4h6"/><path d="M14 5l4 4-4 4"/>',
  monitor: '<rect x="3" y="4" width="18" height="12" rx="2"/><path d="M8 20h8M12 16v4"/>',
  eye: '<path d="M2.500 12S6 5.500 12 5.500 21.500 12 21.500 12 18 18.500 12 18.500 2.500 12 2.500 12z"/><circle cx="12" cy="12" r="3"/>',
  eyeoff: '<path d="M10.600 5.600A9.700 9.700 0 0 1 12 5.500c6 0 9.500 6.500 9.500 6.500a17 17 0 0 1-2.600 3.400M6.400 6.900C3.900 8.600 2.500 12 2.500 12S6 18.500 12 18.500c1.900 0 3.500-.6 4.900-1.500"/><path d="M9.900 9.900a3 3 0 0 0 4.200 4.200M3 3l18 18"/>',
  close: '<path d="M6 6l12 12M18 6L6 18"/>',
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

// The cover or background of a project: the user's own picture (from the
// launcher's pictures folder, through the art: protocol), or else the hero
// from its repository. `focus` is the part of the picture kept in sight when a
// frame cuts it.
const artUrl = (file) => T.core.convertFileSrc(file, 'art');
const artSrc = (p, kind) => (p[kind] ? artUrl(p[kind].file) : p.hero);
const artFocus = (p, kind) => (p[kind] ? `${p[kind].focus[0]}% ${p[kind].focus[1]}%` : '');
function artPicture(p, kind, cls) {
  const src = artSrc(p, kind);
  if (!src) return null;
  const img = picture(src, cls);
  img.style.objectPosition = artFocus(p, kind);
  return img;
}

// The game's icon, square: the user's own (cut where they chose), else the
// one the registry names, else its cover
function iconPicture(p, a = p.icon) {
  if (!a && !p.hub_icon) return artPicture(p, 'cover');
  const img = picture(a ? artUrl(a.file) : p.hub_icon);
  if (a) img.style.objectPosition = `${a.focus[0]}% ${a.focus[1]}%`;
  return img;
}

// How a game is named in big (game page, Home, its card, its popups): a
// logo (the user's own, or one its registry entry names), or its name as
// lettering. Lettering when the user chose it, when there is no logo, and
// when the logo does not load (no connection)
const logoSrc = (p) => (p.logo_art ? artUrl(p.logo_art.file) : p.logo);
const showsLogo = (p) => !!logoSrc(p) && !p.text_title;
function gameMark(p, cls, textCls) {
  if (!showsLogo(p)) return titleText(p, textCls);
  const img = el('img', { src: logoSrc(p), alt: '', class: cls || '', draggable: 'false' });
  img.addEventListener('load', () => img.classList.add('in'));
  img.addEventListener('error', () => img.replaceWith(titleText(p, textCls)));
  return img;
}
// The name as a title: the user's own name (or the registry's), large and
// condensed, over a short accent line. "Main: Sub" or "Main - Sub" sets the
// second part as a smaller line under the first. A long name is set smaller
// and may take two lines
function titleText(p, cls) {
  const m = /^(.+?)\s*(?::|\s[-–—])\s+(.+)$/.exec(p.name);
  const main = m ? m[1] : p.name;
  const size = main.length <= 12 ? 'l' : main.length <= 20 ? 'm' : 's';
  return el('div', { class: `wordmark ${size} ${cls || ''}`, role: 'img', 'aria-label': p.name },
    el('span', { class: 'wm-main' }, main),
    m ? el('span', { class: 'wm-sub' }, m[2]) : null);
}

// A message in the corner: something went wrong, or (good) something the
// user asked for is done
// A web page in the user's browser. A failure says so instead of leaving the
// click without any effect
function openLink(url) {
  T.opener.openUrl(url).catch((e) => toast(t('link_failed', { url, error: e })));
}

// The newest video plays first in a player over the launcher, from
// youtube-nocookie.com; YouTube itself (and its comments) is one click away,
// in the user's browser. Closing the player stops the video. A video whose
// owner does not let it play elsewhere says so, with the same way out.
const YT_EMBED = 'https://www.youtube-nocookie.com';
let videoClose = null;
function watchVideo(video) {
  const id = video.id || (/[?&]v=([\w-]{11})/.exec(video.url) || [])[1];
  if (!id || !/^[\w-]{11}$/.test(id)) { openLink(video.url); return; }
  if (videoClose) videoClose();
  const params = new URLSearchParams({ autoplay: '1', rel: '0', playsinline: '1', enablejsapi: '1', hl: lang });
  if (/^https?:/.test(location.origin)) params.set('origin', location.origin);
  const frame = el('iframe', {
    src: `${YT_EMBED}/embed/${id}?${params}`, title: video.title,
    allow: 'autoplay; encrypted-media; picture-in-picture; fullscreen', allowfullscreen: '',
    referrerpolicy: 'strict-origin-when-cross-origin',
  });
  const screen = el('div', { class: 'vp-frame' }, frame);
  const onYouTube = () => { close(); openLink(video.url); };
  const blocked = () => {
    if (screen.classList.contains('blocked')) return;
    screen.classList.add('blocked');
    const img = el('img', { src: video.thumbnail, alt: '', draggable: 'false' });
    img.addEventListener('error', () => img.remove());
    screen.replaceChildren(img, el('div', { class: 'vp-blocked' },
      el('p', {}, t('video_blocked')),
      el('button', { class: 'btn primary', onclick: onYouTube }, icon('external'), t('home_video_watch'))));
  };
  // The player talks back only once told someone listens: what YouTube's own
  // script does, without loading it
  let heard = false, pings = 0, pinger = null;
  const onMessage = (e) => {
    if (e.origin !== YT_EMBED || e.source !== frame.contentWindow) return;
    let data; try { data = JSON.parse(e.data); } catch { return; }
    heard = true;
    if (data && data.event === 'onError') blocked();
  };
  window.addEventListener('message', onMessage);
  frame.addEventListener('load', () => {
    clearInterval(pinger);
    const ping = () => {
      if (heard || ++pings > 40 || !frame.contentWindow) { clearInterval(pinger); return; }
      frame.contentWindow.postMessage(JSON.stringify({ event: 'listening', id: 1, channel: 'widget' }), YT_EMBED);
    };
    pinger = setInterval(ping, 250);
    ping();
  });
  const node = el('div', { class: 'modal video-modal', role: 'dialog', 'aria-modal': 'true', 'aria-label': video.title },
    el('div', { class: 'vp-box' },
      el('div', { class: 'vp-head' },
        el('div', { class: 'vp-title' }, el('strong', {}, video.title),
          el('span', { class: 'dim small' }, [video.date && dayLabel(video.date), 'YouTube'].filter(Boolean).join(' · '))),
        el('button', { class: 'btn ghost small', onclick: onYouTube }, icon('external'), t('home_video_watch')),
        el('button', { class: 'vp-close', title: t('close'), 'aria-label': t('close'), onclick: () => close() }, icon('close'))),
      screen));
  node.addEventListener('click', (e) => { if (e.target === node) close(); });
  function close() {
    if (videoClose !== close) return;
    videoClose = null;
    clearInterval(pinger);
    window.removeEventListener('message', onMessage);
    node.remove();   // the frame goes with it, and the sound stops
  }
  videoClose = close;
  document.body.append(node);
}

function toast(text, good, action) {
  const node = el('div', { class: 'toast' + (good ? ' good' : ''), onclick: () => node.remove() }, text,
    action ? el('button', { class: 'toast-action', onclick: action.go }, action.label) : null);
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
  task.wasPending = !!p.rebuild_pending;
  const started = await call('start_task', { id: p.id, action });
  if (started === undefined) { task = null; clearInterval(timer); }
  await update('snapshot');
}

// The file dialog for a project's own file: titled with what is asked for and
// showing that kind of file first ("swtrilgy.zip (Rev. A)" -> .zip)
async function addShortcut(p) {
  if ((await call('desktop_shortcut', { id: p.id })) !== undefined) toast(t('shortcut_done', { name: p.name }), true);
}

// The user's own name for a project, kept in their settings only
async function rename(p) {
  const input = el('input', { type: 'text', value: p.name, placeholder: p.default_name, maxlength: 40, spellcheck: 'false' });
  // A game with a logo shows another name in it: the new name can show in
  // its place
  const lettering = logoSrc(p) ? el('input', { type: 'checkbox', checked: !!p.text_title }) : null;
  const asked = ask({
    title: t('rename_title'),
    body: [
      el('p', { class: 'dim' }, t('rename_lead')),
      field(t('rename_label'), input),
      el('p', { class: 'dim small rename-hint' }, t('rename_hint', { name: p.default_name })),
      lettering ? el('label', { class: 'check rename-text' }, lettering, el('span', {}, t('rename_text'))) : null,
    ],
    go: [t('rename_save')], later: t('cancel'),
  });
  input.addEventListener('keydown', (e) => {
    if (e.key !== 'Enter') return;
    e.preventDefault();
    input.closest('.modal-box').querySelector('.modal-actions .btn.primary').click();
  });
  input.focus();
  input.select();
  if (!(await asked)) return;
  await saveName(p, input.value);
  if (lettering && lettering.checked !== !!p.text_title) await setTextTitle(p, lettering.checked);
}

function setTextTitle(p, on) {
  return update('set_project_text_title', { id: p.id, on });
}

async function saveName(p, name) {
  const s = await update('set_project_name', { id: p.id, name });
  const now = s && s.projects.find((x) => x.id === p.id);
  if (now && now.name !== p.name) toast(t('renamed', { name: now.name }), true);
}

// ------------------------------------------------------------ artwork
// The user's own cover and background for a project. A picture picked here is
// copied into the launcher's pictures folder at once, so it can be shown; the
// settings change only on Save, and Cancel deletes what was copied. The two
// are linked while they show the same picture: then the background follows
// the cover, until the background is given a picture of its own.
const ART_EXTENSIONS = ['jpg', 'jpeg', 'png', 'webp', 'ico'];
let artDrop = null;   // the open dialog's handler for a file dropped on the window

function artwork(p) {
  // A picture: its file, its focus, and in the dialog only, the screenshot it
  // was made from (by index), to mark it in the strip
  const copy = (a) => (a ? { file: a.file, focus: [...a.focus], screen: a.screen } : null);
  const same = (a, b) => (!a && !b) || (!!a && !!b && a.file === b.file && a.focus[0] === b.focus[0] && a.focus[1] === b.focus[1]);
  const draft = { cover: copy(p.cover), background: copy(p.background), logo: copy(p.logo_art), icon: copy(p.icon) };
  // The picture of the user's own, besides the screenshots, for each tab: the
  // one they had, or the last one they picked from their PC
  const own = { cover: copy(p.cover), background: copy(p.background), logo: copy(p.logo_art), icon: copy(p.icon) };
  let linked = same(p.cover, p.background);
  let tab = 'cover';
  let working = false;
  let problem = null;
  const box = el('div', { class: 'modal art-modal', role: 'dialog', 'aria-modal': 'true' });

  // What a tab shows now: null is the project's own picture (the hero)
  const shown = (kind) => (kind === 'background' && linked ? draft.cover : draft[kind]);
  const set = (kind, art) => {
    if (kind === 'background' && linked) { linked = false; }
    draft[kind] = art;
  };

  async function prepare(args, done) {
    working = true; problem = null; draw();
    try {
      const file = await invoke('prepare_art', { id: p.id, kind: tab, ...args });
      done({ file, focus: [50, 50] });
    } catch (e) {
      problem = String(e);
    }
    working = false; draw();
  }
  const pickScreen = (i) => {
    if (tab !== 'icon' && p.screens[i] === p.hero) { set(tab, null); problem = null; draw(); return; }
    const kind = tab;
    prepare({ screen: i }, (art) => set(kind, { ...art, screen: i }));
  };
  const pickFile = (path) => {
    const kind = tab;
    prepare({ path }, (art) => { set(kind, art); own[kind] = copy(art); });
  };
  const choose = async () => {
    const path = await T.dialog.open({ directory: false, multiple: false, title: t('art_pick'), filters: [{ name: t('art_files'), extensions: ART_EXTENSIONS }] });
    if (path) pickFile(path);
  };
  const close = (saved) => {
    closeAsk = null; artDrop = null; box.remove();
    if (!saved) call('discard_art');
  };
  const save = async () => {
    const strip = (a) => (a ? { file: a.file, focus: a.focus } : null);
    const cover = strip(draft.cover);
    const background = linked ? cover : strip(draft.background);
    const logo = strip(draft.logo);
    const icon = strip(draft.icon);
    working = true; draw();
    try {
      snap = await invoke('set_project_art', { id: p.id, cover, background, logo, icon });
      close(true);
      render();
      toast(t('art_saved'), true);
    } catch (e) {
      working = false; problem = String(e); draw();
    }
  };

  // Dragging the picture moves the part of it in sight. The frame shows it
  // filling it (cover), so it can only move the way it overflows: a picture
  // of the frame's own shape does not move at all.
  const overflow = (frame, img) => {
    if (!img.naturalWidth) return [0, 0];
    const r = frame.getBoundingClientRect();
    const scale = Math.max(r.width / img.naturalWidth, r.height / img.naturalHeight);
    return [img.naturalWidth * scale - r.width, img.naturalHeight * scale - r.height];
  };
  function dragFocus(frame, img, e) {
    if (tab === 'logo') return;
    const a = shown(tab);
    const over = overflow(frame, img);
    if (!a || working || !over.some((v) => v > 2)) return;
    e.preventDefault();
    frame.setPointerCapture(e.pointerId);
    const start = [e.clientX, e.clientY, ...a.focus];
    frame.classList.add('dragging');
    const move = (ev) => {
      const d = [ev.clientX - start[0], ev.clientY - start[1]];
      a.focus = [0, 1].map((i) => (over[i] > 1 ? Math.round(Math.min(100, Math.max(0, start[2 + i] - (d[i] / over[i]) * 100))) : a.focus[i]));
      const pos = `${a.focus[0]}% ${a.focus[1]}%`;
      box.querySelectorAll('.aw-shown').forEach((n) => { n.style.objectPosition = pos; });
    };
    const up = () => {
      frame.classList.remove('dragging');
      frame.removeEventListener('pointermove', move);
      frame.removeEventListener('pointerup', up);
      frame.removeEventListener('pointercancel', up);
    };
    frame.addEventListener('pointermove', move);
    frame.addEventListener('pointerup', up);
    frame.addEventListener('pointercancel', up);
  }

  function draw() {
    // On the logo tab, the logo is shown over the background
    const a = shown(tab === 'logo' ? 'background' : tab);
    const src = a ? artUrl(a.file) : tab === 'icon' && p.hub_icon ? p.hub_icon : p.hero;
    const pos = a ? `${a.focus[0]}% ${a.focus[1]}%` : '';
    const img = (cls) => {
      const n = el('img', { src: src || '', alt: '', class: 'aw-shown ' + (cls || ''), draggable: 'false' });
      n.style.objectPosition = pos;
      if (!src) n.hidden = true;
      return n;
    };
    // The previews show the logo being picked (a new one is shown, not the name)
    const pv = { ...p, logo_art: draft.logo, text_title: draft.logo && !same(draft.logo, p.logo_art) ? false : p.text_title };
    const logo = (cls) => gameMark(pv, cls, cls + '-text');
    const main = img('aw-img');
    const hint = a && tab !== 'logo' && !working ? el('span', { class: 'aw-hint' }, icon('move'), t('art_drag')) : null;
    if (hint) hint.hidden = true;
    const wait = working ? el('span', { class: 'aw-wait' }, el('span', { class: 'spinner' }), t('art_preparing')) : null;
    let stage, also, frame;
    if (tab === 'icon') {
      // Big, where it can be framed, and at the sizes Windows draws it
      frame = el('div', { class: 'aw-icon-big', style: `background-image:${gradient(p)}` }, main, hint, wait);
      stage = el('div', { class: 'aw-icon-stage' }, frame,
        el('div', { class: 'aw-icon-sizes' }, el('div', { class: 'aw-also-label' }, t('art_sizes')),
          el('div', { class: 'aw-icon-row' }, ...[64, 48, 32, 16].map((n) => el('span', { class: `aw-icon-s s${n}`, style: `background-image:${gradient(p)}` }, img())))));
      also = [
        el('div', { class: 'aw-also-label' }, t('art_side')),
        el('div', { class: 'aw-side' }, el('span', { class: 'side-art side-icon', style: `background-image:${gradient(p)}` }, img()), el('span', { class: 'side-name' }, shortName(p)), el('span', { class: 'dot ' + p.state })),
        el('div', { class: 'aw-also-label' }, t('art_desk')),
        el('div', { class: 'aw-desk' }, el('span', { class: 'aw-desk-item' },
          el('span', { class: 'aw-desk-icon', style: `background-image:${gradient(p)}` }, img(), el('span', { class: 'aw-desk-arrow' }, icon('shortcut'))),
          el('span', { class: 'aw-desk-name' }, p.name))),
        el('p', { class: 'dim small aw-also-note' }, t('art_desk_note')),
      ];
    } else if (tab === 'cover') {
      stage = el('div', { class: 'aw-card', style: `background-image:${gradient(p)}` }, main, el('span', { class: 'aw-card-shade' }),
        logo('aw-card-logo'),
        el('div', { class: 'aw-card-foot' }, showsLogo(pv) ? el('h3', {}, p.name) : null, el('div', { class: 'state' }, ...stateLine(p))), hint, wait);
      // The sidebar shows the cover only for a game with no icon at all
      also = [
        p.hub_icon || draft.icon ? null : el('div', { class: 'aw-also-label' }, t('art_side')),
        p.hub_icon || draft.icon ? null : el('div', { class: 'aw-side' }, el('span', { class: 'side-art', style: `background-image:${gradient(p)}` }, img()), el('span', { class: 'side-name' }, shortName(p)), el('span', { class: 'dot ' + p.state })),
        el('div', { class: 'aw-also-label' }, t('art_strip')),
        el('div', { class: 'aw-pick', style: `background-image:${gradient(p)}` }, img()),
      ];
    } else if (tab === 'logo') {
      const cover = shown('cover');
      const coverImg = el('img', { src: cover ? artUrl(cover.file) : p.hero || '', alt: '', class: 'aw-shown', draggable: 'false' });
      if (cover) coverImg.style.objectPosition = `${cover.focus[0]}% ${cover.focus[1]}%`;
      stage = el('div', { class: 'aw-hero', style: `background-image:${gradient(p)}` }, main, el('span', { class: 'aw-hero-shade' }),
        el('div', { class: 'aw-hero-content' }, logo('aw-hero-logo'),
          el('div', { class: 'aw-hero-btns' }, el('span', { class: 'aw-fake-play' }, icon('play'), t('do_play')), el('span', { class: 'aw-fake-gear' }, icon('gear')))),
        wait);
      also = [
        el('div', { class: 'aw-also-label' }, t('art_card')),
        el('div', { class: 'aw-home aw-lib', style: `background-image:${gradient(p)}` }, coverImg, el('span', { class: 'aw-home-shade' }), logo('aw-lib-logo')),
      ];
    } else {
      stage = el('div', { class: 'aw-hero', style: `background-image:${gradient(p)}` }, main, el('span', { class: 'aw-hero-shade' }),
        el('div', { class: 'aw-hero-content' }, logo('aw-hero-logo'),
          el('div', { class: 'aw-hero-btns' }, el('span', { class: 'aw-fake-play' }, icon('play'), t('do_play')), el('span', { class: 'aw-fake-gear' }, icon('gear')))),
        hint, wait);
      also = [
        el('div', { class: 'aw-also-label' }, t('art_home')),
        el('div', { class: 'aw-home', style: `background-image:${gradient(p)}` }, img(), el('span', { class: 'aw-home-shade' }), logo('aw-home-logo')),
        el('p', { class: 'dim small aw-also-note' }, t('art_blur')),
      ];
    }
    frame = frame || stage;
    frame.addEventListener('pointerdown', (e) => dragFocus(frame, main, e));
    // Once the picture is in, whether it can move in this frame
    main.addEventListener('load', () => {
      const movable = !!a && overflow(frame, main).some((v) => v > 2);
      frame.classList.toggle('movable', movable);
      if (hint) hint.hidden = !movable;
    });

    // The screenshots, the user's own picture (when there is one), and a
    // button for another from their PC
    const on = (thumb) => !working && thumb;
    const logoTab = tab === 'logo';
    const iconTab = tab === 'icon';
    // The logo tab: the name (no logo), the user's own logo, one from the PC
    const thumbs = logoTab ? [el('button', { class: 'aw-thumb aw-logo-thumb' + (on(!draft.logo) ? ' on' : ''), disabled: working, onclick: () => { draft.logo = null; problem = null; draw(); } },
      titleText({ ...p, name: shortName(p) }, 'aw-thumb-title'), el('span', { class: 'aw-badge' }, t('art_logo_name')))] : p.screens.map((url, i) => {
      const isDefault = !iconTab && url === p.hero;
      const selected = isDefault ? !a : !!a && a.screen === i;
      return el('button', { class: 'aw-thumb' + (iconTab ? ' aw-icon-thumb' : '') + (on(selected) ? ' on' : ''), disabled: working, title: isDefault ? t('art_default') : '', onclick: () => pickScreen(i) },
        el('img', { src: url, alt: '', draggable: 'false' }),
        isDefault ? el('span', { class: 'aw-badge' }, t('art_default')) : null);
    });
    // The icon tab starts with the game's own icon
    if (iconTab && p.hub_icon) {
      thumbs.unshift(el('button', { class: 'aw-thumb aw-icon-thumb' + (on(!a) ? ' on' : ''), disabled: working, title: t('art_default'), onclick: () => { draft.icon = null; problem = null; draw(); } },
        el('img', { src: p.hub_icon, alt: '', draggable: 'false' }), el('span', { class: 'aw-badge' }, t('art_icon_game'))));
    }
    const mine = own[tab] || (tab === 'background' && linked ? own.cover : null);
    if (mine) {
      const current = shown(tab);
      const selected = !!current && current.screen == null && current.file === mine.file;
      thumbs.push(el('button', { class: 'aw-thumb' + (logoTab ? ' aw-logo-thumb' : iconTab ? ' aw-icon-thumb' : '') + (on(selected) ? ' on' : ''), disabled: working, onclick: () => { set(tab, copy(mine)); problem = null; draw(); } },
        el('img', { src: artUrl(mine.file), alt: '', draggable: 'false' }),
        el('span', { class: 'aw-badge' }, t(logoTab ? 'art_yours_logo' : iconTab ? 'art_yours_icon' : 'art_yours'))));
    }
    thumbs.push(el('button', { class: 'aw-thumb aw-own', disabled: working, onclick: choose }, el('span', { class: 'aw-own-label' }, icon('folder'), t('art_from_pc'))));

    const check = el('input', { type: 'checkbox' });
    check.checked = linked;
    check.addEventListener('change', () => {
      if (check.checked) linked = true; else { linked = false; draft.background = copy(draft.cover); }
      draw();
    });
    const tabButton = (kind, label, sub) => el('button', { class: tab === kind ? 'on' : '', role: 'tab', 'aria-selected': String(tab === kind), onclick: () => { tab = kind; problem = null; draw(); } },
      label, el('small', {}, sub));
    const custom = !!shown(tab);
    const reset = () => {
      if (tab === 'background') set('background', null); else draft[tab] = null;
      problem = null; draw();
    };
    box.replaceChildren(el('div', { class: 'modal-box aw-box' },
      el('div', { class: 'aw-head' },
        el('h2', {}, t('art_title'), el('span', { class: 'aw-of' }, p.name)),
        el('div', { class: 'aw-seg', role: 'tablist' }, tabButton('cover', t('art_cover'), t('art_cover_sub')), tabButton('background', t('art_bg'), t('art_bg_sub')), tabButton('logo', t('art_logo'), t('art_logo_sub')), tabButton('icon', t('art_icon'), t('art_icon_sub')))),
      el('div', { class: 'aw-stage' }, stage, el('div', { class: 'aw-also' }, ...also)),
      el('div', { class: 'aw-src' },
        el('div', { class: 'aw-src-head' }, el('span', { class: 'aw-label' }, t(logoTab ? 'art_logo_pick' : iconTab ? 'art_icon_pick' : 'art_pick')), el('span', { class: 'dim small' }, t(logoTab ? 'art_logo_hint' : iconTab ? 'art_icon_hint' : 'art_pick_hint'))),
        el('div', { class: 'aw-strip' }, ...thumbs),
        problem ? el('div', { class: 'aw-problem' }, icon('ban'), el('span', {}, problem)) : null),
      el('div', { class: 'modal-actions aw-actions' },
        el('button', { class: 'btn ghost', disabled: !custom || working, onclick: reset }, icon('refresh'), t('art_reset')),
        logoTab || iconTab ? null : el('label', { class: 'aw-check' }, check, el('span', {}, t(tab === 'cover' ? 'art_same_cover' : 'art_same_bg'))),
        el('span', { class: 'aw-gap' }),
        el('button', { class: 'btn ghost', onclick: () => close(false) }, t('cancel')),
        el('button', { class: 'btn primary', disabled: working, onclick: save }, t('art_save')))));
  }

  artDrop = (path) => { if (!working) pickFile(path); };
  closeAsk = () => close(false);
  draw();
  document.body.append(box);
}

// A picture dropped on the window goes to the artwork dialog, when it is open
T.event.listen('tauri://drag-enter', () => { if (artDrop) document.querySelector('.art-modal')?.classList.add('drop'); });
T.event.listen('tauri://drag-leave', () => document.querySelector('.art-modal')?.classList.remove('drop'));
T.event.listen('tauri://drag-drop', (e) => {
  document.querySelector('.art-modal')?.classList.remove('drop');
  const path = e.payload && e.payload.paths && e.payload.paths[0];
  if (artDrop && path) artDrop(path);
});

// The sidebar's "...": a small box beside the game with only its name in it.
// Enter or the tick keeps it; Escape or a click elsewhere leaves it as it was.
let renaming = null;
function quickRename(p, anchor) {
  if (closeAsk) closeAsk();
  const input = el('input', { type: 'text', value: p.name, placeholder: p.default_name, maxlength: 40, spellcheck: 'false', 'aria-label': t('rename_label') });
  const ok = el('button', { class: 'rename-ok', title: t('rename_save'), 'aria-label': t('rename_save') }, icon('check'));
  const box = el('div', { class: 'rename-pop', role: 'dialog' }, icon('pencil'), input, ok);
  const r = anchor.closest('.side-row').getBoundingClientRect();
  box.style.left = `${r.right + 8}px`;
  box.style.top = `${r.top + r.height / 2}px`;
  const away = (e) => { if (!box.contains(e.target)) close(); };
  const close = () => {
    closeAsk = null;
    renaming = null;
    box.remove();
    document.removeEventListener('mousedown', away, true);
    render();
  };
  const keep = () => { const name = input.value; close(); saveName(p, name); };
  ok.addEventListener('click', keep);
  input.addEventListener('keydown', (e) => { if (e.key === 'Enter') { e.preventDefault(); keep(); } });
  document.addEventListener('mousedown', away, true);
  closeAsk = close;
  renaming = p.id;
  document.body.append(box);
  render();
  input.focus();
  input.select();
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
    default: return make('play', t('do_play'), async () => { if ((await call('launch', { id: p.id })) !== undefined) update('snapshot'); }, 'play');
  }
}

// Something new is waiting for an installed project: news it does not have,
// or the rebuild of an update that was cancelled or failed
const hasUpdate = (p) => p.can_update && (p.new_entries.length > 0 || p.rebuild_pending);
const updateChip = (p) => (p.new_entries.length ? el('span', { class: 'chip new' }, t('update_available'))
  : hasUpdate(p) ? el('span', { class: 'chip new' }, t('update_unfinished')) : null);

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

// The date of a news title, or null when the title is something else; and
// how many days ago it was
function dayOf(title) {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(dayKey(title));
  return m ? new Date(+m[1], +m[2] - 1, +m[3]) : null;
}
function daysAgo(day) {
  const now = new Date();
  return Math.round((new Date(now.getFullYear(), now.getMonth(), now.getDate()) - day) / 86400000);
}
const capital = (text) => text.charAt(0).toUpperCase() + text.slice(1);

function dayLabel(title) {
  const day = dayOf(title);
  if (!day) return title;
  const ago = daysAgo(day);
  if (ago === 0) return t('today');
  if (ago === 1) return t('yesterday');
  const opts = { weekday: 'long', day: 'numeric', month: 'long' };
  if (day.getFullYear() !== new Date().getFullYear()) opts.year = 'numeric';
  return capital(day.toLocaleDateString(lang, opts));
}

// The heading of a day of news: a calendar leaf (month and day) on the
// timeline, and beside it the weekday (or Today, Yesterday) and the date, so
// one day stands apart from the next at a glance. A title that is not a date
// gets a plain mark on the timeline and the title as it is
function dayHead(title, ...extra) {
  const day = dayOf(title);
  if (!day) return el('h3', { class: 'day' }, el('span', { class: 'day-cal blank', 'aria-hidden': 'true' }), el('span', { class: 'day-text' }, el('span', { class: 'day-name' }, title), ...extra));
  const ago = daysAgo(day);
  const opts = { day: 'numeric', month: 'long' };
  if (day.getFullYear() !== new Date().getFullYear()) opts.year = 'numeric';
  return el('h3', { class: 'day' + (ago <= 1 ? ' recent' : '') },
    el('span', { class: 'day-cal', 'aria-hidden': 'true' },
      el('i', {}, day.toLocaleDateString(lang, { month: 'short' }).replace('.', '')),
      el('b', {}, String(day.getDate()))),
    el('span', { class: 'day-text' },
      el('span', { class: 'day-name' }, ago === 0 ? t('today') : ago === 1 ? t('yesterday') : capital(day.toLocaleDateString(lang, { weekday: 'long' }))),
      el('span', { class: 'day-date' }, capital(day.toLocaleDateString(lang, opts))),
      ...extra));
}

// A picture that fills `box`, changed only when it is another one: the page
// is drawn again at every change of state, and the picture must not blink
// The new picture fades in over the one before it, which then goes away.
function backdrop(box, src, focus) {
  const key = (src || '') + (src && focus ? ' ' + focus : '');
  if (box.dataset.src === key) return false;
  box.dataset.src = key;
  const old = [...box.children];
  if (src) {
    const img = picture(src);
    img.style.objectPosition = focus || '';
    box.append(img);
  }
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
  const list = listed();
  // Not while something else has the eye: another page, a question on screen,
  // the pointer on the spotlight, or the window out of sight or in the
  // background (a game in front of it)
  if (list.length < 2 || view !== 'home' || document.hidden || !document.hasFocus() || document.querySelector('.modal') || $('view-home').querySelector('.spot:hover')) return;
  const at = list.findIndex((p) => p.id === $('spot-bg').dataset.id);
  spot = list[(at + 1) % list.length].id;
  renderHome();
}
function spotRestart() {
  clearInterval(spotTimer);
  spotTimer = setInterval(spotNext, SPOT_EVERY);
}
spotRestart();

// The games the user did not hide, in their order: what Home, the sidebar and
// the Library list
const listed = () => (snap ? snap.projects.filter((p) => !p.hidden) : []);

function renderHome() {
  const games = listed();
  const items = [];
  for (const e of snap.launcher_news || []) items.push({ ...e, day: dayKey(e.title), order: -1, from: null });
  games.forEach((p, order) => {
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
  const lead = games.find((p) => p.id === spot) || (newest && newest.from) || games[0];
  const open = (p) => (!snap.install_root && !p.dir ? show('library') : openProject(p.id));
  backdrop($('spot-bg'), lead && artSrc(lead, 'background'), lead && artFocus(lead, 'background'));
  const swapped = $('spot-bg').dataset.id !== (lead ? lead.id : '');
  $('spot-bg').dataset.id = lead ? lead.id : '';
  $('spot-content').classList.remove('swap');
  if (swapped) { void $('spot-content').offsetWidth; $('spot-content').classList.add('swap'); }
  if (lead) {
    $('spot-bg').style.backgroundImage = gradient(lead);
    const last = items.find((e) => e.from === lead);
    $('spot-content').replaceChildren(
      el('div', { class: 'hero-logo' }, gameMark(lead, 'in', 'spot-title')),
      el('p', { class: 'tagline' }, lead.tagline),
      last ? el('p', { class: 'spot-news' }, el('b', {}, dayLabel(last.title)), last.lines[0] || '') : null,
      el('div', { class: 'hero-actions' },
        !snap.install_root && !lead.dir
          ? el('button', { class: 'btn primary lg', onclick: () => show('library') }, icon('download'), t('do_install'))
          : primaryButton(lead, true),
        el('button', { class: 'btn lg glass', onclick: () => open(lead) }, t('details')),
        el('span', { class: 'state' }, ...stateLine(lead)),
        updateChip(lead)));
  } else $('spot-content').replaceChildren();
  $('spot-picks').classList.toggle('dots', games.length > 5);
  $('spot-picks').replaceChildren(...(games.length > 1 ? games.map((p) =>
    el('button', { class: 'pick' + (p === lead ? ' on' : ''), title: p.name, style: `background-image:${gradient(p)}`, onclick: () => { spot = p.id; spotRestart(); render(); } },
      artPicture(p, 'cover'))) : []));

  const block = (b) => el('div', { class: 'feed-block' + (b.from ? ' link' : ''), onclick: b.from ? () => open(b.from) : null },
    el('div', { class: 'feed-from' },
      !b.from ? el('span', { class: 'mark', 'aria-hidden': 'true' }) : showsLogo(b.from) ? picture(logoSrc(b.from), 'feed-logo') : null,
      el('strong', {}, b.from ? b.from.name : 'NicoLauncher'),
      b.isNew ? el('span', { class: 'chip new' }, t('update_available')) : null),
    el('ul', {}, ...b.lines.map((l) => el('li', {}, l))),
    b.had.length ? el('div', { class: 'feed-had' }, el('span', { class: 'feed-had-label' }, icon('check'), t('news_installed'))) : null,
    b.had.length ? el('ul', { class: 'had' }, ...b.had.map((l) => el('li', {}, l))) : null);
  $('home-feed').replaceChildren(...(days.length
    ? days.map((d) => el('div', { class: 'entry' }, dayHead(d.title), el('div', { class: 'day-body' }, ...d.blocks.map(block))))
    : [el('p', { class: 'dim' }, t('home_empty'))]));
  $('home-feed').classList.toggle('timeline', days.length > 0);

  // Beside the news: the pictures of the project in the spotlight (changed
  // only with it, so they do not blink), the library in numbers and the links
  let shots = lead ? (lead.screens && lead.screens.length ? lead.screens : lead.hero ? [lead.hero] : []).slice(0, 4) : [];
  if (shots.length === 3) shots.pop();
  // (its name follows a rename at once)
  $('shots-title').textContent = lead ? lead.name : '';
  if ($('home-shots').dataset.id !== (lead ? lead.id : '')) {
    $('home-shots').dataset.id = lead ? lead.id : '';
    $('home-shots').replaceChildren(...shots.map((src) => {
      const img = picture(src);
      img.addEventListener('click', () => { $('lightbox-img').src = src; $('lightbox').hidden = false; });
      return img;
    }));
  }
  const count = (state) => games.filter((p) => p.state === state).length;
  const fresh = games.filter(hasUpdate);
  const todo = count('needs_input') + count('needs_build');
  const line = (dot, key, n) => (n ? el('div', { class: 'state' }, el('span', { class: 'dot ' + dot }), t(key, { n })) : null);
  $('home-library').replaceChildren(...[
    line('ready', 'count_ready', count('ready')),
    line('needs_build', 'count_todo', todo),
    line('', 'count_missing', count('not_installed')),
    ...fresh.map((p) => el('div', { class: 'fresh' },
      el('span', {}, el('strong', {}, p.name.replace(/ Native$/, '')), updateChip(p)),
      el('button', { class: 'btn small attn', disabled: !!snap.busy, onclick: () => askUpdate(p) }, icon('up'), t('do_update')))),
    el('button', { class: 'btn ghost small', onclick: () => show('library') }, t('home_open_library')),
  ].filter(Boolean));

  // The author's newest video: rebuilt only when it is another one
  const video = snap.latest_video;
  $('home-video-panel').hidden = !video;
  if (video && $('home-video').dataset.url !== video.url) {
    $('home-video').dataset.url = video.url;
    $('home-video').title = t('home_video_play');
    const img = el('img', { src: video.thumbnail, alt: '', draggable: 'false' });
    img.addEventListener('error', () => img.remove());
    $('home-video').replaceChildren(
      el('span', { class: 'video-art' }, img, el('span', { class: 'video-play' }, icon('play'))),
      el('span', { class: 'video-text' }, el('strong', {}, video.title),
        el('span', { class: 'dim small' }, [video.date && dayLabel(video.date), 'YouTube'].filter(Boolean).join(' · '))));
  }

  // The sidebar's shortcuts, and the picture far behind everything
  // Each with a "..." that shows on hover: a quick rename
  $('side-games').replaceChildren(...games.map((p) => {
    const more = el('button', { class: 'side-more', title: t('m_rename'), 'aria-label': t('m_rename') }, el('span'), el('span'), el('span'));
    more.addEventListener('click', (e) => { e.stopPropagation(); quickRename(p, more); });
    return el('div', { class: 'side-row' + (renaming === p.id ? ' renaming' : '') },
      el('button', { class: 'side-game' + (view === 'project' && current === p.id ? ' on' : ''), title: p.name, onclick: () => open(p) },
        el('span', { class: 'side-art side-icon', style: `background-image:${gradient(p)}` }, iconPicture(p)),
        el('span', { class: 'side-name' }, p.name.replace(/ Native$/, '')),
        el('span', { class: 'dot ' + (snap.busy === p.id ? 'busy' : hasUpdate(p) ? 'fresh' : p.state) })),
      more);
  }));
  const behind = view === 'project' ? snap.projects.find((p) => p.id === current) : lead;
  backdrop($('ambient'), behind && artSrc(behind, 'background'), behind && artFocus(behind, 'background'));
}

let showHidden = false;
function renderLibrary() {
  if (dragging) return;   // the cards stay as they are under the pointer
  $('first-run').hidden = !!snap.install_root;
  $('first-run-title').textContent = snap.missing_root ? t('root_gone_title') : t('first_run_title');
  $('first-run-text').textContent = snap.missing_root ? t('root_gone', { path: snap.missing_root }) : t('first_run');
  const hidden = snap.projects.filter((p) => p.hidden).length;
  if (!hidden) showHidden = false;
  const games = showHidden ? snap.projects : listed();
  const ready = snap.projects.filter((p) => p.state === 'ready').length;
  // The user's own order is changed by dragging: said once, under the title
  const movable = snap.sort === 'custom' && games.length > 1 && !!snap.install_root;
  $('lib-sub').textContent = t('lib_sub', { n: snap.projects.length, ready }) + (movable ? ' · ' + t('lib_drag') : '');

  // How they are listed, and the way back to the hidden ones
  $('lib-tools').replaceChildren(
    hidden ? el('button', { class: 'btn ghost toggle' + (showHidden ? ' on' : ''), 'aria-pressed': String(showHidden), onclick: () => { showHidden = !showHidden; render(); } },
      icon(showHidden ? 'eye' : 'eyeoff'), t('show_hidden', { n: hidden })) : '',
    el('div', { class: 'seg', role: 'radiogroup', 'aria-label': t('sort_label') }, ...['custom', 'name', 'recent'].map((sort) =>
      el('button', { type: 'button', role: 'radio', 'aria-checked': String(snap.sort === sort), class: snap.sort === sort ? 'on' : '',
        onclick: () => { if (snap.sort !== sort) update('set_library_sort', { sort }); } }, t('sort_' + sort)))));

  const cards = $('cards');
  cards.classList.toggle('movable', movable);
  if (!games.length) {
    cards.replaceChildren(el('div', { class: 'empty-lib' }, el('p', { class: 'dim' }, t('all_hidden')),
      el('button', { class: 'btn ghost', onclick: () => { showHidden = true; render(); } }, icon('eye'), t('show_hidden', { n: hidden }))));
    return;
  }
  cards.replaceChildren();
  for (const p of games) {
    const art = el('div', { class: 'art', style: `background-image:${gradient(p)}` });
    const cover = artPicture(p, 'cover');
    if (cover) art.append(cover);
    // Hide (or show again) right on the card, when the pointer is on it
    const eye = el('button', { class: 'card-eye', title: t(p.hidden ? 'do_unhide' : 'do_hide'), 'aria-label': t(p.hidden ? 'do_unhide' : 'do_hide') },
      icon(p.hidden ? 'eye' : 'eyeoff'));
    eye.addEventListener('click', (e) => { e.stopPropagation(); setHidden(p, !p.hidden); });
    eye.addEventListener('pointerdown', (e) => e.stopPropagation());
    const top = el('div', { class: 'card-top' },
      gameMark(p, 'card-logo', 'card-title'),
      el('div', { class: 'card-flags' }, p.hidden ? el('span', { class: 'chip' }, t('chip_hidden')) : null, updateChip(p), eye));
    // Until there is a folder to install into, the projects are only shown:
    // the welcome box above is the one thing to do. A project that already
    // has a folder of its own stays in use. A card has no button of its own:
    // it opens the project's page, where everything is done
    const locked = !snap.install_root && !p.dir;
    const card = locked
      ? { class: 'card locked', 'aria-disabled': 'true', title: t('first_run') }
      : { class: 'card', tabindex: '0', onclick: () => { if (!dragged) openProject(p.id); } };
    // The name in big at the top already: not again at the bottom
    if (!showsLogo(p)) card.class += ' titled';
    if (p.hidden) card.class += ' hidden-game';
    const node = el('div', card,
      art, top,
      el('div', { class: 'card-foot' },
        el('div', {}, el('h3', {}, p.name), el('div', { class: 'state' }, ...stateLine(p)),
          snap.sort === 'recent' ? el('div', { class: 'played dim small' }, playedText(p)) : null),
        achBadge(p)));
    node.dataset.id = p.id;
    if (!locked) node.addEventListener('keydown', (e) => { if (e.key === 'Enter') openProject(p.id); else if (movable && e.ctrlKey && /^Arrow(Left|Right|Up|Down)$/.test(e.key)) { e.preventDefault(); nudge(p.id, /Left|Up/.test(e.key) ? -1 : 1); } });
    if (movable && !locked) node.addEventListener('pointerdown', (e) => startDrag(e, node));
    cards.append(node);
  }
}

function playedText(p) {
  if (!p.last_played) return t('played_never');
  const d = new Date(p.last_played * 1000);
  const ago = daysAgo(new Date(d.getFullYear(), d.getMonth(), d.getDate()));
  return ago <= 0 ? t('played_today') : ago === 1 ? t('played_yesterday') : t('played_days', { n: ago });
}

// Hidden: out of the library, the sidebar and Home. Undone from the toast,
// the card (with "Show hidden") or the game's gear menu
async function setHidden(p, hidden) {
  const s = await update('set_project_hidden', { id: p.id, hidden });
  if (s && hidden) toast(t('hidden_done', { name: p.name }), true, { label: t('undo'), go: () => update('set_project_hidden', { id: p.id, hidden: false }) });
}

// ------------------------------------------------------------ the user's order
// In "My order" a card is moved by dragging it (with the pointer, not the
// browser's drag and drop: the window takes that for dropping files). The
// others slide out of its way; letting go keeps the order. Ctrl+arrows move
// the focused card. Hidden games keep their place among the others.
let dragged = false;    // a drag just ended: the click that follows is not a click
let dragging = null;
const fullOrder = (shownIds) => {
  // The ids on screen in their new order, with the ones not on screen (hidden)
  // put back after the game they followed
  const out = [...shownIds];
  snap.projects.forEach((p, i) => {
    if (out.includes(p.id)) return;
    const before = snap.projects.slice(0, i).reverse().find((x) => out.includes(x.id));
    out.splice(before ? out.indexOf(before.id) + 1 : 0, 0, p.id);
  });
  return out;
};
const saveOrder = (ids) => update('set_library_order', { ids: fullOrder(ids) });

function nudge(id, by) {
  const ids = [...$('cards').children].map((n) => n.dataset.id).filter(Boolean);
  const at = ids.indexOf(id);
  const to = at + by;
  if (at < 0 || to < 0 || to >= ids.length) return;
  ids.splice(to, 0, ids.splice(at, 1)[0]);
  saveOrder(ids).then(() => $('cards').querySelector(`[data-id="${id}"]`)?.focus());
}

// Where each card is now, to slide them from there to their new place
function slide(cards, move) {
  const at = (c) => c.getBoundingClientRect();
  const before = new Map(cards.map((c) => [c, at(c)]));
  move();
  for (const c of cards) {
    if (c === dragging?.node) continue;
    c.getAnimations().forEach((x) => x.cancel());
    const a = before.get(c), b = at(c);
    if (!a || (a.left === b.left && a.top === b.top)) continue;
    c.animate([{ transform: `translate(${a.left - b.left}px, ${a.top - b.top}px)` }, { transform: 'none' }], { duration: 220, easing: 'cubic-bezier(.2, .8, .2, 1)' });
  }
}

function startDrag(e, node) {
  if (e.button !== 0) return;
  const start = { x: e.clientX, y: e.clientY };
  let moved = false;
  const box = $('cards');
  const zoom = node.getBoundingClientRect().width / node.offsetWidth || 1;
  // Where a card's slot is on screen: from the layout, not from where it is
  // drawn now (the others are still sliding, the dragged one follows the pointer)
  const slot = (c) => {
    const r = box.getBoundingClientRect();
    return { left: r.left + c.offsetLeft * zoom, top: r.top + c.offsetTop * zoom, w: c.offsetWidth * zoom, h: c.offsetHeight * zoom };
  };
  let grab = null;
  const follow = (ev) => {
    const home = slot(node);
    node.style.transform = `translate(${(ev.clientX - grab.x - home.left) / zoom}px, ${(ev.clientY - grab.y - home.top) / zoom}px) scale(1.03)`;
  };
  const move = (ev) => {
    if (!moved) {
      if (Math.hypot(ev.clientX - start.x, ev.clientY - start.y) < 6) return;
      moved = true;
      const home = slot(node);
      grab = { x: start.x - home.left, y: start.y - home.top };
      dragging = { node };
      node.classList.add('dragging');
      box.classList.add('sorting');
    }
    // Over another card's slot: it takes that card's place
    const cards = [...box.children];
    const over = cards.find((c) => {
      if (c === node) return false;
      const r = slot(c);
      return ev.clientX > r.left && ev.clientX < r.left + r.w && ev.clientY > r.top && ev.clientY < r.top + r.h;
    });
    if (over) {
      const after = cards.indexOf(over) > cards.indexOf(node);
      slide(cards, () => over[after ? 'after' : 'before'](node));
    }
    follow(ev);
  };
  const up = () => {
    document.removeEventListener('pointermove', move);
    document.removeEventListener('pointerup', up);
    document.removeEventListener('pointercancel', up);
    if (!moved) return;
    // Back into its slot, then the order is kept
    const from = node.style.transform;
    node.style.transform = '';
    node.animate([{ transform: from }, { transform: 'none' }], { duration: 200, easing: 'cubic-bezier(.2, .8, .2, 1)' });
    node.classList.remove('dragging');
    $('cards').classList.remove('sorting');
    dragging = null;
    dragged = true;
    setTimeout(() => { dragged = false; }, 0);
    const ids = [...$('cards').children].map((n) => n.dataset.id).filter(Boolean);
    if (ids.join() !== listedIds().join()) saveOrder(ids);
  };
  const listedIds = () => (showHidden ? snap.projects : listed()).map((p) => p.id);
  document.addEventListener('pointermove', move);
  document.addEventListener('pointerup', up);
  document.addEventListener('pointercancel', up);
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
    $('p-screens').replaceChildren(...p.screens.map((src) => {
      const img = picture(src);
      img.classList.add('in');
      img.addEventListener('click', () => { $('lightbox-img').src = src; $('lightbox').hidden = false; });
      return img;
    }));
    $('view-project').scrollTop = 0;
    achSeen = markAchievementsSeen(p);
  }
  backdrop($('p-bg'), artSrc(p, 'background'), artFocus(p, 'background'));
  // The logo or the name in big, drawn again only when that changes
  const mark = showsLogo(p) ? 'logo ' + logoSrc(p) : 'text ' + p.name;
  if ($('p-logo').dataset.mark !== mark) {
    $('p-logo').dataset.mark = mark;
    $('p-logo').replaceChildren(gameMark(p, '', 'page-title'));
  }
  $('p-screens-block').hidden = !p.screens.length;
  $('p-name').textContent = p.name;
  $('p-name').hidden = !!$('p-logo').children.length;
  $('p-tagline').textContent = p.tagline;
  $('p-desc').textContent = p.description;
  $('p-tags').replaceChildren(...p.tags.map((tag) => el('span', { class: 'chip' }, tag)),
    p.hidden ? el('button', { class: 'chip hidden-chip', title: t('m_unhide'), onclick: () => setHidden(p, false) }, icon('eyeoff'), t('chip_hidden')) : '');
  // Beside the state, the number only; the date and commit are in Details
  const vn = p.version?.number;
  $('p-status').replaceChildren(...stateLine(p), ...(vn != null ? [el('span', { class: 'sep' }, '·'), el('span', { class: 'nowrap' }, t('version_short', { n: vn }))] : []));

  const btn = (ic, label, fn, cls = '') => el('button', { class: 'btn ' + cls, disabled: busy, onclick: fn }, ic ? icon(ic) : null, label);
  const actions = [primaryButton(p, true)];
  if (p.can_update) actions.push(btn('up', t('do_update'), () => askUpdate(p), 'lg' + (hasUpdate(p) ? ' attn' : ' idle')));
  // Everything else one does with a game is in the menu behind the gear
  manageButton.title = t('manage');
  manageButton.setAttribute('aria-label', t('manage'));
  actions.push(manageButton);
  $('p-actions').replaceChildren(...actions);
  if (manageMenu) {
    if (manageMenu.project === p.id) fillManageMenu(p); else closeManageMenu();
  }
  // Play would start the old version until the update is finished
  $('p-note').hidden = !p.rebuild_pending;
  $('p-note').replaceChildren(...(p.rebuild_pending ? [icon('up'), t('unfinished_note', { name: p.name })] : []));

  // Details are facts only; the one link is the code on GitHub
  const installed = p.state !== 'not_installed';
  const details = $('p-details');
  details.replaceChildren(
    el('dt', {}, t('folder')), el('dd', {}, installed ? p.dir : t('not_installed_yet')),
    el('dt', {}, t('source')), el('dd', {}, el('a', { class: 'source', href: '#', title: t('do_github'),
      onclick: (e) => { e.preventDefault(); openLink(p.url); } }, p.url.replace('https://', ''), icon('external'))));
  if (installed && p.version) details.append(el('dt', {}, t('version')), versionDetail(p));
  if (p.input_label) {
    details.append(el('dt', {}, t('your_file')), el('dd', {}, p.input || `${p.input_label} · ${t('not_chosen')}`));
    details.append(el('div', { class: 'note' }, icon('shield'), t('input_note')));
  }

  $('p-news-title').textContent = t('news');
  const news = $('p-news');
  const entry = (e) => el('div', { class: 'entry' + (e.isNew ? ' new' : '') },
    dayHead(e.title, e.isNew ? el('span', { class: 'chip new' }, t('news_new')) : null),
    el('div', { class: 'day-body' },
      el('ul', {}, ...e.lines.map((l) => el('li', {}, l))),
      e.had.length ? el('div', { class: 'feed-had' }, el('span', { class: 'feed-had-label' }, icon('check'), t('news_installed'))) : null,
      e.had.length ? el('ul', { class: 'had' }, ...e.had.map((l) => el('li', {}, l))) : null));
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

// ------------------------------------------------------------ game menu
// The gear beside Play: the rest of what one does with a game, as a menu
// (open its folder, a shortcut, rename, change the file, rebuild, GitHub,
// report a problem, uninstall). One button for the whole life of the page,
// so a menu that is open stays put while the page is drawn again
const manageButton = el('button', { class: 'btn lg icon-only manage', 'aria-haspopup': 'menu', 'aria-expanded': 'false',
  onclick: (e) => { e.stopPropagation(); if (manageMenu) closeManageMenu(); else openManageMenu(e.detail === 0); } }, icon('gear'));
let manageMenu = null;   // the open menu, with .project = the id it is for

function manageItems(p) {
  const busy = !!snap.busy;
  const installed = p.state !== 'not_installed';
  const file = p.input ? p.input.split(/[\\/]/).pop() : p.input_label;
  const groups = [[], [], [], []];
  if (installed) groups[0].push(['folder', t('m_open_folder'), null, busy, () => call('open_folder', { id: p.id })]);
  if (p.state === 'ready') groups[0].push(['monitor', t('m_shortcut'), null, busy, () => addShortcut(p)]);
  else groups[0].push(['folder', t('do_existing'), null, busy, () => chooseExisting(p)]);
  // Only a setting: it can change while a task runs
  groups[0].push(['pencil', t('m_rename'), null, false, () => rename(p)]);
  groups[0].push(['image', t('m_artwork'), null, false, () => artwork(p)]);
  if (logoSrc(p)) groups[0].push(['type', t(p.text_title ? 'm_logo' : 'm_text_title'), null, false, () => setTextTitle(p, !p.text_title)]);
  groups[0].push(p.hidden ? ['eye', t('m_unhide'), null, false, () => setHidden(p, false)] : ['eyeoff', t('m_hide'), t('m_hide_sub'), false, () => setHidden(p, true)]);
  if (p.input_label) groups[1].push(['file', t('m_change_file'), file, busy, () => chooseInput(p, false)]);
  if (p.can_rebuild) groups[1].push(['hammer', t('m_rebuild'), t('m_rebuild_sub'), busy, () => startTask(p, 'build')]);
  groups[2].push(['github', t('do_github'), null, false, () => openLink(p.url), 'ext']);
  groups[2].push(['bug', t('fb_report_game'), null, busy, () => feedback('bug', p.id)]);
  if (installed) groups[3].push(['trash', t('m_uninstall'), null, busy, () => uninstall(p), 'danger']);
  const out = [];
  for (const g of groups.filter((x) => x.length)) {
    if (out.length) out.push(el('div', { class: 'menu-sep', role: 'separator' }));
    for (const [ic, label, sub, off, fn, cls] of g) {
      out.push(el('button', { class: 'menu-item' + (cls ? ' ' + cls : ''), role: 'menuitem', tabindex: '-1', disabled: off,
        onclick: () => { closeManageMenu(); fn(); } },
        icon(ic), el('span', { class: 'menu-text' }, el('span', {}, label, cls === 'ext' ? icon('external') : null), sub ? el('small', {}, sub) : null)));
    }
  }
  return out;
}

function fillManageMenu(p) {
  const focused = manageMenu.contains(document.activeElement) ? [...manageMenu.querySelectorAll('.menu-item')].indexOf(document.activeElement) : -1;
  manageMenu.replaceChildren(...manageItems(p));
  if (focused >= 0) manageMenu.querySelectorAll('.menu-item')[focused]?.focus();
  placeManageMenu();
}

// Under the gear, or above it when there is more room there; in a window too
// short for either, the menu scrolls. On big screens the page is zoomed, so
// the place is worked out in the window's pixels and then turned into the
// menu's own
function placeManageMenu() {
  manageMenu.style.maxHeight = '';
  const g = manageButton.getBoundingClientRect();
  const m = manageMenu.getBoundingClientRect();
  const z = m.width / manageMenu.offsetWidth || 1;
  const roof = document.querySelector('.titlebar')?.getBoundingClientRect().bottom || 0;
  const room = { below: window.innerHeight - g.bottom - 16, above: g.top - roof - 16 };
  const below = m.height <= room.below || room.below >= room.above;
  const height = Math.min(m.height, below ? room.below : room.above);
  const left = Math.max(8, Math.min(g.left, window.innerWidth - m.width - 8));
  manageMenu.style.left = left / z + 'px';
  manageMenu.style.top = (below ? g.bottom + 8 : g.top - 8 - height) / z + 'px';
  if (height < m.height) manageMenu.style.maxHeight = height / z + 'px';
}

function openManageMenu(byKeyboard) {
  const p = snap.projects.find((x) => x.id === current);
  if (!p) return;
  manageMenu = el('div', { class: 'menu', role: 'menu', 'aria-label': t('manage') });
  manageMenu.project = p.id;
  manageMenu.addEventListener('keydown', manageKeys);
  document.body.append(manageMenu);
  fillManageMenu(p);
  manageButton.classList.add('open');
  manageButton.setAttribute('aria-expanded', 'true');
  if (byKeyboard) manageMenu.querySelector('.menu-item:not(:disabled)')?.focus();
}

function closeManageMenu(refocus) {
  if (!manageMenu) return;
  manageMenu.remove();
  manageMenu = null;
  manageButton.classList.remove('open');
  manageButton.setAttribute('aria-expanded', 'false');
  if (refocus) manageButton.focus();
}

function manageKeys(e) {
  const items = [...manageMenu.querySelectorAll('.menu-item:not(:disabled)')];
  const i = items.indexOf(document.activeElement);
  const go = (n) => { e.preventDefault(); items[(n + items.length) % items.length]?.focus(); };
  if (e.key === 'ArrowDown') go(i + 1);
  else if (e.key === 'ArrowUp') go(i < 0 ? -1 : i - 1);
  else if (e.key === 'Home') go(0);
  else if (e.key === 'End') go(-1);
  else if (e.key === 'Tab') closeManageMenu();
}

// A click anywhere else, Escape, scrolling the page or resizing the window
// closes it
document.addEventListener('pointerdown', (e) => { if (manageMenu && !manageMenu.contains(e.target) && !manageButton.contains(e.target)) closeManageMenu(); });
document.addEventListener('keydown', (e) => { if (e.key === 'Escape' && manageMenu) { e.stopPropagation(); closeManageMenu(true); } }, true);
window.addEventListener('resize', () => closeManageMenu());
$('view-project').addEventListener('scroll', () => closeManageMenu());

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
    el('dt', {}, t('fb_launcher')), el('dd', {}, launcherVersion(t)),
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

// The launcher's version, and whether it is the installed or the portable copy
function launcherVersion(say) {
  return 'v' + snap.version + ' · ' + say(snap.portable ? 'fb_portable' : 'fb_installed');
}

// What goes with a message about `where` (a project id, or 'launcher'), as
// [label, value] rows. Always in English: it is read on GitHub.
function details(where, english) {
  const say = (key, vars) => (english ? STR.en[key].replace(/\{(\w+)\}/g, (_, k) => vars?.[k] ?? '') : t(key, vars));
  const p = snap.projects.find((x) => x.id === where);
  const rows = [[say('fb_launcher'), launcherVersion(say)]];
  // On GitHub, the registry's name: the one the author knows it by
  if (p) {
    rows.push([english ? p.default_name : shortName(p), versionLine(p, say)]);
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
  // The user's name in their own paths stays out of a public page. A very long
  // line (a linker's command line, thousands of characters) keeps only its start:
  // whole, it would push the error lines after it out of the address
  const LINE_MAX = 300;
  const all = log ? log.split('\n').filter((l) => l.trim())
    .map((l) => l.replace(/([A-Za-z]:\\Users\\)[^\\\r\n]+/gi, '$1<user>'))
    .map((l) => (l.length > LINE_MAX ? `${l.slice(0, LINE_MAX)} […]` : l)) : [];
  let lines = all;
  const make = () => {
    let md;
    if (kind === 'recomp') {
      md = `**Game:** ${title}\n**Original system:** ${system || '?'}\n\n**Why this one?**\n\n${text}\n\n---\n_Sent from NicoLauncher v${snap.version}_`;
    } else if (kind === 'bug') {
      md = `**What happened?**\n\n${text}\n\n**Details**\n\n${facts}`;
      if (lines.length) md += `\n\n<details><summary>Log of the last ${STR.en['act_' + lastFail.action]} (failed)</summary>\n\n\`\`\`\n${lines.join('\n')}\n\`\`\`\n</details>`;
    } else {
      md = `${text}\n\n---\n**About:** ${p ? p.default_name : 'the launcher'}\n\n${facts}`;
    }
    const name = kind === 'recomp' && system ? `${title} (${system})` : title;
    return `${base}${sep}title=${encodeURIComponent(name)}&body=${encodeURIComponent(md)}`;
  };
  // A long log does not fit in an address: as many of its last lines as fit,
  // where the error is (the first line to keep found by halving)
  let url = make();
  if (url.length <= URL_MAX || all.length < 2) return url;
  let lo = 1, hi = all.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    lines = all.slice(mid);
    if (make().length <= URL_MAX) hi = mid;
    else lo = mid + 1;
  }
  lines = all.slice(lo);
  return make();
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
      head.append(gameMark(p, 'ann-logo', 'ann-title'));
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

// What an update brings: every news entry newer than the version the user
// has, newest first, so skipping a few updates loses nothing. Shown when
// Update is pressed, before anything is downloaded, so the user reads it and
// chooses; and, for the launcher, after it updated itself without that (from
// an older launcher). The same window for a game and for the launcher.
let clClose = null;
const CL_MAX = 12;
function changelog({ head, title, from, to, entries, actions, all, ready, close }) {
  if (clClose) clClose();
  const box = el('div', { class: 'modal ann cl', role: 'dialog', 'aria-modal': 'true' });
  // A long list scrolls with the wheel, the bar, or the keyboard; a fade at
  // the bottom says there is more to read
  const keys = (e) => {
    const step = { ArrowDown: 60, ArrowUp: -60, PageDown: main.clientHeight - 60, PageUp: 60 - main.clientHeight, End: 1e6, Home: -1e6 }[e.key];
    if (step) { e.preventDefault(); main.scrollBy({ top: step, behavior: 'smooth' }); }
  };
  clClose = () => { clClose = null; document.removeEventListener('keydown', keys); box.remove(); };
  box.addEventListener('click', (e) => { if (e.target === box) clClose(); });
  const x = el('button', { class: 'ann-x', 'aria-label': t('close'), onclick: () => clClose() });
  x.innerHTML = '<svg viewBox="0 0 10 10"><path d="M1 1l8 8M9 1L1 9"/></svg>';
  const badge = ready
    ? el('span', { class: 'ann-badge up' }, icon('up'), t('cl_ready'))
    : el('span', { class: 'ann-badge good' }, icon('check'), t('cl_badge'));
  head.append(el('div', { class: 'ann-tags' }, badge), x);
  const count = entries.length === 1 ? t('cl_one') : t('cl_many', { n: entries.length });
  const meta = el('div', { class: 'cl-meta' },
    el('span', { class: 'cl-ver' }, from ? el('span', { class: 'cl-old' }, from) : null,
      from ? icon('chevr') : null, el('span', { class: 'cl-new' }, to)),
    el('span', { class: 'dim' }, count));
  const list = entries.slice(0, CL_MAX).map((e, n) => el('section', { class: 'cl-entry' + (n === 0 ? ' latest' : '') },
    el('h3', {}, el('span', { class: 'cl-num' }, e.label), el('span', { class: 'dim' }, dateLabel(e.title)),
      n === 0 ? el('span', { class: 'chip new' }, t('cl_latest')) : null),
    el('ul', { class: 'ann-list' }, ...e.lines.map((line) => el('li', {}, line)))));
  const main = el('div', { class: 'ann-main' }, el('h2', {}, title), meta, el('div', { class: 'cl-list' }, ...list));
  const inner = el('div', { class: 'modal-box ann-box cl-box' });
  const more = () => inner.classList.toggle('more', main.scrollTop + main.clientHeight < main.scrollHeight - 4);
  main.addEventListener('scroll', more);
  inner.append(
    head,
    main,
    el('div', { class: 'cl-fade', 'aria-hidden': 'true' }),
    el('div', { class: 'ann-foot' },
      el('button', { class: 'btn ghost small', onclick: () => { clClose(); all(); } }, t('cl_all')),
      el('div', { class: 'modal-actions' }, el('button', { class: 'btn ghost', onclick: () => clClose() }, close || t('close')), ...(actions || []))));
  box.append(inner);
  document.body.append(box);
  document.addEventListener('keydown', keys);
  more();
  // Pictures that load later change nothing below them, but the fonts can
  requestAnimationFrame(more);
  (box.querySelector('.btn.primary') || box.querySelector('.modal-actions .btn'))?.focus();
}

// Update pressed on a game: first what GitHub has that the installed copy
// does not, and the user chooses. Without news to show (none newer, GitHub
// not reached, or only an unfinished update to finish) Update rebuilds right
// away, as before.
function askUpdate(p) {
  if (!p.can_update || !p.new_entries.length) { startTask(p, 'update'); return; }
  const entries = p.new_entries;
  const from = Math.max(0, ...p.news.map((e) => e.number));
  const head = el('div', { class: 'ann-head game' });
  head.style.backgroundImage = gradient(p);
  if (p.hero) head.append(picture(p.hero, 'ann-hero'));
  head.append(gameMark(p, 'ann-logo', 'ann-title'));
  const v = (n) => t('version_short', { n });
  changelog({
    head, title: t('cl_title', { name: p.name }), ready: true, close: t('cl_later'),
    from: from ? v(from) : null, to: v(entries[0].number),
    entries: entries.map((e) => ({ ...e, label: v(e.number) })),
    actions: [el('button', { class: 'btn primary', disabled: !!snap.busy, onclick: () => { clClose(); startTask(p, 'update'); } }, icon('up'), el('span', {}, t('cl_go')))],
    all: () => openProject(p.id),
  });
}

// The launcher's own Update: what the new version brings (its news entries
// that start "Launcher X.Y.Z:"), and the user chooses. Without them (the news
// not loaded) it updates right away, and shows them after the restart.
function askLauncherUpdate() {
  const up = selfUpdate;
  if (!up.changes || !up.changes.length) { installLauncher(false); return; }
  const head = el('div', { class: 'ann-head cl-launcher' },
    el('img', { src: 'img/logo.png', alt: 'nico launcher', class: 'cl-mark', draggable: 'false' }));
  const v = (n) => t('version_short', { n });
  changelog({
    head, title: t('cl_title_launcher'), ready: true, close: t('cl_later'),
    from: v(snap.version), to: v(up.version),
    entries: up.changes.map((e) => ({ ...e, label: v(e.version) })),
    actions: [el('button', { class: 'btn primary', disabled: !!snap.busy, onclick: () => { clClose(); installLauncher(true); } }, icon('up'), el('span', {}, t('cl_go')))],
    all: () => show('home'),
  });
}

async function installLauncher(read) {
  selfUpdate.installing = true;
  render();
  // Read already: not shown again when the new version starts
  if (read) await call('mark_launcher_seen', { version: selfUpdate.version });
  // On success the launcher is replaced and started again: nothing comes back
  try { await invoke('install_launcher_update'); } catch (e) {
    toast(String(e));
    if (read) await call('mark_launcher_seen');
  }
  selfUpdate.installing = false;
  render();
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
    all: () => show('home'),
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
  $('version').textContent = 'v' + snap.version + (snap.portable ? ' · ' + t('portable') : '');
  $('version').title = snap.portable || '';
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
    closeManageMenu();
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
$('self-update').addEventListener('click', () => askLauncherUpdate());
$('announce').addEventListener('click', () => openAnnouncements());
$('lib-refresh').addEventListener('click', () => refresh());
$('home-refresh').addEventListener('click', () => refresh());
$('home-video').addEventListener('click', () => { if (snap && snap.latest_video) watchVideo(snap.latest_video); });
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
    else if (videoClose) videoClose();
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

// The window stays open behind a game it started, and WebView2 keeps drawing
// it there: its animations stop while another window is in use (style.css)
const markIdle = () => document.documentElement.classList.toggle('idle', !document.hasFocus());
window.addEventListener('blur', markIdle);
window.addEventListener('focus', markIdle);
markIdle();

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
    // An update with news: they were read before it started, so only a word
    // that it is done
    // or the rebuild of an unfinished one is done
    else if (payload.ok && finished.own && finished.action === 'update' && p && (p.news.some((e) => e.number > finished.fromVersion) || finished.wasPending)) toast(t('updated_toast', { name: p.name }), true);
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
