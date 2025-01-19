-- From 0x5eal/luau-unzip: https://github.com/0x5eal/luau-unzip, licensed under MIT (see ./luau-unzip-LICENSE.md)

--> util: An MPSC synchronization primitive powered by Lua upvalues which retains only
--> one value at a time.

--- ## Usage
--- ```luau
--- local send, recv = watch((nil :: any) :: string)
--- task.delay(5, send, "hello, world!")
--- task.spawn(function()
---     local value = recv()
---     print("received value:", value)
--- end)
--- ```
type Watch<T> = {
	value: T?,
	receivers: { thread },
}

--- Creates a new `Watch` channel, returning its send and receive handles.
local function chan<T>(_phantom: T): ((T) -> (), () -> T?)
	local watch: Watch<T> = {
		value = nil,
		receivers = {},
	}

	local function send(value: T)
		watch.value = value

		for _, receiver in watch.receivers do
			coroutine.resume(receiver, value)
		end
	end

	local function recv(): T
		local value = watch.value
		watch.value = nil

		if value == nil then
			table.insert(watch.receivers, coroutine.running())
			return coroutine.yield()
		end

		return value :: T
	end

	return send, recv
end

return chan
