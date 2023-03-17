rm -rf assets
mkdir -p assets
curl https://github.com/EnkaNetwork/API-docs/raw/master/store/loc.json -o assets/loc.json -L
curl https://github.com/EnkaNetwork/API-docs/raw/master/store/namecards.json -o assets/namecards.json -L
curl https://github.com/EnkaNetwork/API-docs/raw/master/store/characters.json -o assets/characters.json -L
curl https://raw.githubusercontent.com/theBowja/genshin-db/main/src/data/image/characters.json -o assets/image_characters.json -L
curl https://cdn.discordapp.com/attachments/819643218446254100/960919577867485204/icon.zip -o assets/icon.zip -L