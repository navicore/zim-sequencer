# 🎛️ Zim Sequencer

**Zim** is a text-based musical calculator and sequencer designed for modular synthesists and experimental musicians. Write music as mathematical expressions, hear harmonic relationships instantly, and explore microtonal spaces - all from your text editor.

![Status](https://img.shields.io/badge/status-experimental-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## ✨ Features

- **🎵 Musical DSL** - Express notes, chords, and transformations as text
- **🔬 Harmonic Analysis** - Real-time interval and chord identification  
- **🎹 Built-in Synth** - Instant audio feedback with pure sine waves
- **🌈 Microtonal Support** - Work in cents, frequency ratios, or any EDO
- **🧮 Mathematical Transformations** - Transpose, invert, multiply frequencies
- **📝 REPL Workflow** - R-style interactive evaluation in Neovim

## 🚀 Quick Start

```bash
# In a .zim file:
C E G !          # Play C major chord
C E G +2 !       # Transpose up 2 semitones
C *1.5 !         # Perfect fifth by frequency ratio
C+25 E-14 G+2 !  # Microtonal adjustments
```

## 📦 Installation

### Prerequisites
- Neovim 0.8+
- Rust toolchain (for building the engine)
- Audio output device

### Using lazy.nvim

```lua
{
  "navicore/zim-sequencer",
  dependencies = { "nvim-lua/plenary.nvim" },
  build = "lua build.lua",
  ft = { "zim", "mel" },
}
```

## 🎮 Usage

### Basic Workflow

1. Open a `.zim` or `.mel` file
2. Write musical notation
3. Press `Enter` to evaluate and hear the current line
4. Press `<leader>e` to re-evaluate without moving

### Keybindings

| Mode | Key | Action |
|------|-----|--------|
| Normal | `<CR>` | Play current line and advance |
| Normal | `<leader>e` | Play current line (stay) |
| Visual | `<CR>` | Play selection as sequence |
| Visual | `<leader>e` | Evaluate selection |

### DSL Reference

#### Notes and Chords
```bash
C                # Single note (C4 by default)
C4 E4 G4         # Chord with octaves
C E G            # Chord (implicit octave 4)
C+25             # C plus 25 cents
C4-14            # C4 minus 14 cents
```

#### Transformations
```bash
C E G +2         # Transpose up 2 semitones
C E G -7         # Transpose down 7 semitones
C E G inv        # Invert around first note
C E G reverse    # Reverse note order
C E G spread     # Spread across octaves
```

#### Microtonal Operations
```bash
C *1.5           # Multiply frequency by 1.5 (perfect fifth)
C *1.25          # Major third (5:4 ratio)
C just           # Generate just intonation major scale
C edo19          # 19-tone equal temperament scale
C edo31          # 31-EDO (great for 5-limit JI)
C E G 25c        # Shift all notes up 25 cents
```

#### Audio Control
```bash
C E G !          # Play chord (800ms)
C E G !!         # Play longer (2000ms)
!                # Replay last notes
!stop            # Stop playback
```

#### Comments
```bash
C E G !          # This is a comment
# Full line comment
```

## 🎼 Examples

### Chord Progressions
```bash
# Classic I-IV-V-I
C E G !
F A C !
G B D !
C E G !
```

### Microtonal Exploration
```bash
# Compare equal temperament vs just intonation
C E G !          # Equal temperament
C E-14 G+2 !     # Just intonation approximation
C just !         # Full harmonic series
```

### Mathematical Sequences
```bash
# Harmonic series
C !
C *2 !           # Octave
C *3 !           # Fifth above octave
C *4 !           # Two octaves
C *5 !           # Major third above two octaves
```

## 🔧 Architecture

Zim consists of three main components:

1. **Rust Engine** (`engine/`) - Handles music theory calculations, DSL parsing, and audio synthesis
2. **Neovim Plugin** (`lua/`) - Provides the interactive REPL interface
3. **DSL** - A minimal language optimized for musical exploration

The engine runs as a subprocess, communicating via stdin/stdout for low latency.

## 🎯 Design Philosophy

Zim is designed as a "musical calculator" rather than a traditional sequencer. It emphasizes:

- **Immediate feedback** - Hear what you write instantly
- **Mathematical thinking** - Music as numbers and ratios
- **Exploration** - Discover new harmonic relationships
- **Simplicity** - Minimal syntax, maximum expressiveness

## 🚧 Roadmap

- [x] Basic note/chord evaluation
- [x] Interval and chord analysis
- [x] Built-in audio synthesis
- [x] Microtonal support
- [x] Mathematical transformations
- [ ] MIDI output
- [ ] Dissonance metrics
- [ ] Polyrhythmic patterns
- [ ] Probabilistic sequences
- [ ] Pattern storage/recall

## 🤝 Contributing

This is an experimental project in active development. Issues and PRs welcome!

## 📄 License

MIT - See [LICENSE](LICENSE) file

## 🙏 Acknowledgments

Inspired by:
- TidalCycles and live coding environments
- Modular synthesis workflows
- R's interactive REPL (via R-nvim)
- Just intonation and microtonal music theory

---

*Built for musicians who think in frequencies, ratios, and mathematical relationships.*