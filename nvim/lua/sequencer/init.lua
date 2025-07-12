local M = {}
local engine_path = vim.fn.stdpath("data") .. "/zim-sequencer-bin/zim-sequencer"
local job_id = nil

M.setup = function()
	if vim.fn.filereadable(engine_path) == 0 then
		vim.notify("zim-sequencer engine not found. Did it build correctly?", vim.log.levels.ERROR)
		return
	end

	job_id = vim.fn.jobstart({ engine_path }, {
		stdout_buffered = true,
		on_stdout = function(_, data)
			vim.notify(table.concat(data, "\n"), vim.log.levels.INFO)
		end,
		on_stderr = function(_, data)
			vim.notify(table.concat(data, "\n"), vim.log.levels.ERROR)
		end,
	})
end

M.eval_selection = function()
	if not job_id then
		vim.notify("zim-sequencer engine is not running", vim.log.levels.WARN)
		return
	end

	local start_line = vim.fn.getpos("'<")[2]
	local end_line = vim.fn.getpos("'>")[2]
	local lines = vim.api.nvim_buf_get_lines(0, start_line - 1, end_line, false)
	vim.fn.chansend(job_id, table.concat(lines, "\n") .. "\n")
end

vim.keymap.set("v", "<leader>e", M.eval_selection, { desc = "Eval zim-sequencer block" })

return M
