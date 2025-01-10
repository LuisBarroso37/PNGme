# PNGme

Add, show and remove secret messages from your PNG files

This project follows [this guide from Picklenerd](https://jrdngr.github.io/pngme_book/).

## Install

Locally:

    cargo install

Remotely:

    cargo install --git https://github.com/LuisBarroso37/pngme

## Running

Add a secret message to a PNG in a "RuST" chunk:\
_Output file is an optional flag which allows you save your changes in a new file_

    pngme encode ./<file name>.png RuST "<Secret message>" [output file]

Show secret message:

    pngme decode ./<file name>.png RuST

Remove a secret message:

    pngme remove ./<file name>.png RuST

Print out every chunk in the PNG file:

    pngme print ./<file name>.png
