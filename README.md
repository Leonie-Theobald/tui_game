# Room - TUI-based 2D Riddle Game (Birthday Gift)
## Prerequites
- Install Rust: Follow instructions on [official Rust page](https://rust-lang.org/tools/install/)
- Open an ANSI terminal (e.g. the terminal application on MacOS or Linux (required by dependency 'termion')

## Start Game For the First Time
- Start game with `cargo run`
- Select `Start new game`
- Read game instructions
- Continue to start with actual game

### Use Docker Desktop with `devcontainer`
- Run `devcontainer up --workspace-folder .` to build and start container
- Run `devcontainer exec --workspace-folder . bash` to enter container
- Start game with `cargo run`

## Continue Existing Game
> Whenever a running game is stopped with exit key binding (`ESC`), the current game state is stored into `/quick_saves/quick_save`
- Start game with `cargo run`
- Select `Load existing game`
- Game automatically starts where you left off

## Goals
- Solve quests (check current one with `q`)
- Find your birthday cake
