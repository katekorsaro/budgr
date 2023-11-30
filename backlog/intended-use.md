# General use
- command name: **budgr** 

## sub commands
- backup
- check
- import
- init
- list
- list-tag
- report
- restore
- tag

## backup (b)
Creates a backup db file

## check (c)
Performs various checks on the db
- search for duplicates

## import (i) FILE
Imports FILE into db

### Options
- **--mappings (m) FILE**: apply mappings found in FILE while importing data

## init
Initilizes a new empty db

## list (ls)
Lists all operations in chronological order

### Options
- **--account (-a) ACC**: list operations tagged with account ACC (can be used in conjunction with --no-tag)
- **--goal (-g) GOAL**: list operations tagged with goal GOAL (can be used in conjunction with --no-tag)
- **--head (-h) N (default 10)**: list only the first N operations
- **--income (-i)**: list only incoming operations
- **--no-tag (-nt)**: list operations with missing tagging
- **--outcome (-o)**: list only outgoing operations
- **--purpose (-p) PUR**: list operations tagged with purpose PUR (can be used in conjunction with --no-tag)
- **--rev-chrono (-rc)**: reverse chronological order
- **--tail (-t) N (default 10)**: list only the last N operations
- **--width (-w) N (default 80)**: force the output width to N

## list-tag (lt)
Lists all tags

### Options
- **--account (-a)**: list all account tags
- **--goal (-g)**: list all goal tags
- **--purpose (-p)**: list all purpose tags

## report (r) ID
Produces report ID

### Options
- **--list (-l)**: list all installed reports

##  restore (r) FILE
Restores FILE as the current db

## tag (t) ID TAG
Tags operation ID with tag TAG
