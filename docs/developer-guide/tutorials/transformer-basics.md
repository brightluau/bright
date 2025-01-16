---
description: Transformer development starts somewhere, and in this case at least, it starts right here.
---

# Transformer Basics

<!-- b:construction -->

!!! abstract

	This tutorial will cover the basics of writing a transformer. By the end of this, you will have written a skeleton
	of a transformer, ready to make it do something interesting.

Transformers are the core component of Bright, and are the functions executed by the CLI to modify your code.

## Writing the transformer

Before we do anything too complex with manipulation, let's pedal back and just get Bright to load and run *something*.
This is actually quite a trivial task! Let's start out like this:

```luau title="bright/transformers/someTransformer.luau"
local bright = require("@bright/std") -- (1)!

local function transformer(cst: bright.Cst): bright.Cst
	return cst
end
```

1.  Bright's libraries are commonly aliased to `@bright/` for convenience and cross-compatibility (and this is recommended),
    but your project doesn't necessarily need to be set up this way. The CLI automatically generates a `.luaurc` file for
	this support if you're using [pesde](https://pesde.dev/). All libraries are accessible through a root import if
	necessary, so `#!luau require("path/to/bright").std` works as well.

This is the simplest possible transformer: a no-op. We'll add more to this later, of course, but for now let's break down
what's going on.

`bright.Cst` is a re-export of [Poke](../poke/index.md)'s `Cst` type. A CST is like an AST, except that it includes extra
information about spacing and comments, referred to as trivia. The Poke documentation contains more information about them,
as well as the API for how you can manipulate them.

## Defining configuration

Transformers often take a configuration, referred to as the *rules* of a transformer. As you should've spotted, we use
a TOML file to define Bright's overall configuration, but this also includes the rules for transformers. For the sake of
the example, let's say we want our transformer to take this sort of configuration:

```toml title="bright.toml"
[rules.some-transformer]
some_option = true
```

??? note "Regarding naming"
	TOML's specification doesn't dictate whether dashes or underscores should be used for naming and both are valid,
	however we recommend naming your transformers with dash-separated names (like `some-transformer`), and your rule keys
	with underscores (like `some_option`). This seems to be the common way of doing things and is also the most convenient
	to do in Luau.

We can define it like this in Luau:

```luau
local config = {
	some_option = {
		description = "Should do xyz",
		default = true
	}
}
```

This object defines that we have `some_option`, with a default value of `true`, and a description of... nothing of substance,
but it does have one. By itself, this is all Bright needs at runtime to automatically generate a configuration object to
pass to your transformer function, but the type system needs a little help. This is where [`bright.Config`](../../api/std/Config.md)
comes in:

```luau
type Config = bright.Config<typeof(config)>
```

This is a type function which takes in your config object and produces a type that looks like this:

```luau
type Config = {
	read some_option: boolean
}
```

As you can see, it returns a key-value pair of your options and the type of the **default value**. Options can only be of
one type, so no variants here!

To accept your new configuration, update your `transformer` function to look like the following:

```luau hl_lines="1"
local function transformer(cst: bright.Cst, config: Config): bright.Cst
	return cst
end
```

Then, you can read the configuration options just as you would expect:

```luau hl_lines="2-4"
local function transformer(cst: bright.Cst, config: Config): bright.Cst
	if config.some_option then
		print("some_option set!")
	end

	return cst
end
```

There are some quirks with how configuration works, so it is advisable to read [Configuration](../transformonomicon/configuration.md) as you
develop your transformers.

## Registration

For Bright to detect your transformer, you need to register it. This is accomplished with
[`bright.registerTransformer`](../../api/std/registerTransformer.md). It takes three arguments: the transformer's name,
the configuration object, and the transformer function.

```luau title="bright/transformers/someTransformer.luau"
bright.registerTransformer("some-transformer", config, transformer)
```

That's it! Your transformer is now ready for use!

!!! tip
	Because `registerTransformer` takes a name, you can register a transformer under a different name compared to its
	file name, or you can even register multiple transformers per file if necessary (or you want a common set of transformers).

## Conclusion

You've now got a transformer that looks like this:

```luau title="bright/transformers/someTransformer.luau"
local bright = require("@bright/std")

local config = {
	some_option = {
		description = "Should do xyz",
		default = true
	}
}
type Config = bright.Config<typeof(config)>

local function transformer(cst: bright.Cst, config: Config): bright.Cst
	if config.some_option then
		print("some_option set!")
	end

	return cst
end

bright.registerTransformer("some-transformer", config, transformer)
```

Congratulations! You now have your first ever transformer! It only goes downhill from here, trust me...
