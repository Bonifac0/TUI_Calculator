# TUI Calculator

An advanced **CASIO-inspired scientific calculator** that runs in your terminal — built in Rust with [Ratatui](https://github.com/ratatui-org/ratatui) and [Crossterm](https://github.com/crossterm-rs/crossterm).

![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)
![AUR](https://img.shields.io/aur/version/tui_calculator)

---

## Features

- **2D Math Rendering** — stacked fractions, square roots, and matrix blocks rendered visually inside the terminal input canvas
- **LaTeX Syntax** — type `\frac{1}{2}`, `\sqrt{x}`, `\sin`, `\begin{matrix}...\end{matrix}` directly
- **Matrix & Vector Algebra** — determinant, inverse, eigenvalues, eigenvectors, dot/cross products, norm
- **Scientific Functions** — trig, inverse trig, hyperbolic, logarithms, factorial, combinations, permutations
- **4-Way Viewport Scrolling** — scroll arrows appear when expressions overflow the visible canvas
- **Responsive Layouts** — auto-selects Big / Medium / Small / Tiny mode based on your terminal size
- **Variable Registers** — store results in `A`–`F`, always have `ans` from the last evaluation
- **Non-Interactive CLI Mode** — pipe expressions in scripts: `tui_calculator "sin(30deg) + pi"`
- **XDG Config File** — `~/.config/tui_calculator/config.toml` for angle unit, precision, theme, layout

---

## Installation

### Arch Linux — AUR

```bash
# Using yay
yay -S tui_calculator

# Using paru
paru -S tui_calculator

# Manually
git clone https://aur.archlinux.org/tui_calculator.git
cd tui_calculator
makepkg -si
```

### From Source (requires Rust)

```bash
git clone https://github.com/Bonifac0/TUI_Calculator.git
cd TUI_Calculator
cargo build --release
sudo install -Dm755 target/release/tui_calculator /usr/local/bin/tui_calculator
```

---

## Usage

### Interactive TUI mode

```bash
tui_calculator          # auto layout
tui_calculator -b       # force Big layout
tui_calculator -m       # force Medium layout
tui_calculator -s       # force Small layout
```

### Non-interactive / scripting mode

```bash
tui_calculator "3 * sin(30deg)"
tui_calculator "det([[1,2],[3,4]])"
tui_calculator "\frac{1}{2} + \sqrt{9}"
tui_calculator "eigenval([[4,1],[2,3]])"
```

Output goes to `stdout`, exit code `0` on success, `1` on error.

---

## Key Bindings

| Key | Action |
|---|---|
| `Enter` / `=` | Evaluate expression |
| `Backspace` | Delete character or whole token (e.g. `sin(`) |
| `← →` | Move cursor |
| `↑ ↓` | Navigate inside fractions / scroll canvas |
| `Home` / `End` | Jump to start / end |
| `Tab` | Toggle DEG ↔ RAD |
| `Shift + A…F` | Store `ans` into variable A–F |
| `A…F` | Insert variable into expression |
| `?` / `F1` | Open help modal |
| `q` / `Ctrl+C` | Quit |

---

## LaTeX Support

Both TUI and non-interactive mode accept standard LaTeX math syntax:

```
\frac{num}{den}          → stacked fraction
\sqrt{x}                 → square root
\sin  \cos  \tan         → trig functions
\arcsin  \arccos  \arctan
\det  \vec{v}
\begin{matrix} 1 & 2 \\ 3 & 4 \end{matrix}
\begin{bmatrix} ... \end{bmatrix}
```

---

## Supported Functions

| Category | Functions |
|---|---|
| Trigonometric | `sin`, `cos`, `tan`, `asin`, `acos`, `atan` |
| Hyperbolic | `sinh`, `cosh`, `tanh` |
| Logarithmic | `ln`, `log10`, `log2` |
| Powers & roots | `sqrt`, `root(n, x)`, `^` |
| Combinatorics | `!` (factorial), `nCr`, `nPr` |
| Other | `abs`, `%` (modulo) |
| Matrix / Vector | `det`, `inv`, `eigenval`, `eigenvec`, `dot`, `cross`, `norm` |
| Constants | `pi`, `e` |

---

## Configuration

The config file is created at `~/.config/tui_calculator/config.toml` on first run.

```toml
# Angle unit: "deg" or "rad"
angle_unit = "deg"

# Decimal places in output
precision = 6

# Layout override: "auto", "big", "medium", "small"
default_layout = "auto"

# Color theme: "dark", "light", "dracula", "monokai"
theme = "dark"

# Enable 2D LaTeX rendering
latex_rendering = true
```

---

## Layout Modes

| Mode | When used |
|---|---|
| **Big** | Wide terminal — full 3-column layout with variables panel |
| **Medium** | Mid-size terminal — expression + keypad, no variables panel |
| **Small** | Compact — expression + basic number pad |
| **Tiny** | Very short terminal — expression canvas and result only |

All keyboard shortcuts remain functional in every mode regardless of which buttons are visible.

---

## Project Structure

```
src/
├── main.rs         # CLI entry, terminal lifecycle
├── app/            # App state, editor model, history, variables
├── config/         # Clap CLI flags, TOML config loader
├── parser/         # Lexer + Pratt AST parser (standard & LaTeX)
├── eval/           # Math evaluator, nalgebra matrix engine
├── ui/             # Ratatui layout, canvas, keypad, help modal
└── input/          # Crossterm event loop, AppAction mapping
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full module breakdown and data-flow diagram.

---

## License

[MIT](LICENSE) © 2026 Bonifac0
