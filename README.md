# reto_dApp
Reto dApp platzi Equipo 4

## Importante antes de empezar
Aseguraos de que vuestra rama esté actualizada con master, he cambiado el gitignore para que no meta basura y así nos evitamos toda la basura que hay en los targets que no necesitamos

## Forma de trabajar
Gitflow: https://www.atlassian.com/es/git/tutorials/comparing-workflows/gitflow-workflow
Cada uno con su rama hasta los merge en los release candidate y después a la main

## Comandos útiles

### Smart contract en rust
Para añadir el target a compilar debes ejecutar `rustup target add wasm32-unknown-unknown`
Usa `cargo build --target wasm32-unknown-unknown --release` para compilar el smart contract y sacar el `.wasm` que necesitamos para publicar

