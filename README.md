a wrapper around `git` named `gitim` which logs invocations, meant to be part of a personal organizer system

productivity person Elizabeth Filips advocates<sup>[&ask;]</sup> diving into new projects half-assedly as soon as possible, as soon as you know just barely enough to possibly complete them, on the basis that delaying until you're 'ready' means you might get distracted and never even start, and you can always go back and fill in gaps in your knowledge and praxis later

i was worried this would just lead to dozens of semicompleted projects (which admittedly is better than the scores of unstarted daydreams i have currently), so i thought if i could automatically keep track of what i've worked on i could write a script later to automatically highlight things i've not worked on in a while

this program is a simple wrapper around git called `gitim` which logs every time you use it to a directory named by the environmental variable TIMESHEET_DIR (set it with something like `export TIMESHEET_DIR=~/sync/timesheets/` in your `~/.zshrc` file) before passing its arguments to git

the logs are in directories named like "2024-01", in files named like "2024-01-21", and consist of lines of `YYYY-MM-DD HH:MM \t path-of-invocation \t git-command-issued` (`\t` is the tab character)

## to install

- [install rust](https://rustup.rs/)
- from inside this repository, say `cargo install --path .`

## to use

use it exactly as you would `git`, and it will log the time, current working directory, and command issued, and show you the output from git

`gitim status`
`gitim commit -am"hi"`
`gitim push`

etc

## warning: probably won't work on windows

i don't know what the command line situation is over there, i think there are <abbr title="cmd.exe, Power Shell, and the Windows Linux Subsystem (and possibly two versions of that also)">at least three</abbr> systems?

<hr>

<sup>[&ask;]</sup> <small>in [the PDF available here](https://lizziefilips.gumroad.com/l/krcnc); Filips' other two tips are to focus not on results but on the increase in capabilities which will result from doing the work, and to use that as motivation by looking at the works of others with those abilities and contemplating what other possibilities are now open to you</small>
