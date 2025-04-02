# API HTTP Headers en Rust
### Description

Ce projet implémente une API HTTP simple qui retourne les headers de la requête au format JSON.
Développé en Rust avec le framework Axum, ce projet répond aux exigences suivantes :
Endpoint GET /ping qui retourne les headers de la requête au format JSON
Réponse vide avec code 404 pour toute autre requête (méthode différente ou route inexistante)
Port d'écoute configurable via la variable d'environnement PING_LISTEN_PORT
Interface graphique de test intégrée accessible sur la route racine

### Prérequis
Rust et Cargo installés sur votre système
Installation via [rustup.rs](https://rustup.rs/)

### Installation
```
git clone https://github.com/salimskander/tp-wik-dps-tp01.git
cd tp-wik-dps-tp01
```
### Compilation
```
cargo build --release
```
### Lancement  
```
cargo build --release
```
### Utilisation
Une fois le serveur lancé, vous pouvez :

Accéder à l'interface graphique de test :

Ouvrez votre navigateur et accédez à [localhost port 3000](http://localhost:3000/ )

Utilisez les boutons pour tester les différentes fonctionnalités

Tester l'API directement :

Structure du projet

src/main.rs : Code source principal contenant la logique de l'API (et l'interface graphique)

Cargo.toml : Fichier de configuration du projet et des dépendances
Dépendances

axum : Framework web pour Rust

Auteur
Ali Salim Skander
