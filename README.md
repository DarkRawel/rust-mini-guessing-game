# 🎮 Rust Guessing Game

Welcome to the **Rust Guessing Game**!  
A simple but fun number guessing game built in Rust where the computer picks a number between 1 and 100 — and you have just **5 attempts** to guess it!

---

## ✨ Features

- 🎲 Random number between **1 and 100**
- 💬 Live feedback: _Too high_, _Too low_, or _Correct!_
- ❌ Quit anytime with `'q'`
- 🔁 Auto-restart after each game
- 🧠 Input validation and error-proof

---

## ▶ How to Play

1. Run the program.
2. Enter a number between 1 and 100.
3. You get 5 tries — good luck!
4. Type `q` to quit at any time.

---

## 🚀 Run the Game

Make sure you have Rust and Cargo installed. Then run:

```bash
cd rust_guessing_game
cargo run
```

---

## 🗂 File Structure

```
rust_guessing_game/
├── src/
│   └── main.rs         # Game logic code
├── Cargo.toml          # Project metadata and dependencies
├── Cargo.lock          # Automatically generated lock file
└── README.md           # You're here!
```

---

## 🛠 Dependencies

- `rand` crate for generating random numbers.
- Standard `std::io` for user input.

To add `rand`, it's already included in your `Cargo.toml`:
```toml
[dependencies]
rand = "0.8"
```

---

## 🧠 Behind the Code

The game uses a loop labeled `'game` to allow full game restarts.  
Input is read as a string and parsed. The loop counts attempts and checks whether the player guessed correctly. Here's a short snippet from `main.rs`:

```rust
if guess == secret {
    println!("You guessed correctly! The secret number is {}!", secret);
    won = true;
    break;
} else if guess > secret {
    println!("Your guess of {} is **too high**.", guess);
} else {
    println!("Your guess of {} is **too low**.", guess);
}
```

---

## 💡 Ideas for Improvements

- Add difficulty levels (easy = 10 tries, hard = 3 tries)
- Track number of wins/losses
- Save game history to a file
- GUI version using egui or TUI with `crossterm`

---

### 📜 License

MIT — Free to use, modify, and share.  
Have fun and keep coding in Rust! 🦀🔥
