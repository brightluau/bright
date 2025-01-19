local process = require("@lune/process")

export type AnnotationInfo = {
	file: string?,
	title: string?,
	col: number?,
	endColumn: number?,
	line: number?,
	endLine: number?,
}

local function checkGitHub(): boolean
	return process.env["GITHUB_ACTIONS"]
end

local function annotation(type: string, message: string, info: AnnotationInfo?)
	if not checkGitHub() then
		return -- no-op if this isn't github actions
	end

	local output = `::{type}`

	if info then
		local attributes: { string } = {}
		for k, v in info :: { [string]: any } do
			table.insert(attributes, `{k}={v}`)
		end

		output ..= " " .. table.concat(attributes, ",")
	end

	output ..= `::{message}`

	print(output)
end

local function debug(message: string)
	annotation("debug", message)
end

local function notice(message: string, info: AnnotationInfo?)
	annotation("notice", message, info)
end

local function warning(message: string, info: AnnotationInfo?)
	annotation("warning", message, info)
end

local function error(message: string, info: AnnotationInfo?)
	annotation("error", message, info)
end

local function group(groupName: string)
	if not checkGitHub() then
		return
	end

	print(`::group::{groupName}`)
end

local function endGroup()
	if not checkGitHub() then
		return
	end

	print("::endgroup::")
end

return {
	debug = debug,
	notice = notice,
	warning = warning,
	error = error,

	group = group,
	endGroup = endGroup,
}
