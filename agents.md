# Agent instructions

Run `cargo bake agent:context:install` to install dependency context and skills, and use `cargo bake agent:context:list` and `cargo bake agent:context:show` to discover guidance. The installer excludes workspace members, so this repository's own guides are not installed here: follow them from `context/`, starting with `context/conventions.md`, together with the relevant installed skills under `.agents/`.

Before planning or changing behavior, discover and read applicable `design/*.md`. Use `base-design-review` to assess plans and changes, and `base-design-author` when user requirements are introduced, changed, or removed. Review design compliance before opening or updating a pull request.
