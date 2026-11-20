# Changelog

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
- **Mémoire** : allocation VRAM compacte (`MemoryHints::MemoryUsage`),
  indices U16 quand le maillage le permet (moitié de la mémoire d'indices),
  MSAA retiré de la caméra vue en main (jusqu'à 60 Mo de VRAM en moins),
  carte d'ombres initiale 2048 (et non 4096), ancienne carte libérée dès le
  début d'une régénération (pic de RAM divisé par deux).

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
