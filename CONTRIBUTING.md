# Contribuer à BoomGround

Merci de vouloir aider ! Les contributions sont bienvenues, dans le cadre de la
[licence](LICENSE).

## Ce que tu peux faire

- Forker le dépôt sur GitHub et proposer une *pull request*.
- Signaler un bug ou proposer une idée via une *issue*.
- Utiliser toute forme d'intelligence artificielle pour modifier le code via un *fork* et faire une *pull request*.

## Avant de faire une pull request

Vérifier que le code fonctionne:

```bash
cargo fmt
cargo test --release
cargo clippy --release
```

Vérifier bien ces points:

- Le jeu et le serveur partagent le même monde : toute modification du protocole
  (`src/net/protocol.rs`) doit incrémenter `PROTOCOL_VERSION`.
- Le monde est **déterministe** à partir de la seed : n'utilise pas d'aléa non seedé dans
  `map.rs`.
