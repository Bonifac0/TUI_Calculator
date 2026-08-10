# Changelog

<!--- Types of changes:
Added-      for new features.
Changed-    for changes in existing functionality.
Deprecated- for soon-to-be removed features.
Removed-    for now removed features.
Fixed-      for any bug fixes.
Hint how to make changelogs: https://keepachangelog.com/en/1.0.0/ --->

## TODO
- Add new changes under [Unreleased].

## [Unreleased]

<!--- for documenting not yet tagged changes --->

### Fixed
- Moved clear input to `Del` and all clear to `Shift+Del` so `C` can be used as a variable key.
- Disallowed symbols (`~`, `@`, `#`, `$`, `&`, etc.) now show a footer warning instead of being silently inserted or causing parse errors.
- `Esc` / `q` now closes the help modal first if it is open, instead of quitting the app directly.
- Fraction insertion now consumes the full numeric token around the cursor (left and right), so inserting `a/b` inside `12345` produces `\frac{12345}{}` instead of splitting the number.
- Vertical navigation now moves out of nested fractions correctly while preserving visual X position (e.g. from inner numerator up to outer numerator).
### Added
- Direct keypad buttons for `(`, `)`, `[`, and `]`.
- Vim-style `h`, `j`, `k`, `l` cursor movement shortcuts.
- Variable keys `A`–`F` always inserted as uppercase regardless of shift state.
- Typing non-variable letters (`G`–`Z`) or unhandled keys now shows a warning in the footer bar instead of quitting.
- Warnings displayed in the footer bar (yellow, under the keypad); auto-clear on next valid input.
- Symbol allowlist: only characters meaningful to the parser (`0`–`9`, `.`, `+`, `-`, `*`, `/`, `%`, `^`, `!`, `(`, `)`, `[`, `]`, `{`, `}`, `,`, `;`, `\`, `×`, `÷`, `√`) can be typed directly.

## [0.1.0] - 2026-07-27

### Added
- Initial release.
