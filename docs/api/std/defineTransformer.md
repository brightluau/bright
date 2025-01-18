---
tags: [Experimental]
---

# `#!luau function defineTransformer`
<!-- b:version dev -->

```luau
function defineTransformer<C>(
	name: string,
	config: C,
	func: TransformerFunction<Config<C>>
): Transformer<C>
```

Defines a transformer to be returned from the transformer module.

The `func` provided will be given a configuration object as defined by [`Config<C>`](./Config.md).
