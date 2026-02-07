-- global vim variables
vim.g.editorconfig = true
vim.g.loaded_netrw = 1
vim.g.loaded_netrwPlugin = 1
vim.g.loaded_node_provider = false
vim.g.loaded_perl_provider = false
vim.g.loaded_python_provider = false
vim.g.mapleader = " "
vim.g.maplocalleader = "\\"
vim.g.markdown_recommended_style = 0

-- global options
vim.go.tabline = "%!v:lua.utils.ui.tabline.activate()"

-- options
local shell = { main = "fish", fallback = "bash" }
vim.o.shell = vim.fn.exepath(shell.main) ~= "" and vim.fn.exepath(shell.main) or vim.fn.exepath(shell.fallback)

vim.o.autoindent = true
vim.o.autoread = true
vim.o.autowrite = false
vim.o.clipboard = "unnamedplus"
vim.o.completeopt = "menu,menuone,noselect"
vim.o.conceallevel = 0
vim.o.confirm = true
vim.o.cursorline = true
vim.o.cursorlineopt = "number"
vim.o.expandtab = true
vim.o.exrc = true
vim.o.incsearch = true
vim.o.number = true
vim.o.relativenumber = true
vim.o.scrolloff = 5
vim.o.sessionoptions = "curdir,folds,globals,help,tabpages,terminal,winsize"
vim.o.shiftwidth = 4
vim.o.showmode = false
vim.o.showtabline = 1
vim.o.laststatus = 3
vim.o.signcolumn = "yes"
vim.o.smartindent = false
vim.o.smoothscroll = false
vim.o.tabstop = 4
vim.o.termguicolors = true
vim.o.termsync = true
vim.o.wrap = false

-- nice and simple folding:
vim.o.fillchars = "foldopen:,foldclose:,fold: ,foldsep: ,diff:╱,eob: "
vim.o.foldcolumn = "0"
vim.o.foldenable = true
vim.o.foldlevel = 99
vim.o.foldmethod = "expr"
vim.o.foldtext = ""
vim.o.foldexpr = "v:lua.vim.treesitter.foldexpr()" -- default to treesitter folding

vim.o.list = false
vim.o.listchars = "tab: ->,space:."

local signs = utils.icons.diagnostics
-- diagnostic options
vim.diagnostic.config({
    severity_sort = true,
    underline = true,
    update_in_insert = false,
    virtual_lines = false,
    virtual_text = true,
    signs = {
        text = {
            [vim.diagnostic.severity.ERROR] = signs.error,
            [vim.diagnostic.severity.HINT] = signs.hint,
            [vim.diagnostic.severity.INFO] = signs.info,
            [vim.diagnostic.severity.WARN] = signs.warn,
        },
    },
})
