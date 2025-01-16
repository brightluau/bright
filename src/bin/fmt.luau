--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

local process = require("@lune/process")
local stdio = require("@lune/stdio")

local colorful = require("@vendor/colorful")

local symbols = {
	success = colorful.color.green("✔"),
	error = colorful.color.red("✖"),
	warning = colorful.combineStyles({ colorful.color.yellow, colorful.modifier.bold })("!"),
	important = colorful.color.blue("★"),
}

local hintFormat = colorful.combineStyles({ colorful.color.gray, colorful.modifier.italic })

--- Formats the input arguments into a hint string.
local function hint(...): string
	return hintFormat(stdio.format(...))
end

--- Logs a success string.
local function success(...)
	stdio.write(`{symbols.success} {stdio.format(...)}\n`)
end

--- Logs an error and bails.
local function error(...)
	stdio.ewrite(`{symbols.error} {stdio.format(...)}\n`)
	process.exit(1) -- bail
end

--- Logs a warning.
local function warn(...)
	stdio.ewrite(`{symbols.warning} {stdio.format(...)}\n`)
end

--- Logs an important, but not operationally substantial, message.
---
--- Reserve for actually important messages (such as updates). Use `print()` for anything else.
local function important(...)
	stdio.write(`{symbols.important} {stdio.format(...)}\n`)
end

return {
	hint = hint,

	success = success,
	error = error,
	warn = warn,
	important = important,

	symbols = symbols,
}
