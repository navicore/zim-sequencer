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

		-- get visual selection the safest way
		local start_pos = vim.fn.getpos("'<")[2]
		local end_pos = vim.fn.getpos("'>")[2]
		local lines = vim.fn.getline(start_pos, end_pos)
		local input = table.concat(lines, "\n")

		print("[zim-sequencer] sending input:\n" .. input)

		vim.fn.chansend(M.job_id, input .. "\n")
	end)
end

M.setup = function()
	print("[zim-sequencer] setup called")

	local path = engine_path()
	if vim.fn.executable(path) == 0 then
		vim.notify("[zim-sequencer] Engine not executable: " .. path, vim.log.levels.ERROR)
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
		end,

		on_stderr = function(_, data)
			if data and #data > 0 and data[1] ~= "" then
				print("[zim-sequencer] engine stderr:")
				print(vim.inspect(data))
				vim.notify(table.concat(data, "\n"), vim.log.levels.ERROR)
			end
		end,
	})

	local buf = vim.api.nvim_get_current_buf()
	vim.keymap.set("v", "<leader>e", M.eval_selection, {
		desc = "Evaluate Zim block",
		noremap = true,
		silent = true,
		buffer = buf,
	})

	vim.b.zim_sequencer_active = true
end

vim.api.nvim_create_autocmd("FileType", {
	pattern = "zim",
	callback = function()
		require("sequencer").setup()
	end,
})

return M
