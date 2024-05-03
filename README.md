# Rush!

Rush stands for ***Rust Shell***.
It is a very basic shell I wrote to get a little familiar with rust.  

Now, I have come to understand that because this is a program written in ***RUST*** that I must make some claims here.

This shell is :fire: :rocket: ***SUPER FAST*** :rocket: :fire: and :lock: ***SAFE*** :lock:

sorry

## Builtin

The few system calls I built into the shell

### cd

`cd` Is just a basic plain-old cd to change directories whenever you feel like it, hopefully with some helpful error messages. 

### pwd

`pwd` will display the current dir.

### hostname

`hostname` will display the hostname, the crate I used says it will work on Windows, but I never checked.

### mkdir

just calling mkdir will make a directory but will not create the parents.
it will accept the flag `-p` to make the parent directories.

``` bash
  # normal mkdir
  mkdir foo
  # make parents
  mkdir -p foo/bar
```
### exit

It exits, that is it.

## everything else!

will spawn a process and execute whatever program you call.
