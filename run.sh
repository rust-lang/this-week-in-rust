#!/bin/sh
. ./bin/activate
./bin/pelican content -s pelicanconf.py -t themes/pelican-elegant-1.3 || exit
if [ "$REALLY_DEPLOY" = "1" ]; then
    rsync -e "ssh -i $HOME/.ssh/twir_deploy_key" -razvP --delete-after output/ twir@octayn.net:production
fi
