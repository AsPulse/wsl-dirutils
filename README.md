# wsl-dirutils
The tool that wraps directory/path-related commands in wslpath.

<br />

## Installation
```
cargo install wsl-dirutils
```
This tool is intended to be used from shell functions and shell aliases;  
see Example Config and How to use...

<br />

## Example Config

After installing wsl-dirutils, append the below code to your `.zshrc`.
```zsh
function cd() {
  builtin cd "$(wsl-dirutils convert "$*")"
}
function pwd() {
  wsl-dirutils pwd > /dev/null
}
```
![image](https://github.com/AsPulse/wsl-dirutils/assets/84216737/c47883a1-a4dd-4840-86c6-5308797355be)



<br />

## How to use?

`wsl-dirutils convert <path>` outputs the path for Linux to stdout.  
Stderr is used to output messages for the user.

```bash
$ wsl-dirutils convert "C:\Users\aspulse"

stdout:
/mnt/c/Users/aspulse

stderr:
⊘ wsl-dirutils Converting... C:\users\aspulse
 ⮑ /mnt/c/users/aspulse
```

`wsl-dirutils pwd <path>` outputs the path of the current directory to stdout.  
If the current directory is under Windows such as `/mnt/c/...`, the wsl-dirutils outputs both styles to stderr.
```bash
$ wsl-dirutils pwd

stdout:
/mnt/c/Users/aspulse

stderr:
⊘ wsl-dirutils
 ⮑  /mnt/c/Users/aspulse (linux)
 ⮑  C:\Users\aspulse (win)
```

<br />

## Thanks to wsl-path-rust! 

We express our gratitude to [wsl-path-rust](https://github.com/pratikpc/wsl-path-rust), the crate that calls wslpath.exe from Rust!  
It made it really easy to implement...!

<br />

## Author

~~Twitter~~ X: [@\_AsPulse\_](https://x.com/_AsPulse_)


