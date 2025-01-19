--
-- This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not
-- distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
--

--- Converts a configuration definition to a K/V pair of configuration options.
export type function Config(userConfig)
	-- if we get any, just return any
	if userConfig:is("any") then
		return userConfig
	end

	-- if we get unknown, this is usually a typing issue, so give a hint
	if userConfig:is("unknown") then
		error("param is unknown"
			.. "\n[bright] This commonly occurs when defining a transformer. Add a type to your transformer's second argument.", 0)
	end

	if not userConfig:is("table") then
		error(`param must be a table, is actually {userConfig.tag}`, 0)
	end

	-- if there are no properties in the config table, then the config will never be passed to the transformer, so just
	-- return never
	if not next(userConfig:properties()) then
		return types.never
	end

	local config = types.newtable()

	for optionName, v in userConfig:properties() do
		local optionDef = v.read

		if not optionDef:is("table") then
			error(`{optionName:value()} is not a valid option definition`
				.. `\n[bright] You are probably trying to do \`{optionName:value()} = ...\`, which is not valid. `
				.. `Create an option definition like so: \`{optionName:value()} = \{ default = ... }\`.`, 0)
		end

		local defaultDefinition = optionDef:readproperty(types.singleton("default"))

		if not defaultDefinition then
			error(`{optionName:value()} is missing a default value`, 0)
		end

		if defaultDefinition:is("negation") then
			error(`{optionName:value()} cannot be a negation`, 0)
		end

		if defaultDefinition:is("union") or defaultDefinition:is("intersection") then
			error(`{optionName:value()} cannot be a {defaultDefinition.tag}`
				.. `\n[bright] This type is currently not supported by the runtime type checker.`, 0)
		end

		if defaultDefinition:is("singleton") then
			error(`{optionName:value()} cannot be a singleton`
					.. "\n[bright] A singleton does not make sense in the context of a configuration. Do you need to assert "
					.. "the type?", 0)
		end

		config:setreadproperty(optionName, defaultDefinition)
	end

	return config
end

--- A blank configuration.
export type NoConfig = Config<{}>

export type TransformerFunction<C> = (string, config: C) -> string

export type Transformer<C> = {
	name: string,
	config: C,
	func: TransformerFunction<Config<C>>,
}

export type TransformerConfigOption<T> = {
	description: string,
	default: T,
}

export type TransformerConfig = {
	[string]: TransformerConfigOption<any>
}

return nil
