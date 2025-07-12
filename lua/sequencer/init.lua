local M = {}

-- define the path to the binary
local engine_path = vim.fn.stdpath("data") .. "/lazy/zim-sequencer/engine/target/release/zim-sequencer"

-- setup function
M.setup = function()
	if vim.fn.executable(engine_path) == 0 then
		vim.notify("[zim-sequencer] Engine binary not found or not executable: " .. engine_path, vim.log.levels.ERROR)
		return
	end

	M.job_id = vim.fn.jobstart({ engine_path }, {
		stdout_buffered = true,
		on_stdout = function(_, data)
			if data then
				vim.notify(table.concat(data, "\n"))
			end
		end,
		stderr_buffered = true,
		on_stderr = function(_, data)
			if data then
				vim.notify(table.concat(data, "\n"), vim.log.levels.ERROR)
			end
		end,
	})

	-- ✅ define the keymap *after* M.eval_selection exists
	vim.keymap.set("v", "<leader>e", M.eval_selection, {
		desc = "Evaluate Zim block",
		noremap = true,
		silent = true,
	})
end

-- define eval_selection before M.setup() uses it
M.eval_selection = function()
	if not M.job_id then
		vim.notify("[zim-sequencer] Engine is not running", vim.log.levels.WARN)
		return
	end

	local start_line = vim.fn.getpos("'<")[2]
	local end_line = vim.fn.getpos("'>")[2]
	local lines = vim.api.nvim_buf_get_lines(0, start_line - 1, end_line, false)
	local input = table.concat(lines, "\n")
	vim.fn.chansend(M.job_id, input .. "\n")
end

return M
