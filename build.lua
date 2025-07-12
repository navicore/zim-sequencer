-- build.lua
local uv = vim.loop
local Job = require("plenary.job")

return function()
	local cargo = vim.fn.executable("cargo") == 1 and "cargo" or nil
	if not cargo then
		vim.notify("Rust is required to build zim-sequencer", vim.log.levels.ERROR)
		return
	end

	local build_dir = vim.fn.stdpath("data") .. "/zim-sequencer-bin"
	vim.fn.mkdir(build_dir, "p")

	Job:new({
		command = "cargo",
		args = { "build", "--release" },
		cwd = vim.fn.stdpath("config") .. "/lazy/zim-sequencer/engine", -- adjust if using pack
		on_exit = function()
			local binary = "zim-sequencer"
			local src = vim.fn.stdpath("config") .. "/lazy/zim-sequencer/engine/target/release/" .. binary
			local dst = build_dir .. "/" .. binary
			vim.fn.copy(src, dst)
		end,
	}):start()
end
