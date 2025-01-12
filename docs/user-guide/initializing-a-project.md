---
description: Set your project up with Bright.
---

# Initializing a Project

--8<-- "./docs/includes/snippets.md:construction"

To get Bright set up on a project, run `bright init`. This will set up a `bright/` directory and add a `bright.toml` file
with a basic configuration. Notably, you will have no rules at this point, which is where you have to pick a path.

## Preset Rules

This is the path most people will take, and the path recommended for simplicity's sake. Check out the [transformers](../transformers/index.md)
page for some common rules you can grab, as well as some community-provided ones.

## Custom Rules

If you want to write your own transformer rules, you should get started with the [developer guide](../developer-guide/index.md#transformer-development).
It's going to be a long ride, friend.
