local fs = require("@lune/fs")

local actions = require("@lune-lib/actions")
local discover = require("@lune-lib/discover")

local EXCLUDE = {
	"./src/vendor",
}

local LICENSE_HEADER = fs.readFile("./include/license-header.txt")

local paths = discover("./src", "luau", EXCLUDE)

for _, file in paths do
	local src = fs.readFile(file)

	if src:sub(1, #LICENSE_HEADER) ~= LICENSE_HEADER then
		actions.error("License header missing or malformed", {
			file = file,
		})
	end
end

return nil
