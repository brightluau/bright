--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

local fs = require("@lune/fs")
local stdio = require("@lune/stdio")

local clappy = require("@vendor/clappy")

local constants = require("../constants")
local fmt = require("../fmt")

local CONFIG_DEFAULT_CONTENTS = [[
source = "src/"
output = "output/"

# Transformers are applied in the order they are listed here.
transformers = []

# Transformers can accept a table of rules when they're executed, which can modify their behavior.
#
# N.B.: When running transformers from a file path, their rules will not apply unless this file includes a rule with that
# path.
[rules.example]
some_option = true
]]

local function run(_: any, opts: { force: boolean? })
	if (fs.isFile(constants.CONFIGURATION_FILE) or fs.isDir(constants.BRIGHT_FOLDER)) and not opts.force then
		local promptResult =
			stdio.prompt("confirm", "There appears to already be a Bright setup here, recreate?", false)

		if not promptResult then
			return
		end

		pcall(function()
			fs.removeFile(constants.CONFIGURATION_FILE)
		end)
		pcall(function()
			fs.removeDir(constants.BRIGHT_FOLDER)
		end)
	end

	fs.writeFile(constants.CONFIGURATION_FILE, CONFIG_DEFAULT_CONTENTS)

	fmt.success("Created a new Bright setup here.")
end

return clappy.command({
	name = "init",
	description = "Initializes the current folder with a Bright setup",

	parameters = {},
	opts = {
		force = clappy.opt({
			description = "Forcefully overwrite any Bright configuration non-interactively",
			type = clappy.types.boolean(),
		}),
	},
}, run)
