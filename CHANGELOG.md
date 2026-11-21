# Changelog

## 1.8.0

- **Cycle jour/nuit** : une journée dure 10 minutes (départ 9 h 30), le
  soleil se lève à 6 h et se couche à 18 h ; ciel, brouillard et ambiance
  suivent. Heure réglable : boutons du menu pause (8 h / 13 h / 20 h /
  minuit) ou commande `/time 22`, même en solo. Protocole v9.
- **Vraie lumière des lampes la nuit** : les 16 lampes les plus proches
  deviennent de vraies lumières (12 000 cd — la flaque de lumière
  traverse la rue) ; le jour, seul l'éclat émissif subsiste.
- **Nuages d'orage** : une nappe sombre recouvre le ciel quand il pleut
  ou qu'une tornade approche, au lieu d'un bleu éclatant.
- **Zoom (C)** — style OptiFine/Zoomify : maintenir **C** resserre le
  champ partout (à pied, en vol, en voiture), la sensibilité souris
  suit le zoom pour viser. La vue de voiture passe à **X** ; en vol,
  Ctrl seul fait descendre.
- **Ombre du joueur** : disque dégradé au sol, qui s'écrase et s'efface
  quand on s'élève ; cachée en voiture et en vol libre haut.
- **Voitures** :
  - **2 places** : conducteur + passager (siège reculé, côté droit).
    Si le conducteur sort, le passager récupère le volant.
  - **vraies collisions à pied** : les corps rigides (voitures, immeubles
    effondrés) bloquent le joueur — on ne traverse plus une voiture
    dès qu'elle a quitté sa place de parking.
  - **un poteau n'arrête plus net une voiture lancée** : elle le défonce
    et passe, juste freinée par l'effort.
  - **plus résistante** : les impacts coûtent moins de vitesse et
    l'abîment moins (chips d'impact resserrés).
  - **son de moteur retravaillé** : grondement harmonique avec
    échappement filtré qui suit le régime (la vitesse de lecture suit
    les km/h), plus de scie pure hurlante.
- **Feux tricolores** : des vrais feux — mât fin sur le trottoir du
  carrefour (plus jamais planté sur l'asphalte au milieu de la
  chaussée), tête noire à trois feux, rouge ou vert allumé selon le
  carrefour. Nouveau matériau vert (bloqué « feu » par les tornades
  comme le reste du mobilier).
- **Tornades au sol** : elles suivaient la hauteur du terrain et
  grimpaient sur les toits rencontrés ; elles vivent désormais au
  niveau de la rue.
- **Tsunami** : la vague couvre **toute la largeur de la carte**, un
  vrai raz-de-marée au lieu d'un couloir de 65 m.
  - **L'eau est enfin visible** : le front de vague salissait ~3 000
    morceaux d'eau par seconde alors que le mailleur n'en reconstruisait
    que 1 400/s — l'affichage prenait des secondes de retard et la vague
    passait en détruisant sans se montrer. Budget porté à 120 par
    image : la vague se voit, au front comme à l'arrière.
  - **L'écoulement retombe** : derrière la vague ne reste plus deux
    cellules pleines (0,8 m d'eau sur toute la ville — or seules les
    cellules pas tout à fait pleines s'évaporent : l'inondation
    restait à vie) mais un filet peu profond qui coule vers les creux
    et s'évapore en une minute.
- **Grincement de porte** : joué en 2D avec le volume qui suit la
  distance ; la spatialisation crachotait dès qu'on tournait la tête
  pendant le grincement (le son sautait d'une oreille à l'autre).
- **Perf : grandes cartes** : la distance de chargement effective est
  plafonnée selon la qualité (240 m en Ultra). À 600 m, une grande
  carte tenait ~80 000 morceaux vivants et mangeait le budget des 60
  fps ; le brouillard suit la distance effective donc rien ne manque à
  l'œil. ~88 EPS maintenus en mégabombes en rafale sur la grande
  carte.
