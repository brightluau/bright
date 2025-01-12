local clappy = require("@vendor/clappy/types")

return {
	require("./run"),
	require("./cmd_init"),
} :: { clappy.CommandInfo }
