# Dollah
Dollah (short for dollar) is a short little program written in Rust to help you
follow tutorials which display shell commands to run. Sometimes they copy the
whole thing (including the $/#) which can lead to errors where you have to
navigate all the way to the start of the line which can be infuriating at times.
With Dollah, no more. It can even run root applications using the '#' executable instead.

## Why Rust and not just Shell?
I wanted to write this in Rust because while you could just write a shell script
and call it a day, you don't get the memory management capabilities, where its
all just handles by bash/dash. Also, the shell used varies from Linux distro to
Operating System and therefore cannot be trusted in the same way that Rust can.
x86_64 Assembly will always be x86_64 assembly, where as a shell could be Zsh,
Bash, Dash, or even FISH (although why you would set /bin/sh as FISH). \
Another reason for having it as Rust is just to get to know the language more, and although it is relatively small I sill learned alot from it.

## Installation
Ok so I basically wrote a whole on cooking article up there but the installation is simple.
```bash
git clone https://github.com/Mythbusters123/dollah
cd dolla
chmod a+x install.sh
./install.sh
```
**Windows is not supported, and probably never will be as there is no use.**