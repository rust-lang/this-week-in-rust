this-week-in-rust
=================

Data for this-week-in-rust.org

## How I get PR lists:

```
git log --author=bors --since='MM/DD/YYYY 12:00PM' --until='MM/DD/YYYY 12:00PM' --pretty=oneline > ~/entropy/twir.txt
# edit in vim to get rid of everything but PR number, copy into clipboard
for pr in $(xsel -ob); do firefox https://github.com/mozilla/rust/pull/$pr; sleep 0.07; done
# wait a long time...
# write TWIR
```

## How I get new contributors: 

  new_contribs.sh 6/21/2014 > ~/entropy/newbies.txt

Where `new_contribs.sh` is:

```sh
#!/usr/bin/sh                     

INITIAL_COMMIT=c01efc6
START_COMMIT=`git log --before="$1" --author=bors --pretty=format:%H|head -n1`
ALL_NAMES=`git log $INITIAL_COMMIT.. --pretty=format:%an|sort|uniq`
OLD_NAMES=`git log $INITIAL_COMMIT..$START_COMMIT --pretty=format:%an|sort|uniq`
echo "$OLD_NAMES">names_old.txt
echo "$ALL_NAMES">names_all.txt
diff names_old.txt names_all.txt
rm names_old.txt names_all.txt
```
