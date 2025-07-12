local M = {}

local engine_path = vim.fn.stdpath("data") .. "/lazy/zim-sequencer/engine/target/release/zim-sequencer"

M.setup = function()
	print("[zim-sequencer] setup called")
	print("[zim-sequencer] engine path: " .. engine_path)

	if vim.fn.executable(engine_path) == 0 then
		vim.notify("[zim-sequencer] Engine not found or not executable", vim.log.levels.ERROR)
		return
	end

	print("[zim-sequencer] starting engine...")

	M.job_id = vim.fn.jobstart({ engine_path }, {
		stdout_buffered = true,
		stderr_buffered = true,

		on_stdout = function(_, data)
			print("[zim-sequencer] engine stdout:")
			print(vim.inspect(data))
			if data then
				vim.notify(table.concat(data, "\n"))
			end
		end,

		on_stderr = function(_, data)
			print("[zim-sequencer] engine stderr:")
			print(vim.inspect(data))
			if data then
				vim.notify(table.concat(data, "\n"), vim.log.levels.ERROR)
			end
		end,
	})
end

M.eval_selection = function()
	print("[zim-sequencer] eval_selection called")

	if not M.job_id then
		vim.notify("[zim-sequencer] Engine not running", vim.log.levels.WARN)
		return
	end

	local start_line = vim.fn.getpos("'<")[2]
	local end_line = vim.fn.getpos("'>")[2]
	local lines = vim.api.nvim_buf_get_lines(0, start_line - 1, end_line, false)
	local input = table.concat(lines, "\n")

	print("[zim-sequencer] sending input:\n" .. input)

	vim.fn.chansend(M.job_id, input .. "\n")
end

vim.keymap.set("v", "<leader>e", M.eval_selection, {
	desc = "Evaluate Zim block",
	noremap = true,
	silent = true,
})

return M
