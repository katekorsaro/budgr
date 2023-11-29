# General use

## budgr list (ls)
List all operations in chronological order

### Options
- --width (-w) N (default 80): force the output width to N
- --head (-h) N (default 10): list only the first N operations
- --tail (-t) N (default 10): list only the last N operations
- --income (-i): list only incoming operations
- --outcome (-o): list only outgoing operations
- --rev-chrono (-rc): reverse chronological order
- --no-tag (-nt): list operations with missing tagging
- --account (-a) ACC: list operations tagged with account ACC
- --purpose (-p) PUR: list operations tagged with purpose PUR
- --goal (-g) GOAL: list operations tagged with goal GOAL

## budgr list-tag (lt)
List all tags

### Options
- --account (-a): list all account tags
- --purpose (-p): list all purpose tags
- --goal (-g): list all goal tags

## budgr tag (t) ID TAG
Tag operation ID with tag TAG

### Options
- --account (-a) ACC: tag operation ID with account tag ACC
- --purpose (-p) PUR: tag operation ID with purpose tag PUR
- --goal (-g) GOAL: tag operation ID with goal tag GOAL
