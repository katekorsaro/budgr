#! /bin/fish

ls -R *.rs | xargs -I {} rustfmt {}
