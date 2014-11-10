#!/bin/sh

INITIAL_COMMIT=c01efc6
START_COMMIT=`git log --before="$1" --author=bors --pretty=format:%H|head -n1`
ALL_NAMES=`git log $INITIAL_COMMIT.. --pretty=format:%an|sort|uniq`
OLD_NAMES=`git log $INITIAL_COMMIT..$START_COMMIT --pretty=format:%an|sort|uniq`
echo "$OLD_NAMES">names_old.txt
echo "$ALL_NAMES">names_all.txt
names=`diff names_old.txt names_all.txt`
rm names_old.txt names_all.txt
names=`echo "$names" | grep \> | sed 's/^>/*/'`
echo "$names"
