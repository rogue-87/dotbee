---@type vim.keymap.set.Opts
local opts = { noremap = true, silent = true }

-- Clipboard related stuff
-- DELETE
vim.keymap.set({ "n", "v" }, "d", '"_d', opts)
-- DELETE LINE
vim.keymap.set("n", "dd", '"_dd', opts)
-- DELETE TILL END OF LINE
vim.keymap.set("n", "D", '"_D', opts)

-- CHANGE
vim.keymap.set({ "n", "v" }, "c", '"_c', opts)
-- CHANGE LINE
vim.keymap.set("n", "cc", '"_cc', opts)
-- CHANGE TILL END OF LINE
vim.keymap.set("n", "C", '"_C', opts)

-- CUT
vim.keymap.set({ "n", "v" }, "x", "d", opts)
-- CUT LINE
vim.keymap.set("n", "xx", "dd", opts)
-- CUT TILL END OF LINE
vim.keymap.set("n", "X", "D", opts)

-- SELECT
vim.keymap.set({ "n", "v" }, "s", '"_s', opts)
-- SELECT LINE
vim.keymap.set("n", "S", '"_S', opts)

-- Actual Keymaps
opts.desc = "write file"
vim.keymap.set("n", "<leader>ww", "<cmd>w<cr>", opts)

opts.desc = "write all files"
vim.keymap.set("n", "<leader>wa", "<cmd>wa<cr>", opts)

opts.desc = "quit/session"
vim.keymap.set("n", "<leader>q", "", opts)

opts.desc = "quit & save"
vim.keymap.set("n", "<leader>qq", "<cmd>wqa<cr>", opts)

opts.desc = "quit witout saving"
vim.keymap.set("n", "<leader>qQ", "<cmd>qa!<cr>", opts)

opts.desc = "restart editor"
vim.keymap.set("n", "<leader>qr", "<cmd>restart<cr>", opts)

-- vim.keymap.set("n", "<A-n>", "<cmd>tabnew<cr>", opts)
-- vim.keymap.set("n", "<A-c>", "<cmd>tabclose<cr>", opts)
-- vim.keymap.set("n", "<A-.>", "<cmd>tabn<cr>", opts)
-- vim.keymap.set("n", "<A-,>", "<cmd>tabp<cr>", opts)
-- vim.keymap.set("n", "<C-,>", "<cmd>-tabmove<cr>", opts)
-- vim.keymap.set("n", "<C-.>", "<cmd>+tabmove<cr>", opts)

