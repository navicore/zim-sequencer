local Job = require("plenary.job")

return function()
	local cargo = vim.fn.executable("cargo") == 1 and "cargo" or nil
	if not cargo then
		vim.notify("[zim-sequencer] Rust not found", vim.log.levels.ERROR)
		return
	end

	local plugin_root = debug.getinfo(1, "S").source:sub(2):match("(.*/)")
	local target = plugin_root .. "engine/target/release/zim-sequencer"
	local output = vim.fn.stdpath("data") .. "/zim-sequencer-bin"
	vim.fn.mkdir(output, "p")

	Job:new({
		command = "cargo",
		args = { "build", "--release" },
		cwd = plugin_root .. "engine",
		on_exit = function()
			local dst = output .. "/zim-sequencer"
			vim.fn.system({ "cp", target, dst })
		end,
	}):start()
end
