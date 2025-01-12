# Configuration

--8<-- "./docs/includes/snippets.md:construction"

Transformers can register a configuration definition for Bright to provide custom rules in its configuration file. These
allow a user to specify how a transformer should act, such as appending a comment to the end of a file rather than the
start. For transformers that inject new nodes into the CST, it is advisable that you define a configuration.

## Setup

As written in [Your First Transformer](./your-first-transformer.md#defining-configuration), a configuration is defined
like so:

```luau
local config = {
	some_option = {
		description = "Should do xyz",
		default = true
	}
}
type Config = bright.Config<typeof(config)>
```

Which maps to the following TOML:

```toml
[rules.<transformer-name>]
some_option = true
```

When registered with [`registerTransformer`](../../api/std/registerTransformer.md), your function will be provided a
table that conforms to the newly-created `Config` type as its second argument:

```luau
local function transformer(cst: bright.Cst, config: Config): bright.Cst
	if config.some_option then
		print("huzzah!")
	end
end

bright.registerTransformer("transformer", config, transformer)
```

Even if the configuration isn't defined in `bright.toml`, Bright will always provide values based on your default settings.
A default is **mandatory** because of this. Transformers cannot stop working because a configuration isn't provided, but
the default operation should be documented.

The `description` field of your configuration is used by Bright's `config` command. This generates a transformer rule
block and outputs it to the terminal, primarily to make it easier for end users to discover what configuration options
your transformer has.

## Type Validation

If an option is of the wrong type in the configuration file, Bright will throw a runtime error stating this.

Any type is valid as an option type, as long as it can be resolved down to a singular type and not a union, intersection,
or negation. If you do accidentally stumble into this, the type checker will throw an error like so:

```
'Config' type function errored at runtime: some_option cannot be a union
```

If you want to provide multiple possible values for a singular option key, you can use a table:

```luau hl_lines="4-7"
local config = {
	some_option = {
		description = "Should do xyz"
		default = {
			a = true,
			b = 123
		}
	}
}
```

And you will get a type like so:

```luau
type Config = {
	read some_option: {
		a: boolean,
		b: number
	}
}
```

!!! warning
	By using a table type, Bright cannot check if a configuration is valid at runtime, meaning that the onus is on you to
	validate and throw an appropriate error.
