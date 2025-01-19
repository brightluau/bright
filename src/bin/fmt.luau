--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

local process = require("@lune/process")
local stdio = require("@lune/stdio")
local task = require("@lune/task")

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

--- Logs an error.
local function error(...)
	local fmt = stdio.format(...)

	if fmt:sub(1, 1) == "\n" then
		-- this probably relates to a progress thing, don't add a symbol
		stdio.ewrite(`{stdio.format(...)}\n`)
	else
		stdio.ewrite(`{symbols.error} {stdio.format(...)}\n`)
	end
end

--- Logs an error and bails.
local function bail(...): never
	error(...)
	return process.exit(1) -- bail
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

local spinners = { "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏" }

export type Progress = {
	step: (self: Progress, prevStepSymbol: string, stepNum: number, newText: string?) -> (),
	finish: (self: Progress, finalSymbol: string) -> (),
	writeOutput: (self: Progress, output: string) -> (),
	tick: (self: Progress, unstable: boolean?, prepend: string?) -> (),
} & {
	_maxSteps: number,
	_step: number,
	_text: string,

	_outputStack: { string },
	_lastWrittenLineCount: number,
	_currentSpinner: number,

	_thread: thread,

	_skipTick: boolean,
}

--- Starts a progress spinner that's indeterminate
local function progress(initialMessage: string, stepsCount: number): Progress
	local progressSpinner = {
		_maxSteps = stepsCount,
		_step = 1,
		_text = initialMessage,
		_outputStack = {},
		_lastWrittenLineCount = 0,
		_currentSpinner = 1,
		_skipTick = false,
	} :: Progress

	local function blank()
		local outputBuf = ""

		for _ = 1, progressSpinner._lastWrittenLineCount do
			outputBuf ..= `\x1b[2K\x1b[0G\x1b[1A`
		end

		outputBuf ..= "\x1b[2K\x1b[0G"

		return outputBuf
	end

	local function step(self: Progress, prevStepSymbol: string, stepNum: number, newText: string?)
		local outputBuf = blank()

		outputBuf ..= colorful.color.gray(`[{self._step}/{self._maxSteps}] `)
		outputBuf ..= `{prevStepSymbol} {self._text}`
		outputBuf ..= `{string.rep("\n", progressSpinner._lastWrittenLineCount + 1)}`

		self._step = stepNum
		if newText then
			self._text = newText
		end

		self._outputStack = {}

		self:tick(true, outputBuf)
	end

	local function writeOutput(self: Progress, output: string)
		table.insert(self._outputStack, output)

		while #self._outputStack > 5 do
			table.remove(self._outputStack, 1)
		end

		self:tick(true)
	end

	local function finish(self: Progress, finalSymbol: string)
		task.cancel(self._thread)

		local outputBuf = blank()

		outputBuf ..= colorful.color.gray(`[{self._step}/{self._maxSteps}] `)
		outputBuf ..= `{finalSymbol} {self._text}\n`

		stdio.write(outputBuf)
	end

	local function tick(self: Progress, unstable: boolean?, prepend: string?)
		local outputBuf = prepend or ""
		local stepCounter = ""

		if progressSpinner._maxSteps then
			stepCounter = colorful.color.gray(`[{self._step}/{self._maxSteps}] `)
		end

		outputBuf ..= blank()
		outputBuf ..= `{stepCounter}{colorful.color.yellow(spinners[self._currentSpinner])} {self._text}`

		for _, line in self._outputStack do
			outputBuf ..= "\n" .. colorful.color.gray(line)
		end

		stdio.write(outputBuf)

		self._lastWrittenLineCount = #self._outputStack

		if unstable then
			self._skipTick = true
		end
	end

	progressSpinner.step = step
	progressSpinner.finish = finish
	progressSpinner.writeOutput = writeOutput
	progressSpinner.tick = tick

	progressSpinner._thread = task.spawn(function()
		while true do
			if not progressSpinner._skipTick then
				progressSpinner:tick()
			end

			progressSpinner._skipTick = false

			progressSpinner._currentSpinner += 1
			if progressSpinner._currentSpinner > #spinners then
				progressSpinner._currentSpinner = 1
			end

			task.wait(0.1)
		end
	end)

	task.wait() -- yield one cycle

	return progressSpinner :: Progress
end

return {
	hint = hint,

	success = success,
	error = error,
	bail = bail,
	warn = warn,
	important = important,

	progress = progress,

	symbols = symbols,
}
