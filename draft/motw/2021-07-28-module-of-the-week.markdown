Title: Module of the Week 1 (std::fs)
Date: 2021-07-28 9:58
Category: Module of the Week
Tags: motw, fs, easy
Status: draft

Hello and welcome to *Module of the Week*. Inspired by [PyMOTW](https://pymotw.com/3/) (and similar ideas), this blog series is meant to dig into some modules available when working with Rust. This will primarily cover the [Standard Library](https://doc.rust-lang.org/std/index.html#modules) but you'll probably see some popular Crates show up once in a while as well.

Each issue has a bit of a story (not to the length you'd find on some Recipe sites) to guide new users and make learning a bit more fun.

Enough said, let's get to `std::fs`!

<!-- more -->

# The story

[Rudgal The Delirious](https://www.fantasynamegenerators.com/dnd-orc-names.php) is the local Ornithologist. Armed with his trusty camera he's been tasked by the local Government to catalog and identify all of the birds in his local area. As he has to identify not only the species but the individual birds he's ended up with quite a few photos!

Having coded in college (he didn't end up with the title *The Delirious* for nothing), he sits down with his favorite IDE, forgets about tabs and spaces and dives right into `std::fs`.

# The module

[std::fs](https://doc.rust-lang.org/std/fs/index.html) is the standard Rust module for interacting with a Filesystem. It handles a lot of the messy work when it comes to all the different ways that the underlying OS likes to handle files. Creating, Reading, Updating, Deleting, it's all there! Not every operation is supported by every OS, but this guide will try to remain as platform agnostic as possible. 

!!! note
    There is a lot of overlap with [std::path](https://doc.rust-lang.org/std/path/index.html) in all things filesystem related, but that will be discussed in the next part.

Rudgal keeps all of his files in `/home/rudgal/birds`, so he'll start with that.

    :::rust
    const PHOTO_HOME: &'static str = "/home/rudgal/birds";

## Listing all of the files in a directory

As an easy start, Rudgal wants to list all of the photos he's taken. Browsing through the documents, he comes across [std::fs::read_dir](https://doc.rust-lang.org/std/fs/fn.read_dir.html).

> "Aha! An Iterator. A most important tool in Ornithology!" 

Rudgal shouts, to no one in particular.

`std::fs::read_dir` gives you an Iterator (aptly named [ReadDir](https://doc.rust-lang.org/std/fs/struct.ReadDir.html)) of [DirEntry](https://doc.rust-lang.org/std/fs/struct.DirEntry.html) objects. More specifically it's a [io::Result](https://doc.rust-lang.org/std/io/type.Result.html) of `DirEntry`, in case something goes wrong along the way.

> "I'll start with the paths of the files."

``` rust
use std::{fs, io};

let entries = fs::read_dir(PHOTO_HOME)?
    .map(|entry_res| entry_res.map(|entry| entry.path()))
    .collect::<Result<Vec<_>, io::Error>>()?;

println!("{:?}", entries);
```

!!! note
    If you're not familiar with the `?` syntax or `Result`s in general, stay tuned for when we dive into the [std::result](https://doc.rust-lang.org/std/result/index.html) module!

After running his program, he gets the following output:

``` shell
["/home/rudgal/birds/winter2010","/home/rudgal/birds/savalirwood", "/home/rudgal/birds/DCIM_0001.jpg", "/home/rudgal/birds/DCIM_0002.jpg", ... "/home/rudgal/birds/DCIM_9999.jpg"]
```

Now he has a list of everything in the `PHOTO_HOME` directory. This includes both files and sub-directories.

## Filtering files in a directory

Rudgal wants to sort out some of the entries from the previous section. Specifically he just wants to count the photos in the root of `PHOTO_HOME`. For that he finds his way to [std::fs::metadata](https://doc.rust-lang.org/std/fs/fn.metadata.html) in the docs.

``` rust
// See previous section for the definition of "entries"

let just_photos = entries.iter()
    .map(|entry| (entry, fs::metadata(entry)))
    // Only select files, filtering out any directories or errors
    .filter(|(entry, meta)| meta.as_ref().map_or(false, |meta| meta.is_file()))
    // Unwrap the results
    .map(|(entry, meta)| (entry, meta.unwrap()))
    .collect::<Vec<_>>();

println!("There are {} actual files.", just_photos.len());
```

Satisfied with his `Vec` of `(path, metadata)` pairs, Rudgal gave it a spin.

``` shell
There are 9999 actual files.
```

> "A perfectly sized collection of fauna!"

## Getting the size of a file

Now that he had a list of just the photos in `PHOTO_HOME`, Rudgal wanted to know how much space all of the files were taking up. Luckily, [std::fs::Metadata](https://doc.rust-lang.org/std/fs/struct.Metadata.html) had something for just such an occasion!

``` rust
let photo_size:u64 = just_photos.iter()
    .map(|(path, meta)| meta.len())
    .sum();

println!("Total size is {}", photo_size);
```

``` shell
Total size is 127190000
```

> "Maybe I should organize some of this."

Back to the docs, Rudgal!

## Recursively get files in a directory

Rudgal quickly realized he had a problem. By filtering for just files from his original list, he's left out a number of directories left over from some previous half-hearted attempts to organize his photos.

> "Recursion! The cause of, and solution to, all of life's problems!"

Said Rudgal the Delirious.

``` rust
use std::path::PathBuf;

fn visit_dirs(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut files = vec![];
    if dir.is_dir() {
        let mut dir_files = vec![];
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.push(visit_dirs(&path)?);
            } else {
                dir_files.push(path);
            }
        }
        files.push(dir_files);
    }
    Ok(files.iter().flatten().collect())
}

...

let all_files = visit_dirs(&PathBuf::from("."))?;
println!("The number of files (recursively) is {}", all_files.len());
```

It's directories all the way down! Let's see what Rudgal got as a result.

``` shell
The number of files (recursively) is 24038
```

Much better! Now he can be sure he didn't miss any files. He's a bit of a shutterbug.

# Next week

Stay tuned for next week when we'll continue exploring `std::fs`, more specifically focusing on creating directories and moving files around.