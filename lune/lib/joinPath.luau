local function joinPath(...): string
	return table.concat({ ... }, "/")
end

return joinPath
