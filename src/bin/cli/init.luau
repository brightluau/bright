--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

local argparse = require("@cli/argparse")

return {
	require("./cmd_init"),
	--require("./run"),
	require("./update"),
} :: { (argparse.Parser) -> () }
