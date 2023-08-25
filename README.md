# wsl-dirutils
The tool that wraps directory/path-related commands in wslpath.

<br />

## Example Config

After installed wsl-dirutils, append below code to your `.zshrc`.
```zsh
function cd() {
  builtin cd "$(wsl-dirutils convert "$*")"
}
```

<br />

## How to use?

`wsl-dirutils convert <path>` output the path for linux to stdout.
Stderr is used to output message for user.

```bash
$ wsl-dirutils convert "C:\Users\aspulse"

stdout:
/mnt/c/Users/aspulse

stderr:
⊘ wsl-dirutils Converting... C:\users\aspulse
 ⮑ /mnt/c/users/aspulse
```

<br />

## Thanks to wsl-path-rust! 

We express our gratitude to [wsl-path-rust](https://github.com/pratikpc/wsl-path-rust), the crate that calls wslpath.exe from Rust!
It made it really easy to development...!

<br />

## Author

~~Twitter~~ X: [@\_AsPulse\_](https://x.com/_AsPulse_)


