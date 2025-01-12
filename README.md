<p align="center">
	<a href="https://brightluau.dev/">
		<img src="./docs/assets/images/hero.svg" height="250" alt="Bright hero">
	</a>
</p>

<h1 align="center">Bright</h1>
<p align="center">A scriptable tool for transforming and transpiling Luau code</p>

## Non-goals
- Drop-in replacement for Darklua - Bright is designed to be a much more complex transformer and will support more
  complex rule definitions, but in exchange this means that it won't act as a simple drop-in
- Simplicity - since Bright is scriptable, it will support a wide variety of transformations that you can apply to code,
  which means that writing a transformer will not be simple; clear documentation should be prioritised over simplicity
- Producing new ASTs - transformer scripts are for transforming existing ASTs, not making new ones
