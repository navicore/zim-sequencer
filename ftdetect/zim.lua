vim.api.nvim_create_autocmd({ "BufRead", "BufNewFile" }, {
	pattern = { "*.zim", "*.mel" },
	callback = function()
		vim.bo.filetype = "zim"
	end,
})
