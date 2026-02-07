local Tabline = {}

--- simple tabline that only displays the name of the file
function Tabline.activate()
    local tabs = {}
    for i = 1, vim.fn.tabpagenr("$") do
        local is_selected = i == vim.fn.tabpagenr()
        local hilight = is_selected and "%#TabLineSel#" or "%#TabLine#"

        local buflist = vim.fn.tabpagebuflist(i)
        local winnr = vim.fn.tabpagewinnr(i)
        local bufnr = buflist[winnr]
        local file = vim.fn.bufname(bufnr)
        local fname = vim.fn.fnamemodify(file, ":t")
        fname = fname == "" and "[No Name]" or fname

        local modified = vim.bo[bufnr].modified and " [+]" or ""

        -- build this specific tab's string
        local tab_str = string.format("%s%%%dT %s%s %d ", hilight, i, fname, modified, i)
        table.insert(tabs, tab_str)
    end

    return table.concat(tabs) .. "%#TabLineFill#%T"
end

return Tabline
