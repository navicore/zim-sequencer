# 🧬 Zim Sequencer

**Zim** is a text-based live sequencer for modular and experimental musicians. It combines a custom DSL with a harmony-aware Rust engine and a Neovim plugin that lets you compose and explore music directly from your editor.

- ✍️ Write sequences in plain text
- 🧠 Get feedback on chord and interval structures
- 🔄 Play sequences via MIDI (coming soon)
- ⚡ Integrated REPL workflow inside Neovim

---

## 📦 Installation

Add this to your Lazy.nvim plugin list:

```lua
{
  "navicore/zim-sequencer",
  dependencies = { "nvim-lua/plenary.nvim" },
  build = "lua build.lua",
  config = function()
    require("sequencer").setup()
  end
}
```


