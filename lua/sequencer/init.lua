local M = {}

local function engine_path()
	return vim.fn.stdpath("data") .. "/lazy/zim-sequencer/engine/target/release/zim-sequencer"
end

M.eval_selection = function()
	vim.schedule(function()
		print("[zim-sequencer] eval_selection called")

		if not M.job_id then
			vim.notify("[zim-sequencer] Engine is not running", vim.log.levels.WARN)
			return
		end

		local buf = vim.api.nvim_get_current_buf()
		local start_line = vim.api.nvim_buf_get_mark(buf, "<")[1]
		local end_line = vim.api.nvim_buf_get_mark(buf, ">")[1]
		local lines = vim.api.nvim_buf_get_lines(buf, start_line - 1, end_line, false)
		local input = table.concat(lines, "\n")

		print("[zim-sequencer] sending input:\n" .. input)

		vim.fn.chansend(M.job_id, input .. "\n")
	end)
end

M.setup = function()
	print("[zim-sequencer] setup called")

	local path = engine_path()
	if vim.fn.executable(path) == 0 then
		vim.notify("[zim-sequencer] Engine not found or not executable: " .. path, vim.log.levels.ERROR)
		return
	end

	print("[zim-sequencer] starting engine: " .. path)

	M.job_id = vim.fn.jobstart({ path }, {
		stdout_buffered = true,
		stderr_buffered = true,

		on_stdout = function(_, data)
			if data and #data > 0 and data[1] ~= "" then
				print("[zim-sequencer] engine stdout:")
				print(vim.inspect(data))
				vim.notify(table.concat(data, "\n"))
			end
		end

		on_stderr = function(_, data)
			print("[zim-sequencer] engine stderr:")
			print(vim.inspect(data))
			if data then
				vim.notify(table.concat(data, "\n"), vim.log.levels.ERROR)
			end
		end,
	})

	-- Only set the keymap for this buffer
	local buf = vim.api.nvim_get_current_buf()
	vim.keymap.set("v", "<leader>e", M.eval_selection, {
		desc = "Evaluate Zim block",
		noremap = true,
		silent = true,
		buffer = buf,
	})

	vim.b.zim_sequencer_active = true
end

-- Automatically call setup only for `zim` filetype
vim.api.nvim_create_autocmd("FileType", {
	pattern = "zim",
	callback = function()
		require("sequencer").setup()
	end,
})

return M