- **Bac à sable** : le compte d'armes du sélecteur de mode est calculé
  (76 aujourd'hui) au lieu d'un « 13 » périmé.
- **Re-monter dans une voiture déplacée** : le client ne cherchait
  qu'une voiture encore garée à sa place d'origine (ses voxels) — dès
  qu'une voiture avait roulé, F ne la trouvait plus jamais. Le client
  repère maintenant n'importe quel corps rigide « en forme de voiture »
  à moins de 6 m et le serveur place le joueur dans la voiture la plus
  proche (« Aucun véhicule à portée » sinon).
- **Hitbox des voitures par colonnes** : la collision joueur↔voiture
  utilisait une simple boîte englobante tournée ; elle épouse
  désormais les blocs qui restent (une voiture défoncée n'a plus la
  hitbox de sa carrosserie d'origine).
- **Vraie ombre du joueur** : un mannequin invisible (capsule sur une
  couche que seule la lumière voit) remplace le disque dégradé — le
  joueur projette une ombre nette de silhouette, haute ou basse
  selon le soleil.
- **Nuages volumétriques** : ~30 cumulus en spirale au-dessus de la
  ville, opacité et teinte suivant l'orage (gris clair le jour,
  gris-bleu la nuit) — plus de ciel bleu éclatant pendant une tornade.
- **Skybox** : la couleur du brouillard suit exactement celle du ciel,
  le raccord d'horizon disparaît.
- **Pluie arrêtable** : `/rain off` (ou `/rain` pendant un orage)
  arrête immédiatement l'averse au lieu d'attendre la fin.
- **Portes amochées** : une porte trop détruite par le tsunami est
  arrachée net (plus de portes fantômes bloquant passage).
- **Lumières élargies** : 32 lampes vivantes (au lieu de 16) dans un
  rayon de 120 cellules autour du joueur ; le grincement de porte est
  de nouveau spatialisé (3D), à volume réduit.
- **Portée des lampadaires** : 48 m au lieu de 22, et 45 000 cd au lieu
  de 12 000. La flaque s'arrêtait à deux plots ; la nuit est un peu plus
  sombre pour que la lumière des lampes ressorte.
- **Bunkers praticables** : la porte ne faisait que 4 cellules (1,6 m)
  pour un joueur de 1,7 m — impossible d'entrer. Salle haute de 8
  cellules (3,2 m), porte et escalier dégagés sur 8 cellules, marches
  un peu plus douces.
- **S'accroupir (Ctrl)** : la boîte de collision passe de 1,7 à 1,0 m et
  la caméra descend — on se faufile sous les plafonds bas et on va plus
  lentement. Ctrl continue de faire descendre en vol.
- **Tornades violentes** : rayon 7 → 12 m, 110 s de vie au lieu de 70,
  9 m/s au lieu de 6, 420 arrachements par pas (au lieu de 160) sur 26 m
  de haut, 10 % de tout casser (béton compris), vent 33 % plus large, et
  des joueurs vraiment aspirés/soulevés (il faut reculer de 30 m).
- **Segfault pendant les orages** : le mailleur d'eau recréait des
  centaines d'entités par image pendant un tsunami (spawn/despawn en
  rafale) ; il réutilise désormais les entités existantes en
  échangeant leur maillage.
- **Menu au fond noir / monde invisible au lancement** : deux causes.
  La météo démarrait à 0 h (minuit) tant que le serveur n'avait pas
  envoyé l'heure (ciel noir = « le void »), et le plugin de ray tracing
  expérimental, chargé sans les extensions GPU qu'il exigeait, cassait
  la passe opaque — toute la ville disparaissait, seuls les nuages et
  la pluie restaient. Le départ est en plein jour (9 h 30) et le plugin
  n'est plus chargé du tout (voir ci-dessous).
- **Polices** : les symboles ☀/☾ des boutons d'heure ne sont pas dans
  la police embarquée (carrés vides) — remplacés par du texte
  (« 8 h — matin », « Minuit »…), traduits dans les quatre langues
  comme le reste du menu.
- **Éclairage groupé** : la liste d'indices du clustering GPU est
  pré-réservée (131 072 indices, 8 192 tranches) **avant** l'init du
  rendu, pour que l'allumage des 32 lampes ne corrompe plus quelques
  images (l'ancien réglage arrivait trop tard : l'avertissement
  « resizing the view clustering index list » sortait encore).
- **Ray tracing : repoussé.** Bevy ne sait tracer la lumière que sur des
  maillages « propres » (position/normale/UV/tangente non compressées,
  indice 32 bits, un BLAS par maillage) : il aurait fallu une seconde
  copie de chaque morceau de ville en VRAM et un décodeur par image —
  l'inverse de ce qu'on cherche. Tenté, mesuré : image bruitée et monde
  noir. À refaire le jour où le mailleur produira des morceaux RT.
