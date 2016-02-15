#!/bin/sh
. ./bin/activate
pip2 install -r requirements.txt
./bin/pelican content -s pelicanconf.py || exit
if [ "$REALLY_DEPLOY" = "1" ]; then
    rsync -e "ssh -i $HOME/.ssh/twir_deploy_key" -razvP --delete-after output/ twir@octayn.net:production
fi
