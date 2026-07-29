# TUI Calculator - Feature Specification & Architecture

## Overview
An advanced, CASIO-inspired Scientific Terminal UI (TUI) Calculator built in **Rust** using **Ratatui** and **Crossterm**. The application features full mouse and keyboard interaction, 2D math layout rendering (stacked fractions, matrix blocks), LaTeX expression support, 2D viewport navigation, dynamic responsive layouts, comprehensive scientific functions, matrix/vector algebra, and a non-interactive CLI mode suitable for scripting and Arch Linux (AUR) distribution.

---

## 1. Input & Expression Editing

### 1.1 Visual Rendering & 2D Math Formatting
- **2D Math Representation**: Supports multi-line 2D layout rendering for stacked fractions ($\frac{\text{num}}{\text{den}}$), square roots ($\sqrt{x}$), exponents, and multi-row matrix blocks inside the input display window.
- **LaTeX Expression Syntax Support**:
  - Accepts standard LaTeX mathematical markup in both interactive TUI mode and non-interactive CLI mode.
  - Examples: `\frac{1}{2}`, `\sin(\frac{\pi}{4})`, `\sqrt{16}`, `\begin{matrix} 1 & 2 \\ 3 & 4 \end{matrix}`, `\vec{v}`.
- **2D Viewport Scrolling (Horizontal & Vertical)**:
  - When expressions overflow the input screen width or height (e.g., tall stacked fractions or large matrices), the viewport pans automatically to keep the cursor centered.
  - **4-Directional Overflow Indicators**: Visual arrows (`▲`, `▼`, `◀`, `▶`) appear along the viewport borders to indicate off-screen mathematical content.
  - **Up/Down Arrow Navigation**: Users can navigate multiline/2D expressions up and down using Up/Down arrow keys or mouse scroll.
- **Precision Cursor Navigation**:
  - Cursor movement via arrow keys (Left, Right, Up, Down).
  - Direct Mouse Position Click: Clicking any character/symbol in the rendering canvas moves the editing cursor directly to that character index.
- **Structured Fraction Editing (2D, not plain text placeholder)**:
  - Pressing the fraction button inserts a true fraction node with editable numerator and denominator regions.
  - The input canvas renders the fraction as multiline stacked layout (`numerator`, horizontal bar, `denominator`) instead of showing only `\frac{}{}`
  - Up/Down navigation switches between numerator and denominator while preserving horizontal visual position.
  - If denominator is empty and Backspace is pressed from denominator start, the fraction collapses back to inline numerator text (reverse of fraction insertion).
- **Baseline Inline Alignment**:
  - Content outside fractions stays aligned with the fraction bar line for easier reading.
  - Multiple inline fractions align to the same baseline unless nesting/layout context requires otherwise.
- **Token-Aware Backspace for Inserted Functions**:
  - Function snippets inserted as predefined tokens (e.g. `sin(`, `cos(`, `asin(`, `det(`, `eigenval(`, `norm(`, `log(`, `ln(`, `ans`, `pi`, `√(`) are removed in one Backspace press when the cursor is directly after the token.
  - Regular text editing still supports single-character Backspace behavior.

### 1.2 Matrix & Vector Inline & LaTeX Syntax
- **Inline Array Syntax**: `[[1, 2], [3, 4]]` or `[1, 2; 3, 4]`.
- **LaTeX Matrix Syntax**: `\begin{matrix} 1 & 2 \\ 3 & 4 \end{matrix}` or `\begin{bmatrix} ... \end{bmatrix}`.
- **Matrix / Vector Operations**:
  - Matrix addition, subtraction, scalar multiplication, and matrix multiplication (`*`).
  - Determinant: `det(A)` or `\det(A)`.
  - Matrix Inverse: `inv(A)` or `A^{-1}`.
  - Eigenvalues & Eigenvectors: `eigenval(A)`, `eigenvec(A)`.
  - Vector algebra: Dot product (`dot(u, v)`), Cross product (`cross(u, v)`), Norm/Magnitude (`norm(v)`).

---

## 2. Evaluation & Variable Management

### 2.1 Result Calculation & `ans`
- **Trigger**: Pressing `Enter` or clicking the `=` button evaluates the expression.
- **Output Box**: Displays the computed result in the top result pane.
- **Automatic `ans` Assignment**: Every evaluated result is automatically saved to `ans` for use in subsequent expressions.
- **Extra Bracket Buttons**: The basic keypad includes direct insertion buttons for `(`, `)`, `[`, and `]`.

### 2.2 User Variables (`A` through `F`)
- **Storage**: 6 user-assignable variables (`A`, `B`, `C`, `D`, `E`, `F`). Default value = `0`.
- **Assignment**:
  - `Shift + <Letter>` (or clicking `STO <Letter>`) assigns the current `ans` value (or scalar/matrix) to that variable.
- **Usage**:
  - Pressing `<Letter>` (e.g. `A`) inserts the variable into the active expression.
- **Session Lifecycle**: Variables are **in-memory only** and reset to `0` when the application exits.

---

## 3. Scientific Mathematical Functions

