# wsl-dirutils
The tool that wraps directory/path-related commands in wslpath.

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
![image](https://github.com/AsPulse/wsl-dirutils/assets/84216737/cfe4f0cb-8b06-4a84-abb6-392587ab0d40)


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

`wsl-dirutils pwd <path>` outputs the path of current directory to stdout.  
If the current directory is under windows such as `/mnt/c/...`, the wsl-dirutils outputs both styles to stderr.
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


