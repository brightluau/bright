-- Based on 0x5eal/luau-unzip: https://github.com/0x5eal/luau-unzip, licensed under MIT (see ./luau-unzip-LICENSE.md)

--> lib: Extension to base frktest reporter for live status reporting

local stdio = require("@lune/stdio")

local frktest = require("../../lune_packages/frktest")
local Reporter = frktest._reporters.lune_console_reporter

local watch = require("./channel")

local STYLE = table.freeze({
	suite = function(name: string)
		return `\x1b[1;30;105m SUITE \x1b[39m\x1b[49m {name}`
	end,

	report = function(name: string, state: "success" | "error" | "skip", elapsed: number)
		local state_color: stdio.Color = if state == "success"
			then "102"
			elseif state == "error" then "101"
			elseif state == "skip" then "103"
			else error("Invalid test state")
		return `    \x1b[1;30;{state_color}m {if state == "skip" then "SKIP" else "TEST"} \x1b[0;39;49m {name} \x1b[0m[\x1b[2m{string.format(
			"%.2fms",
			elapsed
		)}\x1b[0m]`
	end,
})

local ReporterExt = {}
function ReporterExt.init()
	frktest.test.on_suite_enter(function(suite)
		print(STYLE.suite(suite.name))
	end)

	frktest.test.on_suite_leave(function()
		stdio.write("\n")
	end)

	local send_ts, recv_ts = watch((nil :: any) :: number)

	frktest.test.on_test_enter(function()
		-- Send over some high precision timestamp when the test starts
		return send_ts(os.clock())
	end)

	frktest.test.on_test_leave(function(test)
		print(STYLE.report(
			test.name,
			if test.failed then "error" else "success",

			-- Await receival of the timestamp and convert the difference to ms
			(os.clock() - assert(recv_ts())) * 1000
		))
	end)

	frktest.test.on_test_skipped(function(test)
		print(STYLE.report(test.name, "skip", 0))
	end)

	Reporter.init()
end

return setmetatable(ReporterExt, { __index = Reporter })
