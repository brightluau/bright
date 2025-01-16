# Development and You

<!-- b:construction -->

So you've installed Bright, tinkered with a few things, decided it wasn't enough and thought "I've got too much time to
kill"? Well, you're in the right place, and I wish you good health from this point forward.

## Transformer Development

Transformers are the meat and potatoes of Bright, and are the part of the tool that is the most flexible. They let you
manipulate your code in a variety of ways, from stripping comments to transforming statements as if they're macros, and
are extremely powerful.

### Before you begin

Before you start writing your own transformers, consider the following:

1. **Transformer development sucks.** This isn't because the API is bad or inherently hard to learn, but more the fact
   that developing transformers sucks the living soul out of you when debugging why they don't work.
2. **It might already exist.** A lot of people publish their transformers for others to use, and existing transformers
   are quite configurable. Why not shop around first so that you don't reinvent the wheel?
3. **You might not need one.** If you are writing a transformer for a specific purpose, make sure that it's not just an
   inherent fault in the way you've written your code. Transformers aren't for fixing mistakes or bad practices (though
   you definitely can use them to do that).

### Pointers

Now that you've read the above and _still_ decided to go through with writing one, here's some pointers to documentation
pieces that you should read.

<div class="grid cards" markdown>

-   [**Poke and You**](./poke/index.md)

    ***

    Bright is powered by Poke, so you should learn Poke's API before writing transformers. It's a real page turner.

-   [**Transformonomicon**](./transformonomicon/index.md)

	***

	The transformonomicon contains all of the information for writing transformers, from how Bright calls your
	code to how to make your transformers customizable.

</div>

## Bright Development

If you're one of the few people that actually wants to use Bright's API or even contribute to the CLI, then first off,
wow, and second, you're also in the right place. Head over to the [Bright](./bright/index.md) part of the developer
guide to find out more.
