#!/bin/sh
. ./bin/activate
./bin/pelican content -s pelicanconf.py -t themes/pelican-elegant-1.3 || exit
if [ "$REALLY_DEPLOY" = "1" ]; then
    rsync -razvP --delete-after output/ cmr@octayn.net:twir
fi
