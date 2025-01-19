local process = require("@lune/process")

require("./license")()

local consoleReporter = require("@lune-lib/reporter")
consoleReporter.init()

local frktest = require("@frktest/frktest")
process.exit(if frktest.run() then 0 else 1)