- **Nuages** : remontés à ~110 m (posés à 50 m, ils passaient devant la
  caméra et voilaient la rue d'une brume laiteuse) et bien plus
  discrets.

## 1.7.1

- **Faces « transparentes » corrigées** là où elles subsistaient :
  - lampes (lampadaires, plafonniers) et vitres posées sur une frontière de
    morceaux : leur face du côté voisin n'était dessinée par personne
    (l'optimisation « rien à mailler » ne regardait que les cellules du
    morceau lui-même) — on voyait à travers ;
  - vitres des voitures : le mélange des faces avant/arrière d'un même
    maillage (AlphaMode::Blend sans écriture de profondeur, aucun tri
    intra-maillage) est remplacé par AlphaToCoverage — plus de vitres qui
    disparaissent selon l'angle ;
  - eau et tsunamis : la face frontière d'une cellule d'eau posée sur une
    frontière de morceaux n'était jamais reconstruite (mur d'eau transparent).
- **Lampes réellement lumineuses** : `unlit` faisait ignorer `emissive` par le
  shader (idem traceurs, feu et flammes) — elles rendaient un jaune pâle plat
  qui se fondait dans le ciel.
- **Frame limit qui fonctionne** : l'ancienne boucle ne dormait que la moitié
  du dépassement (limite 60 = ~85 FPS réels) ; échéance absolue par frame,
  sans dérive ni rattrapage en rafale.
- **Sortie de voiture** : plus d'écrasement par sa propre voiture (exclusion
  de l'ancien véhicule pendant 2 s après la sortie, position serveur posée
  immédiatement, et le dernier recours ne place plus le joueur SOUS la
  voiture — sur le toit, à l'arrêt, sans danger).
