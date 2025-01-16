# API Documentation

<!-- b:construction -->

The documentation on this part of the site pertains to the API of Bright and Poke. It is hand-written (for now), however
it is kept up to date and tags are added appropriately.

## Internals

The internals of Bright are also on this documentation, tagged with "Internal", however they should not be used in
end-user work. These are documented for developers who want to work on Bright itself and the libraries.

## Tags Reference

### <span class="md-tag md-tag-icon md-tag--experimental">Experimental</span> { id="tag-experimental" data-toc-label="Experimental" }

This part of the API is considered experimental, and may change or be removed at any point. This is only applied to APIs
that are only available in <!-- b:version dev --> versions of Bright.

### <span class="md-tag md-tag-icon md-tag--internal">Internal</span> { id="tag-internal" data-toc-label="Internal" }

This part of the API is for internal use, and is only documented for ease of development. Internal APIs are not considered
stable and may potentially break your transformers and/or tooling between non-major versions of Bright. They also do not
have any version tag to further discourage their use.

### <span class="md-tag md-tag-icon md-tag--yields">Yields</span> { id="tag-yields" data-toc-label="Yields" }

This method call yields the thread.
