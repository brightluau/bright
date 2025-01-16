local fs = require("@lune/fs")

local actions = require("../actions")

local EXCLUDE = {
	"./src/vendor"
}

local LICENSE_HEADER = fs.readFile("./include/license-header.txt")

local function joinPath(...): string
	return table.concat({ ... }, "/")
end

local function discover(root: string, extension: string?): { string }
	local discoveredFiles = {}

	local files = fs.readDir(root)

	for _, file in files do
		local fullPath = joinPath(root, file)

		if table.find(EXCLUDE, fullPath) then
			continue
		end

		if fs.isDir(fullPath) then
			local subPaths = discover(fullPath, extension)

			for _, v in subPaths do
				table.insert(discoveredFiles, v)
			end
		else
			if extension and file:sub(-#extension) ~= extension then
				continue
			end

			table.insert(discoveredFiles, fullPath)
		end
	end

	return discoveredFiles
end

local paths = discover("./src", "luau")

for _, file in paths do
	local src = fs.readFile(file)

	if src:sub(1, #LICENSE_HEADER) ~= LICENSE_HEADER then
		actions.error("License header missing or malformed", {
			file = file
		})
	end
end

return nil
