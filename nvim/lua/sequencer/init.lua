local M = {}

M.eval_selection = function()
  local bufnr = vim.api.nvim_get_current_buf()
  local start_line = vim.fn.getpos("'<")[2]
  local end_line = vim.fn.getpos("'>")[2]
  local lines = vim.api.nvim_buf_get_lines(bufnr, start_line - 1, end_line, false)
  local input = table.concat(lines, "\\n")

  local job = vim.fn.jobstart({ "cargo", "run", "--quiet" }, {
    stdout_buffered = true,
    on_stdout = function(_, data)
      if data then
        vim.api.nvim_echo({ { table.concat(data, "\\n"), "Normal" } }, false, {})
      end
    end,
    stderr_buffered = true,
    on_stderr = function(_, data)
      if data then
        vim.api.nvim_err_writeln(table.concat(data, "\\n"))
      end
    end,
  })

  vim.fn.chansend(job, input .. \"\\n\")
end

return M

