#!/bin/sh

read -p "Enter the directory to copy website contents to (likely ending in 'this-week-in-rust.github.io'): " directory

# Check if the provided input is a valid directory
if [ ! -d "$directory" ]; then
  echo "Error: $directory is not a valid directory."
  exit 1
fi

# Check if the file "CNAME" exists in the directory
if [ -f "$directory/CNAME" ]; then
  rsync -razvP --delete --exclude /CNAME --exclude /.git output-website/ $directory
  echo "Finished syncing with directory '$directory'"
else
  echo "ERROR: This does not seem to be the 'this-week-in-rust.github.io' repo..."
  echo "Please copy contents manually using 'rsync -razvP --delete --exclude /CNAME --exclude /.git output-website/ /path/to/this-week-in-rust.github.io/'"
fi
