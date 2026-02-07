if status is-interactive
    set -U fish_greeting
    set -gx EDITOR nvim
    set -gx VISUAL nvim

    fish_add_path ~/.local/bin
end
