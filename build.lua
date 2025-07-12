local Job = require("plenary.job")

return function()
	local cargo = vim.fn.executable("cargo") == 1 and "cargo" or nil
	if not cargo then
		vim.notify("[zim-sequencer] Rust toolchain not found", vim.log.levels.ERROR)
		return
	end

	local info = debug.getinfo(1, "S")
	local plugin_root = info.source:sub(2):match("(.*/)")
	local engine_dir = plugin_root .. "engine"
	local target_bin = engine_dir .. "/target/release/zim-sequencer"
	local output_dir = vim.fn.stdpath("data") .. "/zim-sequencer-bin"
	local output_path = output_dir .. "/zim-sequencer"

	vim.fn.mkdir(output_dir, "p")

	Job:new({
		command = "cargo",
		args = { "build", "--release" },
		cwd = engine_dir,
		on_exit = function(j, return_val)
			if return_val == 0 and vim.fn.filereadable(target_bin) == 1 then
				vim.fn.system({ "cp", target_bin, output_path })
				vim.fn.system({ "chmod", "+x", output_path })
				print("[zim-sequencer] Engine built and copied to: " .. output_path)
			else
				vim.notify("[zim-sequencer] Cargo build failed.", vim.log.levels.ERROR)
			end
		end,
	}):start()
end
