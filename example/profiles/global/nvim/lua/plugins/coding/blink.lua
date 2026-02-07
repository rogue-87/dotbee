---@diagnostic disable: missing-fields
---@type LazySpec
return {
    "Saghen/blink.cmp",
    dependencies = { "rafamadriz/friendly-snippets" },
    version = "*",
    ---@type blink.cmp.Config
    opts = {
        keymap = { preset = "enter" },
        cmdline = { enabled = true, completion = { ghost_text = { enabled = true } } },
        appearance = { nerd_font_variant = "normal" },
        completion = {
            documentation = { window = { border = "padded", scrollbar = true } },
            list = { selection = { preselect = true, auto_insert = false } },
            ghost_text = { enabled = true },
            trigger = { prefetch_on_insert = false },
        },
        signature = { enabled = false }, -- handled by noice
        snippets = { preset = "default" },
        sources = { default = { "lsp", "path", "snippets", "buffer" } },
        fuzzy = { implementation = "lua" },
    },
}
