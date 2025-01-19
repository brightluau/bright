--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

local process = require("@lune/process")

local argparse = require("./argparse")
local cli = require("./cli")
local fmt = require("./fmt")

local bright = argparse()
	:name("bright")
	:description("A scriptable tool for transforming and transpiling Luau code.")
	:epilog("Visit https://brightluau.dev/ for further documentation.")

for i, subcommand in cli do
	subcommand(bright)
end

local ok, info = pcall(function()
	return bright:parse(process.args)
end)

if not ok then
	fmt.bail("Internal error occurred:", info)
end
