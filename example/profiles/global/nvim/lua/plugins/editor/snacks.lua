---@diagnostic disable: missing-fields
---@type LazySpec
return {
    "folke/snacks.nvim",
    -- version = "*",
    priority = 1000,
    lazy = false,
    ---@module "snacks"
    ---@type snacks.Config
    opts = {
        bigfile = { enabled = true },
        dashboard = { enabled = true, preset = { header = utils.ascii.sub_zero } },
        explorer = { enabled = true, replace_netrw = true, trash = true },
        ---@type snacks.picker.Config
        picker = {
            enabled = true,
            ui_select = true,
            sources = {
                explorer = {
                    layout = {
                        layout = { position = "right" },
                    },
                    hidden = true,
                    ignored = false,
                },
                files = { hidden = true },
                grep = { hidden = true },
                grep_word = { hidden = true },
                colorschemes = {
                    transform = function(item)
                        -- I don't like the default themes as they don't play nicely with treesitter
                        -- and nvim plugins
                        local defaults = {
                            ["blue"] = true,
                            ["darkblue"] = true,
                            ["default"] = false,
                            ["delek"] = true,
                            ["desert"] = true,
                            ["elflord"] = true,
                            ["evening"] = true,
                            ["habamax"] = true,
                            ["industry"] = true,
                            ["koehler"] = true,
                            ["lunaperche"] = true,
                            ["minicyan"] = true,
                            ["minischeme"] = true,
                            ["morning"] = true,
                            ["murphy"] = true,
                            ["pablo"] = true,
                            ["peachpuff"] = true,
                            ["pelflord"] = true,
                            ["quiet"] = true,
                            ["retrobox"] = true,
                            ["ron"] = true,
                            ["shine"] = true,
                            ["slate"] = true,
                            ["sorbet"] = true,
                            ["torte"] = true,
                            ["unokai"] = true,
                            ["vim"] = true,
                            ["wildcharm"] = true,
                            ["zaibatsu"] = true,
                            ["zellner"] = true,
                        }
                        if defaults[item.text] then
                            return false
                        end
                    end,
                },
            },
        },
        indent = {
            enabled = true,
            animate = { enabled = false },
            filter = function(buf)
                local filetypes = {
                    "markdown",
                }

                for _, ft in ipairs(filetypes) do
                    if vim.bo[buf].filetype == ft then
                        return
                    end
                end

                return vim.g.snacks_indent ~= false and vim.b[buf].snacks_indent ~= false and vim.bo[buf].buftype == ""
            end,
        },
        input = { enabled = true },
        image = { enabled = false, math = { enabled = false } },
        notifier = {
            enabled = true,
            style = "compact", ---@type snacks.notifier.style
        },
        quickfile = { enabled = true },
        scope = { enabled = true },
        scroll = { enabled = false },
        statuscolumn = { enabled = true },
        words = { enabled = true },
        -- styles = {},
    },
	-- stylua: ignore
	keys = {
		-- Find
		{ "<leader>fF", function() Snacks.picker.files({ hidden = true }) end,                                      desc = "Fuzzy find files in cwd (inlcuding hidden)" },
		{ "<leader>fb", function() Snacks.picker.buffers() end,                                                     desc = "Find buffers" },
		{ "<leader>fc", function() Snacks.picker.files({ cwd = vim.fn.stdpath("config") }) end,                     desc = "Find Config File" },
		{ "<leader>ff", function() Snacks.picker.files() end,                                                       desc = "Fuzzy find files in cwd" },
		{ "<leader>fg", function() Snacks.picker.git_files() end,                                                   desc = "Find Git Files" },
		{ "<leader>fp", function() Snacks.picker.projects() end,                                                    desc = "Projects" },
		{ "<leader>fr", function() Snacks.picker.recent() end,                                                      desc = "Fuzzy find recent files" },
        { "<leader>fs",  function() Snacks.scratch.select() end,                                                    desc = "Scratch Buffer" },
		-- Search
		{ "<leader>sB", function() Snacks.picker.grep_buffers() end,                                                desc = "Grep Open Buffers" },
		{ "<leader>sb", function() Snacks.picker.lines() end,                                                       desc = "Buffer Lines" },
		{ "<leader>sb", function() Snacks.picker.lines() end,                                                       desc = "Buffer Lines" },
		{ "<leader>sc", function() Snacks.picker.colorschemes() end,                                                desc = "Colorschemes" },
		{ "<leader>sg", function() Snacks.picker.grep() end,                                                        desc = "Grep" },
		{ "<leader>sh", function() Snacks.picker.help() end,                                                        desc = "Help Pages" },
		{ "<leader>si", function() Snacks.picker.icons() end,                                                       desc = "Icons" },
		{ "<leader>sk", function() Snacks.picker.keymaps() end,                                                     desc = "Keymaps" },
		{ "<leader>sl", function() Snacks.picker.lsp_config() end,                                                  desc = "LSP Configs" },
		{ "<leader>sw", function() Snacks.picker.grep_word() end,                                                   desc = "Visual selection or word",                                   mode = { "n", "x" } },
		-- Explorer
		{ "<leader>e",  function() Snacks.explorer.open() end,                                                      desc = "explore" },
		{ "<leader>\\", function() Snacks.explorer.reveal() end,                                                    desc = "reveal file location in Filetree" },
		-- Notifier
		{ "<leader>n",  "",                                                                                         desc = "notifications" },
		{ "<leader>nh", function() Snacks.notifier.show_history() end,                                              desc = "History" },
		{ "<leader>ne", function() Snacks.notifier.show_history({ filter = vim.log.levels.ERROR }) end,             desc = "Errors" },
		{ "<leader>ni", function() Snacks.notifier.show_history({ filter = vim.log.levels.INFO }) end,              desc = "Info" },
		{ "<leader>nw", function() Snacks.notifier.show_history({ filter = vim.log.levels.WARN }) end,              desc = "Warn" },
		{ "<c-esc>",    function() Snacks.notifier.hide() end,                                                      desc = "Dismiss" },
		-- Terminal
		{ "<c-/>",      function() Snacks.terminal.toggle(nil, { interactive = true, win = { height = 5, } }) end,  desc = "Toggle Terminal",                                            mode = { "n", "t" } },
		{ "<c-_>",      function() Snacks.terminal.toggle(nil, { interactive = true, win = { height = 5, } }) end,  desc = "Toggle Terminal",                                            mode = { "n", "t" } },
		{ "<leader>rb", function() Snacks.terminal("btop", { win = { position = "float" } }) end,                   desc = "Btop" },
		{ "<leader>rh", function() Snacks.terminal("htop", { win = { position = "float" } }) end,                   desc = "Htop" },
		{ "<leader>rg", function() Snacks.lazygit() end,                                                            desc = "Lazygit" },
		-- Other
		{ "[[",         function() Snacks.words.jump(-vim.v.count1) end,                                            desc = "Prev Reference",                                             mode = { "n", "t" } },
		{ "]]",         function() Snacks.words.jump(vim.v.count1) end,                                             desc = "Next Reference",                                             mode = { "n", "t" } },
        { "<leader>bd", function() Snacks.bufdelete() end,                                                          desc = "Delete Buffer" },
		{ "<leader>go", function() Snacks.gitbrowse() end,                                                          desc = "Open the current file, branch, commit, or repo in a browser" },
        { "<leader>.",  function() Snacks.scratch({ win = { height = 0.90 } }) end,                                                            desc = "Toggle Scratch Buffer" },
		{
			"<leader>N",
			desc = "Neovim News",
			function()
				Snacks.win({
					file = vim.api.nvim_get_runtime_file("doc/news.txt", false)[1],
					width = 0.9,
					height = 0.9,
					border = "none",
					wo = {
						spell = false,
						wrap = false,
						signcolumn = "yes",
						statuscolumn = " ",
						conceallevel = 3,
					},
				})
			end,
		},
	},
    init = function()
        vim.api.nvim_create_autocmd("User", {
            pattern = "VeryLazy",
            callback = function()
				-- Setup some globals for debugging (lazy-loaded)
				-- stylua: ignore
				_G.dd = function(...) Snacks.debug.inspect(...) end
				-- stylua: ignore
				_G.bt = function() Snacks.debug.backtrace() end
                vim.print = _G.dd -- Override print to use snacks for `:=` command
                -- Create some toggle mappings
                Snacks.toggle.dim():map("<leader>ud")
                Snacks.toggle.indent():map("<leader>ug")
				-- stylua: ignore
				Snacks.toggle.option("conceallevel", { off = 0, on = vim.o.conceallevel > 0 and vim.o.conceallevel or 2 })
					:map("<leader>uc")
                Snacks.toggle.option("list", { name = "List" }):map("<leader>ul")
                Snacks.toggle.option("relativenumber", { name = "Relative Number" }):map("<leader>ur")
                Snacks.toggle.option("wrap", { name = "Wrap" }):map("<leader>uw")
                Snacks.toggle.scroll():map("<leader>uS")
                Snacks.toggle.treesitter():map("<leader>ut")
                Snacks.toggle.profiler():map("<leader>pp")
                Snacks.toggle.profiler_highlights():map("<leader>ph")
            end,
        })
    end,
}
