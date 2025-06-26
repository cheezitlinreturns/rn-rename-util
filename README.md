# RN

`Rn` is a rename utility i made because rename just wouldnt work. its very minimalistic, because i never really though of any flags other then -o. its both a batch and singular rename utility. i dont know if this supports other OSes other then linux.
# INSTALLATION
It's pretty simple. you just run:
```bash
git clone https://github.com/cheezitlinreturns/rn-rename-util.git
```
```bash
cd rn-rename-util
```
then, build it. DO NOT RUN AS SUDO, CARGO WILL ERROR.
```
make install
```
this will cd into rn (the top level of the rust project), then cargo build --release it.
if the Makefile doesnt work, here is how to do it manually:
```
git clone https://github.com/cheezitlinreturns/rn-rename-util.git
```
```
cd rn-rename-util
```
```
cd rn
```
```
cargo build --release
```
```
sudo cp target/release/rn /usr/bin/rn
```

# USAGE
******
to rename one file, do this:
```
rn file file2
```
to rename multiple files, as to not confuse the program, the output must be specified with -o.
```
rn file file2 file3 notafile
```
when renaming multiple files, rn will append numbers to each file to differentiate them, so notafile, notafile2, notafile3... so on so on.
