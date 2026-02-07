---@type LazySpec
return {
    -- fancy icons
    {
        "nvim-mini/mini.icons",
        version = "*",
        config = function()
            local mini_icons = require("mini.icons")

            mini_icons.setup({
                default = {},
                extension = {
                    lua = { hl = "MiniIconsBlue" },
                },
                file = {
                    ["init.lua"] = { glyph = "", hl = "MiniIconsBlue" },
                    ["mise.lock"] = { glyph = "", hl = "MiniIconsYellow" },
                    ["LICENSE"] = { glyph = "", hl = "MiniIconsYellow" },
                    ["LICENCE"] = { glyph = "", hl = "MiniIconsYellow" },
                    ["LICENSE-MIT"] = { glyph = "", hl = "MiniIconsYellow" },
                    ["LICENSE-APACHE"] = { glyph = "", hl = "MiniIconsYellow" },
                },
            })
            mini_icons.mock_nvim_web_devicons()
        end,
    },
    -- color highlight
    {
        -- For color highlighting
        "catgoose/nvim-colorizer.lua",
        opts = {
            lazy_load = true,
            filetypes = {
                "*",
                "!popup",
                "!lazy",
                "!noice",
                "!snacks_terminal",
            },
            user_default_options = {
                mode = "background", ---@type "foreground"|"background"|"virtualtext"
                tailwind = true, ---@type true|false|"normal"|"lsp"|"both"
                virtualtext = "󰝤",
                always_update = true,
            },
        },
    },
    -- breadcrumbs
    { "Bekaboo/dropbar.nvim", version = "*", opts = {} },
}
