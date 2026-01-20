# VEX - Vim Movement Trainer

A CLI-based interactive tool for learning Vim movement commands through guided practice. VEX teaches Vim's motion commands through structured lessons with hands-on practice tasks and immediate feedback.

## Features

- **6 Progressive Lessons**: From basic hjkl movement to complex multi-line navigation
- **Interactive Terminal UI**: Clean, intuitive interface built with Ratatui
- **Strict Validation**: Learn the exact commands being taught with immediate feedback
- **Progressive Hints**: Get help when stuck with up to 3 hints per task
- **Real Vim Behavior**: Implements authentic Vim word boundary and motion logic

## Installation

### Prerequisites

- Rust 1.70 or later
- A terminal emulator (any standard terminal works)

### Building from Source

```bash
git clone <repository-url>
cd vex
cargo build --release
```

### Running

```bash
cargo run --release
```

Or after building:

```bash
./target/release/vex
```

## Usage

### Main Menu

When you start VEX, you'll see the main menu with 6 available lessons:

1. Basic Movement (hjkl)
2. Word Movement (wbe)
3. Line Extremes (0$)
4. Combining Movements
5. Vertical Movement (gg, G)
6. Complete Movement Practice

Press **1-6** to select a lesson, or **q** to quit.

### In Lessons

Each lesson teaches specific Vim commands through practice tasks:

- **Complete each task** by using the exact commands taught
- **Visual cursor indicator** shows your current position (green highlight)
- **Immediate feedback** after each command
- Press **r** to reset the current task (start over from the beginning)
- Press **?** to show hints (progressive hints available)
- Press **ESC** to return to the main menu
- Press **Ctrl+Q** to quit the application

### Commands Taught

#### Lesson 1: Basic Movement
- `h` - move left
- `j` - move down
- `k` - move up
- `l` - move right

#### Lesson 2: Word Movement
- `w` - forward to start of next word
- `b` - backward to start of previous word
- `e` - forward to end of word

#### Lesson 3: Line Extremes
- `0` - jump to start of line
- `$` - jump to end of line

#### Lesson 4: Combining Movements
Practice chaining commands from previous lessons efficiently.

#### Lesson 5: Vertical Movement
- `gg` - jump to first line of file
- `G` - jump to last line of file

#### Lesson 6: Complete Practice
Apply all learned movements in realistic scenarios.

## Technical Details

### Architecture

```
src/
├── main.rs           # Entry point and terminal setup
├── app.rs            # Main application state and event handling
├── vim/              # Vim simulation engine
│   ├── buffer.rs     # Text buffer and cursor management
│   ├── command.rs    # Command types and enums
│   ├── parser.rs     # Keystroke to command parsing
│   └── executor.rs   # Command execution on buffer
├── learning/         # Learning system
│   ├── lesson.rs     # Lesson and task structures
│   ├── curriculum.rs # All 6 lessons with content
│   └── validator.rs  # Strict command validation
└── ui/               # Terminal user interface
    ├── layout.rs     # Panel layouts
    ├── components.rs # UI rendering components
    └── events.rs     # Input event handling
```

### Technology Stack

- **Language**: Rust (for safety, performance, and excellent error handling)
- **TUI Framework**: [Ratatui](https://github.com/ratatui-org/ratatui) - Modern terminal UI library
- **Terminal Backend**: [Crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- **CLI Parsing**: [Clap](https://github.com/clap-rs/clap) - Command-line argument parser

### Word Boundary Behavior

VEX implements authentic Vim word boundary detection:
- Alphanumeric characters and underscores form words
- Punctuation is treated as separate words
- Example: `foo.bar` contains 3 words: `foo`, `.`, `bar`

## Development

### Running Tests

```bash
cargo test
```

All core functionality (buffer operations, motion execution, command parsing, validation) has unit test coverage.

### Building for Development

```bash
cargo build
cargo run
```

### Code Style

This project follows standard Rust conventions:
- Run `cargo fmt` to format code
- Run `cargo clippy` for linting

## Future Enhancements (V2+)

Planned features for future versions:
- Editing commands (d, c, y, i, a, p)
- Operator + motion combinations
- Flexible validation mode with efficiency scoring
- Progress persistence between sessions
- Challenge mode with randomized tasks
- Statistics tracking

## Troubleshooting

### Terminal Display Issues

If the UI doesn't render correctly:
- Ensure your terminal supports ANSI colors
- Try resizing the terminal (minimum 80x24 recommended)
- Test with a different terminal emulator

### Keyboard Input Not Working

- Make sure you're using the exact keys shown (case-sensitive)
- For `gg`, press `g` twice quickly
- Check that your terminal isn't intercepting certain key combinations

## License

[To be determined]

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Acknowledgments

Built as a tool for developers learning Vim movement commands with immediate, interactive feedback.

---

**Happy Vim Learning!**
