if command -q -v zellij >/dev/null && status is-interactive
    # set ZELLIJ_AUTO_EXIT true
    # eval (zellij setup --generate-auto-start fish | string collect)
    alias zj=zellij
end
