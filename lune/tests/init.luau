require("./license")()

local consoleReporter = require("@lune-lib/reporter")
consoleReporter.init()

local frktest = require("@frktest/frktest")
frktest.run()
