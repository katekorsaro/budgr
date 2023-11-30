#! /bin/fish

ls *.md | xargs -I {} pandoc {} -o {}.html
