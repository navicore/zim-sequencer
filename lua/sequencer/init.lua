local M = {}

local output_buf = nil
local output_win = nil

local function engine_path()
	return vim.fn.stdpath("data") .. "/lazy/zim-sequencer/engine/target/release/zim-sequencer"
end

local function ensure_output_window()
	if output_win and vim.api.nvim_win_is_valid(output_win) then
		return
	end

	local current_win = vim.api.nvim_get_current_win()
	
	vim.cmd('vsplit')
	vim.cmd('wincmd l')
	
	if not output_buf or not vim.api.nvim_buf_is_valid(output_buf) then
		output_buf = vim.api.nvim_create_buf(false, true)
		vim.api.nvim_buf_set_name(output_buf, "[Zim Output]")
		vim.api.nvim_buf_set_option(output_buf, 'buftype', 'nofile')
		vim.api.nvim_buf_set_option(output_buf, 'swapfile', false)
		vim.api.nvim_buf_set_option(output_buf, 'filetype', 'zimoutput')
	end
	
	vim.api.nvim_win_set_buf(0, output_buf)
	output_win = vim.api.nvim_get_current_win()
	
	vim.api.nvim_win_set_width(output_win, math.floor(vim.o.columns * 0.4))
	
	vim.api.nvim_set_current_win(current_win)
end

local function append_to_output(lines)
	ensure_output_window()
	
	if type(lines) == "string" then
		lines = vim.split(lines, "\n")
	end
	
	local current_lines = vim.api.nvim_buf_get_lines(output_buf, 0, -1, false)
	if #current_lines == 1 and current_lines[1] == "" then
		vim.api.nvim_buf_set_lines(output_buf, 0, -1, false, lines)
	else
		vim.api.nvim_buf_set_lines(output_buf, -1, -1, false, lines)
	end
	
	local line_count = vim.api.nvim_buf_line_count(output_buf)
	if output_win and vim.api.nvim_win_is_valid(output_win) then
		vim.api.nvim_win_set_cursor(output_win, {line_count, 0})
	end
end

M.eval_selection = function()
	if not M.job_id then
		append_to_output("⚠ Engine is not running")
		return
	end

	local start_line = vim.fn.getpos("'<")[2]
	local end_line = vim.fn.getpos("'>")[2]
	local lines = vim.fn.getline(start_line, end_line)
	local input = table.concat(lines, "\n")

	append_to_output("\n>>> " .. input)

	vim.fn.chansend(M.job_id, input .. "\n")
end

M.eval_line = function()
	if not M.job_id then
		append_to_output("⚠ Engine is not running")
		return
	end

	local line = vim.api.nvim_get_current_line()
	append_to_output("\n>>> " .. line)
	vim.fn.chansend(M.job_id, line .. "\n")
end

M.setup = function()
	print("[zim-sequencer] setup called")

	local path = engine_path()
	if vim.fn.executable(path) == 0 then
		vim.notify("[zim-sequencer] Engine not executable: " .. path, vim.log.levels.ERROR)
		return
	end

	print("[zim-sequencer] starting engine: " .. path)

	M.job_id = vim.fn.jobstart(path, {
		stdout_buffered = false,
		stderr_buffered = false,
		on_stdout = function(_, data, _)
			if data then
				for _, line in ipairs(data) do
					if line ~= "" then
						append_to_output(line)
					end
				end
			end
		end,
		on_stderr = function(_, data, _)
			if data then
				for _, line in ipairs(data) do
					if line ~= "" then
						vim.notify("[zim-sequencer] stderr: " .. line, vim.log.levels.ERROR)
					end
				end
			end
		end,
	})

	local buf = vim.api.nvim_get_current_buf()
	vim.keymap.set("v", "<leader>e", ":<C-U>lua require('sequencer').eval_selection()<CR>", {
		desc = "Evaluate Zim selection",
		noremap = true,
		silent = true,
		buffer = buf,
	})
	
	vim.keymap.set("v", "<CR>", ":<C-U>lua require('sequencer').eval_selection()<CR>", {
		desc = "Evaluate Zim selection",
		noremap = true,
		silent = true,
		buffer = buf,
	})
	
	vim.keymap.set("n", "<leader>e", "<cmd>lua require('sequencer').eval_line()<CR>", {
		desc = "Evaluate current line",
		noremap = true,
		silent = true,
		buffer = buf,
	})
	
	vim.keymap.set("n", "<CR>", "<cmd>lua require('sequencer').eval_line()<CR>", {
		desc = "Evaluate current line",
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
