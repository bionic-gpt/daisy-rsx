---
name: ssg-whiz-new-article
description: Create a new blog article for a Rust static site that uses ssg_whiz. Use when the user wants a new article added under crates/<site>/content/blog/<slug>/ with a matching PageSummary entry added to crates/<site>/src/blog_summary.rs.
---

# SSG Whiz New Article

Use this skill when a user wants to add a new blog post to an existing `ssg_whiz` site in this workspace.

## Inputs

- Required: article title
- Optional: crate name, slug, publish date, description, author name, author image, hero image

If the user does not provide a crate, infer it from context or ask.
If the user does not provide a slug, derive one from the title.
If the user does not provide a date, default to today.

## Workflow

1. Ask for the article title if it is missing.
2. Identify the target crate, usually `crates/<site>`.
3. Inspect an existing article in `crates/<site>/content/blog/` and the matching `crates/<site>/src/blog_summary.rs` before editing.
4. Create a new folder at `crates/<site>/content/blog/<slug>/`.
5. Add `index.md` with a basic article stub.
6. If the user wants a placeholder image, add a simple generic image file to the same folder and reference it from the markdown and summary.
7. Update `crates/<site>/src/blog_summary.rs` by inserting a new `PageSummary` entry near the top of the list.
8. Run `cargo check -p <site>` if validation is useful and feasible.

## Article Stub

Keep the initial article simple and easy to revise. A good default is:

```md
# <Title>

## Summary

Short summary of the article.

![Hero image](hero.png "<Title>")

## Introduction

Draft introduction text.

## Key Points

1. First point
2. Second point
3. Third point

## Conclusion

Draft closing paragraph.
```

If the repository's existing articles use a different style, follow the local pattern instead of this stub.

## Updating blog_summary.rs

Match the existing `PageSummary` shape already used in the crate:

- `date`
- `title`
- `description`
- `folder`
- `markdown`
- `image`
- `author`
- `author_image`

Use the new article folder consistently in:

- `folder: "blog/<slug>/"`
- `markdown: include_str!("../content/blog/<slug>/index.md")`
- `image: Some("/blog/<slug>/<image-file>")` when an image exists

Insert the new entry in descending date order unless the file clearly follows a different ordering rule.

## Notes

- Prefer copying the conventions already present in the target crate over inventing a new structure.
- Keep the first draft minimal unless the user asks for a full article.
- If author metadata is consistent across existing posts, reuse that default unless the user asks for something else.
