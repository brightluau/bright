local frktest = require("@frktest/frktest")
local test = frktest.test
local check = frktest.assert.check
local req = frktest.assert.require

local fs = require("@lune/fs")

local discover = require("@lune-lib/discover")

local EXCLUDE = {
	"./src/vendor",
}

local LICENSE_HEADER = fs.readFile("./include/license-header.txt")

local paths = discover("./src", "luau", EXCLUDE)

return function()
	test.suite("License checks", function()
		for _, file in paths do
			test.case(file, function()
				local src = fs.readFile(file)

				req.is_true(src:sub(1, #LICENSE_HEADER) == LICENSE_HEADER)

				return nil
			end)
		end

		return nil
	end)
end
