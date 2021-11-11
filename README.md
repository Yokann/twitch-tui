# Twitch chat in the terminal.

### What it looks like:

![image](https://user-images.githubusercontent.com/15021300/133889088-7ec17848-b6c2-4e80-8dea-47f4b5b9553a.png)

### Keybinds:
<details>
  <summary>Normal mode</summary>

  | Key   | Description                                                                                         |
  |-------|-----------------------------------------------------------------------------------------------------|
  | `c`   | Go to the chat window chat.                                                                         |
  | `i`   | Enter input mode for sending messages. Exit this mode with `Esc`.                                   |
  | `?`   | Have the keybinds window appear.                                                                    |
  | `q`   | Quit out of the entire application.                                                                 |
  | `Esc` | Exits out of layered windows, such as going from input mode, to normal, to exiting the application. |


</details>

<details>
  <summary>Input mode</summary>

  | Key           | Description                                                 |
  |---------------|-------------------------------------------------------------|
  | `Ctrl + w`    | Cuts a single word (from the cursor to the next whitespace) |
  | `Ctrl + u`    | Cuts the entire line                                        |
  | `Ctrl + f`    | Move cursor to the right                                    |
  | `Ctrl + b`    | Move cursor to the left                                     |
  | `Ctrl + a`    | Move cursor to the start                                    |
  | `Ctrl + e`    | Move cursor to the end                                      |
  | `Alt + f`     | Move to the end of the next word                            |
  | `Alt + b`     | Move to the start of the previous word                      |
  | `Ctrl + t`    | Swap previous item with current item                        |
  | `Alt + t`     | Swap previous word with current word                        |
  | `Ctrl + u`    | Remove everything before the cursor                         |
  | `Ctrl + k`    | Remove everything after the cursor                          |
  | `Ctrl + w`    | Remove the previous word                                    |
  | `Ctrl + d`    | Remove item to the right                                    |
  | `Tab`         | Cycle right through the input tabs                          |
  | `Shift + Tab` | Cycle left through the input tabs                           |
  | `Enter`       | Confirm the input text to go through                        |
  | `Esc`         | Drop back to previous window layer                          |

</details>


### Setup:

1. Make sure you have Cargo installed from [rust-lang website](https://www.rust-lang.org/learn/get-started). Make sure the Cargo binary folder is appended to your `$PATH` environment variable.
2. Get an OAuth token from [Twitch](https://twitchapps.com/tmi/), and have it ready to put into the `token` variable in the `config.toml` file that you create. This `config.toml` file should be created in `~/.config/twt/config.toml` if you're on Linux or MacOS. If on Windows, place it in `%appdata%\twt\config.toml` instead.
3. Run `cargo install twitch-tui` and follow the instructions that it prints.
5. You should now be able to run `twt` from anywhere now. To update in the future, run `cargo install twitch-tui`. Have fun!

### More information:

- This project used to be named `terminal-twitch-chat`, but was renamed to `twitch-tui` in version [1.2.2](https://github.com/Xithrius/twitch-tui/releases/tag/v1.2.2).
