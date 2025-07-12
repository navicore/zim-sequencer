local M = {}

local function engine_path()
	local info = debug.getinfo(1, "S")
	local plugin_root = info.source:sub(2):match("(.*/)")
	return plugin_root .. "engine/target/release/zim-sequencer"
end

M.setup = function()
	local path = engine_path()
	if vim.fn.executable(path) == 0 then
		vim.notify("[zim-sequencer] engine not found or not executable: " .. path, vim.log.levels.ERROR)
		return
	end

	M.job_id = vim.fn.jobstart({ path }, {
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
end

M.eval_selection = function()
	-- same as before
end

vim.keymap.set("v", "<leader>e", M.eval_selection)

return M
