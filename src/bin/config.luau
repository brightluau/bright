local fs = require("@lune/fs")
local serde = require("@lune/serde")

local constants = require("./constants")

type Config = {
	read source: string,
	read output: string,

	read transformers: { string },

	read rules: {
		[string]: {
			[string]: any,
		},
	},
}

local DEFAULT_CONFIG: Config = {
	source = "src/",
	output = "output/",

	transformers = {},
	rules = {},
}

local function readConfig(): Config
	if not fs.isFile(constants.CONFIGURATION_FILE) then
		return DEFAULT_CONFIG
	end

	local configFile = fs.readFile(constants.CONFIGURATION_FILE)

	local des = serde.decode("toml", configFile)

	-- all parameters are optional, so make sure they're all set
	for k, v in DEFAULT_CONFIG :: { [string]: any } do
		if des[k] == nil then
			des[k] = v
		end
	end

	return des :: Config
end

return readConfig()
