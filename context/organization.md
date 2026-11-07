# Package Organization

Use this guide when naming a new Base crate or repository, or deciding where a capability belongs. It records how Base packages are grouped and why, so that later packages follow the same reasoning rather than a guess.

## Families

The `base-` prefix identifies a Base package in the flat crates.io namespace. It says nothing about what the package does, so related packages share a second-level prefix that names their family:

- `base-http*`: HTTP transport, including message semantics, bodies, protocol framing, clients, and servers. HTTP carries more than web pages, such as APIs, RPC, and proxies, so it is its own family rather than part of the web family.
- `base-web-*`: web content and addressing standards, such as URLs, media types, multipart bodies, HTML, and form content. The `base-web` name is reserved for the web framework, which will build on and re-export the family.
- `bake-*`: task libraries for Bake. They are development tooling, never runtime dependencies of a library.
- `base-*` without a family: standalone capabilities that no family describes, such as the scheduler, resource pools, endpoints, and benchmark tooling. A foundation that several families build on also has no family, so that neither depends on the other: `base-message` holds the header and trailer field sections that HTTP, email, and MIME messages share, and their interpretation, for `base-http` and `base-web-multipart` alike. Its scope is only those field sections; control data, content, framing, and transport stay with each protocol's crate.

Name a crate for the standard or concept it implements, then place it in the family that standard belongs to. A capability defined by a web standard belongs in `base-web-*` even when it has uses beyond the web: media types and multipart bodies also appear in email, but almost all of their use alongside Base is web content. Prefer the precise standard term over a shorter, ambiguous one: `base-web-media-type`, not `base-web-media`, which suggests images and video.

## Family members stand alone

Each family member must be useful on its own. Never make a family member depend on its family's umbrella or framework crate: `base-web-url` must not depend on `base-web`, and `base-http` must not depend on a server crate. The umbrella or framework crate depends on the members and may re-export them, so applications can depend on one crate while libraries depend only on what they use.

Members may depend on each other where one standard builds on another, such as `base-web-multipart` on `base-web-media-type`. Keep such dependencies in one direction, from the more specific standard to the more general one.

## Repositories and workspaces

Apply the repository and workspace rules in the Base Rust Conventions: a separate repository for each independently versioned crate, and one workspace for crates released together. A crate and data generated for it, such as `base-web-media-type` and `base-web-media-type-registry`, can share a workspace when they are released together.

## Taken names

Crates.io treats `-` and `_` as the same name. Check that a planned name and its variants are free before creating a repository; a taken generic name is one more reason to use a family prefix, as `base-web-url` avoids the unrelated `base_url` crate.
