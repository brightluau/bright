--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

local types = require("./types")

-- re-export all of the types that would be most used
export type Config<C> = types.Config<C>

--- Defines a transformer to be returned from the transformer module.
---
--- @param name string -- The name of the transformer. Used in output and for defining configuration.
--- @param config C -- A configuration object.
--- @param runner TransformerFunction -- The transformer itself.
---
--- @returns Transformer -- A transformer definition.
local function defineTransformer<C>(name: string, config: C, runner: types.TransformerFunction<types.Config<C>>): types.Transformer<C>
	return {
		name = name,
		config = config,
		func = runner,
	}
end

return table.freeze {
	defineTransformer = defineTransformer,
}