- **Portes** : plus aucune porte bloquée dès la génération (pilier, meuble,
  caisses d'entrepôt ou une autre porte sur sa zone de rangement) — on
  déblaie les éléments gênants, sinon l'ouverture reste un passage libre.
- **Fin du crash sans explication quand la VRAM sature** : à 85 % du budget
  signalé par le pilote, l'erreur est propre et précoce ; la qualité est
  dégradée au minimum et le rendu relancé ; au second échec seulement, le
  jeu ferme — avec le motif écrit dans crash.txt et annonce dans le titre de
  la fenêtre.
- **Eau à hauteur variable** : la surface d'une cellule d'eau suit désormais son
  niveau (1/8 à 8/8 de la cellule) au lieu de tout rendre plein — les flaques sont
  des films d'eau, les vagues et tsunamis progressent à leur vraie hauteur. Le
  niveau est transmis au client (protocole v8) ; surfaces et côtés fusionnés par
  niveau (une nappe plate reste un quad).
- **Écran noir du menu et monde invisible en jeu (régression 1.7.1) corrigés** :
  la VRAM économisée en retirant le MSAA de la seule caméra vue en main
  (« jusqu'à 60 Mo en moins », ci-dessous) rendait les deux caméras
  incompatibles — Bevy exige le même `Msaa` sur toutes les caméras qui
  partagent une fenêtre, et rendait donc un écran tout noir. Le MSAA est
  de nouveau identique sur les deux (réglable dans les options) ; les
  changements de réglage s'appliquent bien aux deux caméras.
- **Mémoire** : allocation VRAM compacte (`MemoryHints::MemoryUsage`),
  indices U16 quand le maillage le permet (moitié de la mémoire d'indices),
  carte d'ombres initiale 2048 (et non 4096), ancienne carte libérée dès le
  début d'une régénération (pic de RAM divisé par deux), seuil d'alerte VRAM
  à 85 % du budget du pilote.

## 1.7.0

- **Eau** : simulation voxel (chute, étalement, remplissage des creux, bunkers et escaliers),
  nage, l'eau éteint le feu, les bouches d'incendie cassées fuient.
- **Catastrophes** : tornades (arrachent, aspirent et projettent), tsunamis (vague qui
  balaie la ville puis l'inonde), pluie (averses aléatoires, éteint le feu). Boutons de
  l'hôte dans le menu pause, commandes `tornado`, `tsunami`, `rain`, `drain`.
- **Explosions** : débris projetés selon la puissance (jusque sur les bâtiments voisins),
  plus nombreux et dangereux, étincelles, onde de choc, champignon de fumée.
- **Sol profond** (12 m : roche et terre) et **bunkers** sous les parcs et parkings
  (entrée : escalier bordé de murets, balisé par deux lampes).
- **Chargement** : détail des étapes (génération, morceaux maillés, modifications reçues),
  barre qui ne recule plus.
- **Véhicules** : vue du conducteur par défaut (C : vue de poursuite).
- **Rendu** : plus de murs invisibles quand ils tombaient sur une frontière de morceaux
  (bug apparu avec le chargement progressif).
- **Performances** : maillage du terrain sur tous les cœurs (plus de délai après une
  grosse explosion), morceaux d'un seul matériau compressés, ombres plus légères
  (3 cascades sur 110 m, carte de 3072).
- **Corrections** : roues à leur place, caméra qui ne saute plus au contact d'une voiture,
  débris qui ne se téléportent plus, portes qui ne se ferment plus sur un joueur (et plus
  d'éjection sur le toit), débris figés qui ne coincent plus les joueurs (ni l'anti-triche),
  chat vraiment modéré (mots déguisés, liens, majuscules).
- Les mondes sauvegardés avec une version précédente ne se chargent plus (carte modifiée).

## 1.6.1

- **Langue** : les valeurs des réglages ne disparaissent plus après un changement de langue.
- **Carte** : plus de plafonniers ni de feux tricolores en l'air (test : toute la ville est
  reliée au sol).
- **Physique** : un morceau qui tremblote sans fin finit par se poser ; les voitures ne
  passent plus par-dessus les poteaux (normale de contact corrigée).
- **Feu** : brûle aussi la peinture, les toitures et le plastique.
- **Caméra** : retour à la vue normale en sortant d'une voiture.

## 1.6.0

- **Multijoueur** : plus de blocage sur « Chargement du terrain » en rejoignant un serveur
  actif (les modifications envoyées en continu remettaient le compteur à zéro).
- **Mobile** : un doigt par rôle (joystick dans le bas de l'écran, regard partout ailleurs,
  boutons fonctionnels dont TIR), clavier virtuel pour le chat et les champs, boutons chat
  et pause (et touche Retour), paysage forcé, interface adaptée.
- **Voitures** : vraies roues (suspension, adhérence, braquage des roues avant, rotation).
- **Langues** : français, anglais, espagnol, allemand (Paramètres → Langue).
- **Écran de chargement** dès le lancement, sur PC comme sur mobile.
- **Physique** : les morceaux qui se figent sans appui retombent au lieu de flotter.
- **Sauvegarde** du monde réservée au solo.
- **Sécurité** : lecture réseau bornée par client (plus d'inondation possible), découpage
  des trames en temps linéaire.
- **Android** : correctif de wgpu pour les pilotes Mali (plantage « pool plein »), erreurs
  GPU non fatales, dernier plantage affiché au menu.

## 1.5.5

- Correctifs : MSAA 2× (plantage llvmpipe), pseudos au-dessus des joueurs, anti-triche
  moins sévère, spawn aléatoire sur un trottoir, montée des marches, première version
  Android.

## 1.5.0

- **Armes** : 76 outils (4× plus) rangés en 6 catégories (touches 1–6), boutique par
  catégorie. Nouveautés : lance-flammes, extincteur, mines, claymore, napalm, molotov,
  canon, Tsar, lasers, armes blanches…
- **Véhicules** : les voitures garées se conduisent (corps rigide, adhérence, direction) ;
  elles ne roulent plus quand elles sont trop abîmées.
- **Chat** modéré (débit, anti-répétition, mots interdits, mute) et **voix de proximité**
  (RNNoise, réglage automatique du gain, limiteur).
- **Liste de serveurs** enregistrés avec joueurs, mode, message du jour et ping.
- **Discord Rich Presence** (solo / multijoueur), **icône** du jeu partout (fenêtre,
  exécutable Windows, .desktop, AppImage).
- **Chargement complet** (ville générée en arrière-plan, maillage avant de jouer), maillage
  en tranches de temps.
- **Physique** : les gros morceaux de bâtiment continuent de se briser et de tomber (plus de
  « gel »), les petits finissent en tas de gravats.
- **Vol** : désactivé par défaut en multijoueur (`allow_fly`), collisions et limites de
  carte respectées, plus de faux positifs de l'anti-triche.
- **Pause** : en ligne, le monde (et ton personnage) continuent pendant le menu.
- **Portes** cassées ne s'ouvrent plus. Rouge à l'écran, battements de cœur, acouphène.
- Sons plus puissants à longue portée, musique plus discrète, collage / copie dans les champs.
- Interface à l'échelle de la fenêtre. Console du serveur sans coupure de saisie.
- Taille de ville réglable (`map_size`, choix en solo) : moins de mémoire.
- Protocole réseau v5.
## 1.4.0

- **Physique structurelle** : stabilité par chemin de support (portée selon le matériau),
  piliers porteurs dans les bâtiments, mobilier indépendant de la structure. Les planchers,
  façades et porte-à-faux trop longs tombent.
- **Feu** : combustion et propagation (bois, feuillage, tissu), flammes visibles, brûlures.
- **Poussière et fumée** à chaque écroulement, impact et explosion.
- **Préréglages graphiques** Bas / Moyen / Élevé / Ultra / Personnalisé, avec anticrénelage,
  particules et bloom réglables.
- **Son** : effets normalisés, portée spatiale plus longue, nouveaux sons (grondement,
  impact lourd, crépitement, pose de bloc). **Musique** générative et dossier `music`.
- **Écran de chargement** : la ville est générée en arrière-plan, le jeu démarre quand le
  terrain proche est prêt (plus de saccades à l'arrivée).
- **Mémoire** : maillages compacts (24 octets par sommet, sans copie CPU), chargés selon la
  distance d'affichage, matériaux des morceaux en palette 4 bits. ~1,1 Go au lieu de ~2,8 Go.
- Journal du serveur à l'heure de la machine (plus d'UTC).

## 1.3.0

- **Physique** : un bâtiment dont la base est détruite tombe en bloc. L'ensemble détaché devient
  un corps rigide (masse, centre de masse, inertie, gravité 9,81 m/s²), heurte le monde, se
  casse à l'impact, se scinde, puis se fige. Plus de limite de taille : les ensembles
  flottants n'« oublient » plus de tomber.
- **Solo sur serveur** : le solo lance un serveur privé local ; physique, dégâts, argent,
  portes, blocs et sauvegardes sont les mêmes en solo et en ligne.
- **Serveur autoritaire** : inventaire, argent, munitions, rechargement et cadence de tir
  sont validés côté serveur ; les explosions doivent correspondre à une bombe armée.
- **Anti-triche** : contrôle de vitesse, de traversée de murs et de vol, correction de
  position, expulsion progressive, limite de connexions par IP, bannissement.
- **Sauvegardes** : fichiers `.bgw` contenant tout le monde (gravats et corps compris) ;
  sauvegarde / chargement en solo et sur serveur (automatique et à l'arrêt).
- **server.conf** : ~25 réglages (armes interdites, construction, portes, dégâts de chute,
  effondrement, argent de départ, sauvegarde automatique, anti-triche…).
- **Journal du serveur** style Minecraft (`logs/latest.log`), console avec commandes.
- **AppImage** : plus seulement portable, elle s'installe (menu des applications,
  commande `boomground`).
- **Rendu** : occlusion ambiante, texture de voxel (grain, biseau, joints) avec mipmaps,
  filtrage anisotrope, MSAA 4×, HDR, bloom, tonemapping et étalonnage.
- Protocole réseau v4.

## 1.2.0

- Carte ~30 fois plus grande (845 × 845 m), stockage par morceaux alloués à la demande,
  chargement progressif du maillage.
- Quartiers : centre-ville de tours, quartiers mixtes, banlieue.
- Intérieurs meublés (tables, chaises, bureaux, canapés, étagères, plafonniers) et
  mobilier urbain (lampadaires, bancs, bouches d'incendie, poubelles, feux tricolores).
- Portes interactives (`F`), synchronisées en multijoueur.
- Outil de construction *Blocs* (9 matériaux), synchronisé en multijoueur.
- Physique : effondrement de ce qui n'est plus relié au sol, masse des gravats, écrasement.
- Points de vie, mort, ragdoll, caméra à la troisième personne, réapparition.
- Nouvelles armes : Pioche, Revolver .44, Pistolet-mitrailleur, Minigun, Railgun.
- Réglages : distance d'affichage, ombres, bruitages, secousses, balancement, FPS, viseur.
- Interface : l'aide permanente des touches est retirée du HUD.
- Serveur dédié `BoomGround-server` (Windows, Linux) avec `server.conf` généré au premier
  lancement.
- Protocole réseau v3 (incompatible avec les versions précédentes).
- Nouvelle licence (source disponible, contributions acceptées, copie interdite).

## 1.1.0

- Premier build publié : jeu en un seul fichier (Windows `.exe`, Linux `.AppImage`).