- **Trigonometric Functions**: `sin`, `cos`, `tan` (LaTeX: `\sin`, `\cos`, `\tan`)
- **Inverse Trigonometric**: `asin`, `acos`, `atan` (LaTeX: `\arcsin`, `\arccos`, `\arctan`)
- **Hyperbolic Functions**: `sinh`, `cosh`, `tanh`
- **Logarithmic Functions**: `ln` (natural log), `log10` (base-10 log), `log2` (base-2 log)
- **Exponents & Powers**: `^` (power), `sqrt` ($\sqrt{x}$), `root(n, x)` ($n$-th root)
- **Combinatorics & Factorial**: `!` (factorial), `nCr` (combinations), `nPr` (permutations)
- **Number Theory**: `%` (modulo), `abs` (absolute value)
- **Constants**: `pi` ($\pi \approx 3.14159265...$), `e` ($e \approx 2.71828182...$)

---

## 4. Responsive Layout System

The interface dynamically adapts to terminal window resizing across 4 distinct responsive break-points. All keyboard shortcuts remain fully functional in every layout mode, regardless of whether corresponding visual buttons are rendered.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│ Top Bar: App Name | Status (DEG/RAD, Mode) | Debug / Notification Area          │
├───────────────────────────────────────┬─────────────────────────────────────────┤
│ Expression Input (Top Left)           │ Output Result Pane (Top Right)          │
│ (2D Viewport: ▲ ▼ ◀ ▶)                │                                         │
├───────────────────┬───────────────────┴─────────────────┬───────────────────────┤
│ Basic Keypad      │ Advanced Scientific Keypad          │ Variables Table       │
│ (Numbers, Ops)    │ (Trig, Logs, Matrix Ops, LaTeX)     │ (A-F Values & STO)    │
└───────────────────┴─────────────────────────────────────┴───────────────────────┘
```

### 4.1 Layout Modes

Layout selection is automatic and follows this precedence:

- **Tiny Mode** when the terminal height is too small for the keypad stack.
- **Big Mode** when there is enough width and height for the full three-column layout.
- **Medium Mode** when there is enough space for the split keypad layout but not the full three-column view.
- **Small Mode** as the compact fallback with the keypad still visible.

The modes are:

1. **Big Mode (Full Screen)**:
   - **Top Bar**: Application title, current angle mode (`DEG`/`RAD`), layout mode indicator, and debug log/status region.
   - **Top Left**: Main expression input canvas with 2D math rendering and 4-way viewport scrolling.
   - **Top Right**: Result display panel.
   - **Bottom Left**: Number pad (`0`-`9`), decimal `.`, basic operators (`+`, `-`, `*`, `/`, `=`).
   - **Bottom Center**: Advanced scientific operators, function buttons, and LaTeX shortcuts.
   - **Bottom Right**: Live Variables Table displaying values for `ans` and `A`–`F`.

2. **Medium Mode (Half Screen / Split)**:
   - Hides the Variables Table panel to conserve horizontal space.
   - Retains Expression Canvas, Result Panel, Number Pad, and compact Advanced Functions strip.

3. **Small Mode (Minimal / Compact)**:
   - Displays Expression, Result, and the basic number pad.
   - Advanced operations and LaTeX inputs remain fully functional via keyboard shortcuts.

4. **Tiny Mode (Height-Constrained)**:
   - Displays only the Expression canvas and the Result box.
   - Hides all keypad panels to preserve vertical space when the window is too short.

---

## 5. Top Bar & Help System

- **Top Bar Component**:
  - Application title & version indicator.
  - Active Mode Badges: Angle mode (`DEG` vs `RAD`), layout indicator (`BIG`/`MED`/`SML`/`TINY`).
  - Debug / Log Message Strip for error traces, warnings, or confirmation logs.
- **Help Modal (`?` or `F1` key)**:
  - Interactive overlay window listing full keyboard shortcuts, LaTeX syntax reference, variable guide, and developer credits.

---

## 6. Configuration & System Integration

### 6.1 XDG-Compliant Configuration File
- **Path**: `~/.config/tui_calculator/config.toml`
- **Format**: TOML
- **Configurable Options**:
  ```toml
  # Angle unit: "deg" or "rad"
  angle_unit = "deg"

  # Default display precision (number of decimal places)
  precision = 6

  # Default layout mode override ("auto", "big", "medium", "small")
  default_layout = "auto"

  # Color theme ("dark", "light", "dracula", "monokai")
  theme = "dark"

  # Enable 2D LaTeX rendering by default
  latex_rendering = true
  ```

### 6.2 CLI Arguments & Non-Interactive Mode

The binary supports command-line flags for scripting, debugging, and layout enforcement:

- **Enforce Layout**:
  - `-s`, `--small` : Force Small layout.
  - `-m`, `--medium`: Force Medium layout.
  - `-b`, `--big`   : Force Big layout.
- **Information**:
  - `-h`, `--help`   : Print help menu and exit.
  - `-v`, `--version`: Print version information and exit.
- **Non-Interactive Mode**:
  - Evaluate standard or LaTeX expressions directly from shell:
    ```bash
    tui_calculator "\frac{1}{2} + \sin(30deg) + \det(\begin{matrix} 1 & 2 \\ 3 & 4 \end{matrix})"
    # Output: -1.0
    ```
  - Outputs result to `stdout` with exit code `0` (or `stderr` with exit code `1` on error).

---

## 7. Packaging & Distribution (AUR)

- **Target Package**: Arch User Repository (AUR) `tui_calculator`.
- **Artifacts Included**:
  - `PKGBUILD` script for building from source release tags.
  - Standard man page (`tui_calculator.1`).
  - Desktop launcher entry (`tui_calculator.desktop`).
