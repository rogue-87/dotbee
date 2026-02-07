-- utilities & helper functions
_G.utils = require("extra.utils")
-- global settings
_G.settings = { colorscheme = "duskfox" }

-- Prepend mise shims to PATH
if vim.fn.executable("mise") == 1 then
    vim.env.PATH = vim.env.HOME .. "/.local/share/mise/shims:" .. vim.env.PATH
end

require("config") -- load vim options, lazy.nvim, keymaps, autocmds

-- choose colorscheme
vim.cmd.colorscheme(settings.colorscheme)

-- language server configs to enable
vim.lsp.enable({
    -- shell langs
    "bash-ls",
    -- programming langs
    "dart-ls",
    "emmylua-ls",
    "jdt-ls",
    "rust-analyzer",
    "ty",
    -- other
    "harper-ls",
    "json-ls",
    "taplo",
    "tinymist",
    "zk",
})
