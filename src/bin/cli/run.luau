--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

local fs = require("@lune/fs")

local argparse = require("@cli/argparse")
local config = require("@cli/config")
local fmt = require("@cli/fmt")

local types = require("@lib/types")

local function findTransformer(name: string): string?
	-- is it a literal file path?
	if fs.isFile(name) then
		return name
	end

	-- are we just missing the luau extension?
	if fs.isFile(`{name}.luau`) then
		return `{name}.luau`
	end

	-- try to find in the bright folder
	local path = `./bright/transformers/{name}.luau`

	if fs.isFile(path) then
		return path
	end

	-- are we adding an extra luau extension?
	local pathNoExt = `./bright/transformers/{name}`

	if fs.isFile(pathNoExt) then
		return pathNoExt
	end

	-- no idea
	return nil
end

local function run(opts: { input: string, output: string, transformers: { string } })
	if #opts.transformers == 0 then
		opts.transformers = config.transformers
	end

	if not fs.isDir(opts.input) then
		fmt.bail(`Source path \`{opts.input}\` does not exist.`)
	end

	if #opts.transformers == 0 then
		fmt.bail("No transformers to run.", fmt.hint("Have you configured any transformers in `bright.toml`?"))
	end

	local transformerStack: { types.Transformer<any> } = {}

	for _, transformer in opts.transformers do
		local location = findTransformer(transformer)

		if not location then
			fmt.bail(`Could not find transformer \`{transformer}\`.`)
		end
	end
end

return function(parser: argparse.Parser)
	local command = parser:command("run"):description("Executes and applies transformers on source code"):action(run)

	command:argument("transformers"):description("The transformers to run"):args("*")

	command:option("-i --input"):description("The input source code"):default(config.source)

	command:option("-o --output"):description("The output for the transformed code"):default(config.output)
end
