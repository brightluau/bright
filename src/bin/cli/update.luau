--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

local fs = require("@lune/fs")
local serde = require("@lune/serde")

local argparse = require("@cli/argparse")
local constants = require("@cli/constants")
local fetchLibrariesFromGit = require("@cli/util/fetchLibrariesFromGit")
local fmt = require("@cli/fmt")

local function run()
	if fs.isFile("pesde.toml") then
		local pesdeToml = fs.readFile("pesde.toml")
		local pesde = serde.decode("toml", pesdeToml)

		if pesde.dev_dependencies and pesde.dev_dependencies.bright then
			fmt.bail(
				"Bright is managed through pesde!",
				fmt.hint("To update it, you need to run `pesde add -d bright/bright`.")
			)
		end
	end

	if not fs.isDir(`{constants.BRIGHT_FOLDER}/lib`) then
		fmt.bail("Bright is not installed!", fmt.hint("Run `bright init` to set up your project."))
	end

	fs.removeDir(`{constants.BRIGHT_FOLDER}/lib`)
	fs.writeDir(`{constants.BRIGHT_FOLDER}/lib`)

	local progress = fmt.progress("Updating Bright libraries", 1)

	fetchLibrariesFromGit(progress)

	progress:finish(fmt.symbols.success)
end

return function(parser: argparse.Parser)
	parser:command("update up"):description("Updates the current Bright installation"):action(run)
end
