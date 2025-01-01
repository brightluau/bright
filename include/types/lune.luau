-- This breaks convention of being a "typedef" - this is a compatibility module for Lune so that Bright is more easily
-- invoked.

if (_VERSION):sub(1, 4) ~= "Lune" then
	error("attempt to use @bright/lune when not running under Lune", 2)
end

local process = require("@lune/process")

local bright = {}

local function _runBright(args: { string }): process.SpawnResult
	-- TODO: find bright executable
	return process.spawn("bright", args)
end

function bright.runTransformers(): process.SpawnResult
	return _runBright({ "run" })
end

return bright
