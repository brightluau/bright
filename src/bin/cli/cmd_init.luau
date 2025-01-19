--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

local fs = require("@lune/fs")
local process = require("@lune/process")
local serde = require("@lune/serde")
local stdio = require("@lune/stdio")

local argparse = require("@cli/argparse")
local constants = require("@cli/constants")
local fetchLibrariesFromGit = require("@cli/util/fetchLibrariesFromGit")
local fmt = require("@cli/fmt")

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

local function run(opts: { force: boolean })
	if (fs.isFile(constants.CONFIGURATION_FILE) or fs.isDir(constants.BRIGHT_FOLDER)) and not opts.force then
		fmt.bail(
			"There is already a Bright setup here, refusing to overwrite!",
			fmt.hint("If this was intentional, pass --force.")
		)
	end

	pcall(function()
		fs.removeFile(constants.CONFIGURATION_FILE)
	end)
	pcall(function()
		fs.removeDir(constants.BRIGHT_FOLDER)
	end)

	local usePesde = false

	if fs.isFile("pesde.toml") then
		usePesde = stdio.prompt(
			"confirm",
			"Pesde seems to be in use for this project, do you want it to manage Bright's libraries?",
			true
		)
	end

	local updateLuaurc = stdio.prompt("confirm", "Would you like Bright to create/update your .luaurc file?", true)

	-- create the bright folder
	fs.writeDir(constants.BRIGHT_FOLDER)

	print()

	local progress = fmt.progress("Installing Bright libraries", if updateLuaurc then 3 else 2)

	local libLocation: string?

	if usePesde then
		progress:writeOutput("Adding to pesde.toml")
		local addResult = process.spawn("pesde", { "add", "-d", "brightluau/bright" })
		if not addResult.ok then
			progress:finish(fmt.symbols.error)
			fmt.bail("\nFailed to install Bright libraries through pesde:", addResult.stderr)
		end

		progress:writeOutput("Installing")
		local installResult = process.spawn("pesde", { "install" })
		if not installResult.ok then
			progress:finish(fmt.symbols.error)
			fmt.bail("\nFailed to install Bright libraries through pesde:", installResult.stderr)
		end
	else
		local ok, err = fetchLibrariesFromGit(progress)

		if not ok then
			progress:finish(fmt.symbols.error)
			fmt.bail("\nFailed to download libraries from GitHub:", err)
		end

		libLocation = `{constants.BRIGHT_FOLDER}/lib`
	end

	progress:step(fmt.symbols.success, 2, "Writing configuration files")

	-- write bright config
	fs.writeFile(constants.CONFIGURATION_FILE, CONFIG_DEFAULT_CONTENTS)

	if updateLuaurc then
		progress:step(fmt.symbols.success, 3, "Updating .luaurc")
	else
		progress:finish(fmt.symbols.success)

		if libLocation then
			print("\nTo set up the `@bright/` alias, add the following to your .luaurc:")
			print(serde.encode("json", {
				aliases = {
					bright = libLocation,
				},
			}, true))
		end

		return
	end

	-- create or alter .luaurc
	if libLocation then
		if fs.isFile(".luaurc") then
			local existingLuaurc = fs.readFile(".luaurc")

			local luaurc = serde.decode("json", existingLuaurc)

			if luaurc.aliases == nil then
				luaurc.aliases = {} :: any -- type error :D
			end

			luaurc.aliases.bright = libLocation

			fs.writeFile(".luaurc", serde.encode("json", luaurc, true))
		else
			fs.writeFile(".luaurc", serde.encode("json", { aliases = { bright = libLocation } }, true))
		end

		progress:finish(fmt.symbols.success)
	else
		progress:finish(fmt.symbols.warning)

		stdio.ewrite(
			"\nBright was unable to set up the `@bright/` alias in your .luaurc. You will need to do this manually."
		)
	end
end

return function(parser: argparse.Parser)
	local command = parser:command("init"):description("Initializes the current folder with a Bright setup"):action(run)

	command:flag("-f --force"):description("Forcefully overwrite any Bright configuration non-interactively")
end
