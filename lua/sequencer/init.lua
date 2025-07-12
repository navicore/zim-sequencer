local M = {}

local engine_path = vim.fn.stdpath("data") .. "/lazy/zim-sequencer/engine/target/release/zim-sequencer"

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
end

-- your eval_selection() code here...

return M
