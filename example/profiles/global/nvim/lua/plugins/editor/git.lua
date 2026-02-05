---@type LazySpec
-- git integration
return {
    "lewis6991/gitsigns.nvim",
    version = "*",
    opts = {
        current_line_blame = false,
        on_attach = function(bufnr)
            local gitsigns = require("gitsigns")
            local opts = {}

            opts.desc = "git"
            vim.keymap.set({ "n", "v" }, "<leader>g", "", opts)

            opts.desc = "hunk"
            vim.keymap.set({ "n", "v" }, "<leader>gh", "", opts)

            opts.desc = "Next Hunk"
            vim.keymap.set("n", "]c", function()
                if vim.wo.diff then
                    vim.cmd.normal({ "]c", bang = true })
                else
                    gitsigns.nav_hunk("next")
                end
            end, opts)

            opts.desc = "Previous Hunk"
            vim.keymap.set("n", "[c", function()
                if vim.wo.diff then
                    vim.cmd.normal({ "[c", bang = true })
                else
                    gitsigns.nav_hunk("prev")
                end
            end, opts)

            opts.desc = "Stage Hunk"
            vim.keymap.set("n", "<leader>ghs", gitsigns.stage_hunk, opts)

            opts.desc = "Reset Hunk"
            vim.keymap.set("n", "<leader>ghr", gitsigns.reset_hunk, opts)

            opts.desc = "Stage Hunk"
            vim.keymap.set("v", "<leader>ghs", function()
                gitsigns.stage_hunk({ vim.fn.line("."), vim.fn.line("v") })
            end, opts)

            opts.desc = "Reset Hunk"
            vim.keymap.set("v", "<leader>ghr", function()
                gitsigns.reset_hunk({ vim.fn.line("."), vim.fn.line("v") })
            end, opts)

            opts.desc = "Stage Buffer"
            vim.keymap.set("n", "<leader>ghS", gitsigns.stage_buffer, opts)

            opts.desc = "Reset Buffer"
            vim.keymap.set("n", "<leader>ghR", gitsigns.reset_buffer, opts)

            opts.desc = "Preview Hunk"
            vim.keymap.set("n", "<leader>ghp", gitsigns.preview_hunk, opts)

            opts.desc = "Preview Hunk Inline"
            vim.keymap.set("n", "<leader>ghi", gitsigns.preview_hunk_inline, opts)

            opts.desc = "Blame Line"
            vim.keymap.set("n", "<leader>gb", function()
                gitsigns.blame_line({ full = true })
            end, opts)

            opts.desc = "Blame"
            vim.keymap.set("n", "<leader>gB", gitsigns.blame, opts)

            opts.desc = "Diff This"
            vim.keymap.set("n", "<leader>gd", gitsigns.diffthis, opts)

            opts.desc = "Diff This ~"
            vim.keymap.set("n", "<leader>gD", function()
                gitsigns.diffthis("~")
            end, opts)

            opts.desc = "Set Quickfix List"
            vim.keymap.set("n", "<leader>gq", gitsigns.setqflist, opts)

            opts.desc = "Set Quickfix List All"
            vim.keymap.set("n", "<leader>gQ", function()
                gitsigns.setqflist("all")
            end, opts)
        end,
    },
}
