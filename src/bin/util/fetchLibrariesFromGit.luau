--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

local fs = require("@lune/fs")
local net = require("@lune/net")

local constants = require("@cli/constants")
local fmt = require("@cli/fmt")

local unzip = require("@pkgs-luau/unzip")

local function fetchLibrariesFromGit(progress: fmt.Progress): (boolean, string?)
	progress:writeOutput("Downloading from GitHub")

	local response = net.request(`{constants.GITHUB_LATEST_RELEASE}/download/libraries.zip`)

	if not response.ok then
		return false, response.statusMessage
	end

	progress:writeOutput("Unzipping")

	local archive = unzip.load(buffer.fromstring(response.body))

	archive:walk(function(entry)
		if entry.isDirectory then
			fs.writeDir(`{constants.BRIGHT_FOLDER}/lib/{entry:sanitizePath()}`)
			return
		end

		local content = archive:extract(entry, { type = "text" })
		fs.writeFile(`{constants.BRIGHT_FOLDER}/lib/{entry:sanitizePath()}`, content)
	end)

	return true, nil
end

return fetchLibrariesFromGit
