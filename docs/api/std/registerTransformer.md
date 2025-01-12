---
tags: [Experimental]
---

# `#!luau function registerTransformer`
<!-- b:version dev -->

```luau
function bright.registerTransformer<C>(
	name: string,
	config: C,
	func: TransformerFunction<Config<C>>
): ()
```

Registers a transformer for Bright to use.

The `func` provided will be given a configuration object as defined by [`bright.Config<C>`](./Config.md).
