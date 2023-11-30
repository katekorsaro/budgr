# General use
- command name: **budgr** 

## sub commands
- list
- list-tag
- tag
- report
- backup
- restore
- check
- import

## list (ls)
List all operations in chronological order

### Options
- **--width (-w) N (default 80)**: force the output width to N
- **--head (-h) N (default 10)**: list only the first N operations
- **--tail (-t) N (default 10)**: list only the last N operations
- **--income (-i)**: list only incoming operations
- **--outcome (-o)**: list only outgoing operations
- **--rev-chrono (-rc)**: reverse chronological order
- **--no-tag (-nt)**: list operations with missing tagging
- **--account (-a) ACC**: list operations tagged with account ACC (can be used in conjunction with --no-tag)
- **--purpose (-p) PUR**: list operations tagged with purpose PUR (can be used in conjunction with --no-tag)
- **--goal (-g) GOAL**: list operations tagged with goal GOAL (can be used in conjunction with --no-tag)

## list-tag (lt)
List all tags

### Options
- **--account (-a)**: list all account tags
- **--purpose (-p)**: list all purpose tags
- **--goal (-g)**: list all goal tags

## tag (t) ID TAG
Tag operation ID with tag TAG

## report (r) ID
Produce report ID

### Options
- **--list (-l)**: list all installed reports

## backup (b)
Create a backup db file

##  restore (r) FILE
Restore FILE as the current db

## check (c)
Performs various checks on the db
- search for duplicates

## import (i) FILE
Imports FILE into db

### Options
- --mappings (m) FILE: apply mappings found in FILE while importing data
