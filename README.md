# budgr - your personal finance friend!

budgr is a **quite simple personal finance application**. It offers basic actions
over operation entity and some aggregation capabilities.

budgr is a **command line application** where you can type in one command at a time
and get a result.

budgr is quite **open**. Its source code is freely available on line. It also
doesn't rely on any proprietary format. Files are saved in a self-explanatory
format (pipe-seaprated value) in order to be human and machine readable.

## The underlying idea

budgr does not implement any of those fancy budget methods. It simply allow you
to register operations and get aggregate data out of them. I found, over the
year of using such "method" that the simple way is more often than not the best
way.

By doing so, you can easily get information about cashflow (aggregate operation
over a period of time), center of costs (aggregate operation over a given goal
or purpose).

Speaking of which... budgr gives you the possibility to "tag" operations with
one purpose and one goal tag (as well as one account tag, but that's obvious, so
yes, it can manage multiple accounts).

Purposes are hardcoded. They are:
- **None**
- **Need**: for those operations related to survival (your bills, family expenses,
  insurance and so on)
- **Yearly** Need: for those operations related to survival that happens just once.
  Insurance is recurrent, while an unexpected expenses is not (hopefully)
- **Want**: for those operations related to amusement, entertainment and that are in
  general avoidable but still necessary to give life a meaningful fullness. Such
  expenses can be dinners, movies, some fancy gadget you fell in love with
  (though, try to love people and to like objects and not the other way around).
- **Yearly** want: same as before, but related to entertainment and such.
- **Goal**: for those operations related to a some long term goal. Mortage, savings,
  a travel, the new car are all example of significant goal.
- **Income**: for those operations that brings money home and not viceversa. Salary,
  lottery win, tax reimbursement are all examples of income operations.

Goal tag, on the other hand is not hardcoded. It's common sense that every
operation tagged with Goal purpose, should have a Goal tag as well. Tags are
exactly what I explained before: free text stating what the operation is all
about.

# How to use budgr

budgr has a series of basic commands that grant you access to underlying data.
It provides basic CRUD operations as well as some more advanced bulk operations.

## First execution

Simply type `bgr` into your favourite terminal and see what happens. A little
and, I hope, quite clear help message is displayed. Here you can find a list of
possible commands as well as some filter options.

Some commands such as Modify, Delete, List and Aggregate, work on all the
operations included in the filter expression. So, in general term budgr works
like this:

`bgr [filter] [command] [options]`

Be aware that if you're not specifying any filter condition, you are acting on
the whole data set. My advice would be to put the data folder under some version
control system such as git.

Speaking of data folder, at the first application start, the default data folder
and the default configuration file are created. They are, respectively `./data`
and `budgr.toml`.
