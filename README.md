# budgr - your personal finance friend!

budgr is a quite simple personal finance application. It offers basic actions
over operation entity and some aggregation capabilities.

budgr is a command line application where you can type in one command at a time
and get a result.

budgr is quite open. Its source code is freely available on line. It also
doesn't rely on any proprietary format. Files are saved in a self-explanatory
format (pipe-seaprated value) in order to be human and machine readable.

# How to use budgr

budgr has a series of basic command that grant you access to underlying data.
It provides basic CRUD operations as well as some more advanced bulk operations.

## First execution

Simply type `bgr` into your favourite terminal and see what happens. A little
and, I hope, quite clear help message is displayed. Here you can find a list of
possible commands as well as some filter options.

Some commands such as Modify, Delete, List and Aggregate, work on all the
operations included in the filter expression. So, in general term budgr works
like this:

`budgr [filter] [command] [options]`

Be aware that if you're not specifying any filter condition, you are acting on
the whole data set. My advice would be to put the data folder under some version
control system such as git.

Speaking of data folder, at the first application start, the default data folder
and the default configuration file are created. They are, respectively `./data`
and `budgr.toml`.
