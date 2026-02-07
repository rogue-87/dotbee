function man
	if not command -v batman >/dev/null 2>&1
		command man "$argv"
		return
	end
  	command batman "$argv"
end
