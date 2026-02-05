return {
    "nvim-treesitter/nvim-treesitter",
    branch = "main",
    build = ":TSUpdate",
    lazy = false,
    ---@type TSConfig|fun(): TSConfig?
    config = function()
        local ts = require("nvim-treesitter")
        ts.install({
            -- languages
            "c",
            "dart",
            "javascript",
            "lua",
            "luadoc",
            "rust",
            "typescript",
            "vim",
            "vimdoc",
            -- shell
            "bash",
            "fish",
            -- markup
            "css",
            "html",
            "latex",
            "markdown",
            "markdown_inline",
            "scss",
            "svelte",
            "tsx",
            "typst",
            "vue",
            --
            "json",
            "toml",
            "yaml",
            -- misc
            "comment",
            "diff",
            "query",
            "regex",
            -- shader
            "wgsl",
            -- godot
            "gdscript",
            "gdshader",
        })
    end,
    init = function()
        require("vim.treesitter.query").add_predicate("is-mise?", function(_, _, bufnr, _)
            ---@diagnostic disable-next-line: param-type-mismatch
            local filepath = vim.api.nvim_buf_get_name(tonumber(bufnr) or 0)
            local filename = vim.fn.fnamemodify(filepath, ":t")
            return string.match(filename, ".*mise.*%.toml$") ~= nil
        end, { force = true, all = false })
    end,
}
