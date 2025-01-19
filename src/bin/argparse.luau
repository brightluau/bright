--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

-- whilst argparse has been patched to expose its types, pesde refuses to regenerate the linker script to expose them,
-- so we're doing some trickery here!

local argparse = require("@pkgs/.pesde/caveful_games+argparse/0.1.2/argparse/src/lib")

export type Argument = argparse.argument
export type Option = argparse.option
export type Flag = argparse.flag
export type Command = argparse.command
export type Parser = argparse.Parser

return argparse