-- NOTE: LSP related mappings
utils.lsp.on_attach(function(client, bufnr)
    local ls_opts = { buffer = bufnr }

    if utils.has("which-key.nvim") then
        local whichkey = require("which-key")

        local spec = {
            { "<localleader>", desc = "lsp", buffer = bufnr },
            { "<localleader>c", desc = "code", icon = { icon = "", color = "orange" }, buffer = bufnr },
            { "<localleader>b", desc = "buffer", buffer = bufnr },
            { "<localleader>d", desc = "document", icon = { icon = "󰈙", color = "green" }, buffer = bufnr },
            { "<localleader>g", desc = "goto", icon = { icon = "", color = "orange" }, buffer = bufnr },
            { "<localleader>l", desc = "line", icon = "", buffer = bufnr },
            { "<localleader>r", desc = "rename", icon = { icon = "", color = "orange" }, buffer = bufnr },
            { "<localleader>u", desc = "toggles", buffer = bufnr },
            { "<localleader>w", desc = "workspace", icon = "󱇙", buffer = bufnr },
        }
        ---@diagnostic disable-next-line: param-type-mismatch
        whichkey.add(spec)
    end

    if client == nil then
        vim.notify("Couldn't setup language server related keymaps", vim.log.levels.ERROR)
        return
    end

    if client:supports_method("textDocument/hover") then
        ls_opts.desc = "hover info"
        vim.keymap.set("n", "K", vim.lsp.buf.hover, ls_opts)
    end

    if client:supports_method("textDocument/signatureHelp") then
        ls_opts.desc = "signature help"
        vim.keymap.set({ "n", "i" }, "<C-S-K>", vim.lsp.buf.signature_help, ls_opts)
    end

    if client:supports_method("textDocument/declaration") then
        ls_opts.desc = "goto declaration"
        vim.keymap.set("n", "<localleader>gD", vim.lsp.buf.declaration, ls_opts)
    end

    if client:supports_method("textDocument/definition") then
        ls_opts.desc = "goto definition"
        vim.keymap.set("n", "<localleader>gd", vim.lsp.buf.definition, ls_opts)
    end

    if client:supports_method("textDocument/typeDefinition") then
        ls_opts.desc = "goto type definition"
        vim.keymap.set("n", "<localleader>gt", vim.lsp.buf.type_definition, ls_opts)
    end

    if client:supports_method("textDocument/implementation") then
        ls_opts.desc = "goto type implementation"
        vim.keymap.set("n", "<localleader>gi", Snacks.picker.lsp_implementations, ls_opts)
    end

    if client:supports_method("textDocument/references") then
        ls_opts.desc = "goto type references"
        vim.keymap.set("n", "<localleader>gr", Snacks.picker.lsp_references, ls_opts)
    end

    if client:supports_method("textDocument/documentSymbol") then
        ls_opts.desc = "symbols"
        vim.keymap.set("n", "<localleader>ds", Snacks.picker.lsp_symbols, ls_opts)
    end

    if client:supports_method("textDocument/codeAction") then
        ls_opts.desc = "action"
        vim.keymap.set({ "n", "v" }, "<localleader>ca", vim.lsp.buf.code_action, ls_opts)
    end

    if client:supports_method("textDocument/formatting") and not utils.has("guard.nvim") then
        ls_opts.desc = "format"
        vim.keymap.set("n", "<localleader>df", "<cmd>lua vim.lsp.buf.format({ async = true })<cr>", ls_opts)
    end

    if client:supports_method("textDocument/rangeFormatting") then
        ls_opts.desc = "range format"
        vim.keymap.set("v", "<localleader>df", function()
            vim.lsp.buf.format({
                async = true,
                range = {
                    ["start"] = vim.api.nvim_buf_get_mark(0, "<"),
                    ["end"] = vim.api.nvim_buf_get_mark(0, ">"),
                },
            })
        end, ls_opts)
    end

    if client:supports_method("textDocument/rename") then
        ls_opts.desc = "rename symbol"
        vim.keymap.set("n", "<localleader>rn", vim.lsp.buf.rename, ls_opts)

        ls_opts.desc = "rename file"
        vim.keymap.set("n", "<localleader>rf", Snacks.rename.rename_file, ls_opts)
    end

    if client:supports_method("textDocument/prepareCallHierarchy") then
        ls_opts.desc = "incoming calls"
        vim.keymap.set("n", "<localleader>ci", Snacks.picker.lsp_incoming_calls, ls_opts)

        ls_opts.desc = "outgoing calls"
        vim.keymap.set("n", "<localleader>co", Snacks.picker.lsp_outgoing_calls, ls_opts)
    end

    if client:supports_method("workspace/symbol") then
        ls_opts.desc = "list workspace symbols"
        vim.keymap.set("n", "<localleader>ws", Snacks.picker.lsp_workspace_symbols, ls_opts)
    end

    ls_opts.desc = "goto previous diagnostics"
    vim.keymap.set("n", "[d", function()
        vim.diagnostic.jump({ count = -1, float = true })
    end, ls_opts)

    ls_opts.desc = "goto next diagnostics"
    vim.keymap.set("n", "]d", function()
        vim.diagnostic.jump({ count = 1, float = true })
    end, ls_opts)

    ls_opts.desc = "line diagnostics"
    vim.keymap.set("n", "<localleader>ld", vim.diagnostic.open_float, ls_opts)

    ls_opts.desc = "buffer diagnostics"
    vim.keymap.set("n", "<localleader>bd", Snacks.picker.diagnostics_buffer, ls_opts)

    Snacks.toggle.diagnostics():map("<localleader>ud")

    if client:supports_method("workspace/diagnostic") then
        ls_opts.desc = "workspace diagnostics"
        vim.keymap.set("n", "<localleader>wd", Snacks.picker.diagnostics, ls_opts)
    end

    if client:supports_method("textDocument/inlayHint") then
        Snacks.toggle.inlay_hints():map("<localleader>uh")
    end

    ls_opts.desc = "list workspace folders"
    vim.keymap.set("n", "<localleader>wf", function()
        vim.notify(vim.inspect(vim.lsp.buf.list_workspace_folders()), vim.log.levels.INFO)
    end, ls_opts)

    Snacks.toggle.words():map("<localleader>uw")
end)
