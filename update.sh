rm -rf assets
mkdir -p assets
curl https://github.com/EnkaNetwork/API-docs/raw/master/store/loc.json -o assets/loc.json -L
curl https://github.com/EnkaNetwork/API-docs/raw/master/store/namecards.json -o assets/namecards.json -L
curl https://github.com/EnkaNetwork/API-docs/raw/master/store/characters.json -o assets/characters.json -L