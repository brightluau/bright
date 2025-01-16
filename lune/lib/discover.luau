local fs = require("@lune/fs")

local joinPath = require("@lune-lib/joinPath")

local function discover(root: string, extension: string?, exclusions: { string }?): { string }
	local discoveredFiles = {}

	local files = fs.readDir(root)

	for _, file in files do
		local fullPath = joinPath(root, file)

		if exclusions and table.find(exclusions, fullPath) then
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

return discover
