
to enable plugin in neovom

```lua
vim.keymap.set("v", "<leader>e", function()
  require("zim-sequencer.nvim").eval_selection()
end, { desc = "Eval DSL selection" })
```


