# Chinese checker

[Chinese checker](https://en.wikipedia.org/wiki/Chinese_checkers) (跳棋) is a board game that can be played by up to six players. This project grew out of my interest in both the game itself and the [Rust](https://www.rust-lang.org/) programming language. The current state of the project is that computer players can play out the game according to a fairly simple greedy-ish algorithm. The next steps including training some model to make better moves, and allowing human players.

To see a random game played out by the program, run `cargo run` from the root directory. Be aware that sometimes the game may not be able to end properly because one's pieces can be trapped by others. A succefully played-out example is shown below

![](demo.gif)

Feel free to change the game settings in `main.rs` (for example, to slow down the play to see the moves). Have fun!