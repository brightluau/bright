--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

local clappy = require("@vendor/clappy")
local colorful = require("@vendor/colorful")

local cli = require("./cli")
local fmt = require("./fmt")

print(require("./config"))

local bright = clappy
	.cli("bright")
	:palette({
		primary = colorful.color.yellow,
		secondary = colorful.combineStyles({}),
		tertiary = colorful.combineStyles({}),
	})
	:metadata({
		description = "A scriptable tool for transforming and transpiling Luau code",
		version = "0.1.0",
		author = "lewisakura",

		afterHelp = "",
	})

for i, subcommand in cli do
	if i == 1 then
		bright:defineRootCommand(subcommand)
	else
		bright:defineSubcommand(subcommand)
	end
end

local ok, err = pcall(function()
	bright:run()
	return nil
end)

if not ok then
	fmt.error("Internal error occurred:", err)
end
