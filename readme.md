## QuickTag: A convenient audio file tagger written in rust

### Setup
Create a directory in which you want to tag your files. Then download the pre-build binary and move it into the directory. Next, create a file named **EXACTLY** `config.toml`, paste the pre-made config into that file, and edit it as you wish.

```toml
# Artist name for all of the files
artist = "ARTIST"

# If we should move the finalized file into the configurated folder
move_into_folder = true

# If we should delete the original untagged file
delete_original = true

# List of all albums
[albums]

[albums.EXAMPLE_1]
# The title of the album
title = "EXAMPLE_1"

# The folder which contains all of the album's music
folder = "EXAMPLE_1"

# The cover art of the album, prefixed with the covers directory
cover = "EXAMPLE_1.png"

[albums.EXAMPLE_2]
title = "EXAMPLE_2"
folder = "EXAMPLE_2"
cover = "EXAMPLE_2.png"
```

Now you must structure your directory as follows.
```
covers/
    EXAMPLE_1.png
    EXAMPLE_2.png
EXAMPLE_1/
EXAMPLE_2/
quick_tag.exe
config.toml
```
### Usage
This is a CLI program, meaning it interacts with the user through the command line. The blueprint for running it is `quick_tag.exe <file>...`. To run the program, open a terminal (or command prompt for windows) and type `quick_tag.exe SONG.mp3`. Then just run through the wizard.