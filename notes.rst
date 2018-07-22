
Test all:
pd=$(pwd); for d in `find . -maxdepth 1 -type d`; do if [ -e "$d/Cargo.toml" ]; then echo $d; cd "$d"; cargo test || break; cd $pd; fi; done


