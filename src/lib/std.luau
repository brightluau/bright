local types = require("./types")

-- re-export all of the types that would be most used
export type Config<C> = types.Config<C>

local transformers: { [string]: types.Transformer<any> } = {}

--- @private
local function getTransformers(): { [string]: types.Transformer<any> }
	return transformers
end

--- Registers a transformer.
---
--- @param name string -- The name of the transformer. Used in output and for defining configuration.
--- @param config C -- A configuration object.
--- @param runner TransformerFunction -- The transformer itself.
local function registerTransformer<C>(name: string, config: C, runner: types.TransformerFunction<types.Config<C>>)
	if transformers[name] ~= nil then
		error(`transformer with name {name} already registered`, 2)
	end

	transformers[name] = {
		config = config,
		func = runner,
	} :: types.Transformer<any>
end

return table.freeze {
	registerTransformer = registerTransformer,

	_getTransformers = getTransformers,
}
