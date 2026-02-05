local M = {
    ascii = require("extra.utils.ascii"),
    cmd = require("extra.utils.cmd"),
    icons = require("extra.utils.icons"),
    lsp = require("extra.utils.lsp"),
    ui = require("extra.utils.ui"),
}

---@param plugin string
---@return boolean
function M.has(plugin)
    if package.loaded["lazy"] then
        return require("lazy.core.config").plugins[plugin] ~= nil
    else
        local plugin_name = vim.split(plugin, ".", { plain = true, trimempty = true })
        return package.loaded[plugin_name[1]] ~= nil
    end
end

return M
