[![Build Status](https://travis-ci.org/adrianN/MovieThingy.svg?branch=master)](https://travis-ci.org/adrianN/MovieThingy)

MovieThingy
===========

A small tool to play videos on a Raspberry Pi. It tries to read a working directory from `~/.moviethingy`. The file should contain `dir=<workingdir>`, e.g. `dir=~/Videos/`. If the file is not found, or reading fails, we default to the current directory.

Once started it provides fuzzy search over the mp4 files in (all subdirectories of) the working directory. Hit enter to play the highlighted file. CTRL+p and CTRL+n move the selection. Press CTRL+q to exit.


