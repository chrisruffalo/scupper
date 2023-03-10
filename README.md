# Scupper
> a hole in a ship's side to carry water overboard from the deck.

## Purpose
To quickly and easily send small files, using ngrok, with rust. This is probably stupid.

## Usage
- Get an account at [ngrok](https://ngrok.com/)
- Set your token to the environment variable NGROK_AUTHTOKEN
```bash
[]$ export NGROK_AUTHTOKEN=<redacted>
```
- Run with cargo, accepts one argument (a path to a file you want to serve)
```bash
[]$ cargo run ~/Downloads/rickroll.gif
Serving "/home/user/Downloads/rickroll.gif" on URL: "https://9734-86-19-22-44.ngrok.io"
```
- The file will be accessible to anyone who clicks on the
- RIP your ngrok bandwidth
