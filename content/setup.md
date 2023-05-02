---
layout: layouts/base.njk
tags: project
eleventyNavigation:
  key: Setup
  order: 4
---
# Setup

Here's your pre-work to follow along. First read Netlify's [instructions for running a Rust function](https://www.netlify.com/blog/2021/10/14/write-netlify-functions-in-rust/). Second, make sure [Rust is installed](https://www.rust-lang.org/tools/install) in your dev environment. And check to see what tools your IDE has for Rust development -- e.g. [Rust Analyzer](https://code.visualstudio.com/docs/languages/rust) if you use VS Code.





Now you have some new files in `netlify/functions`. Be sure to run `git add` at this point to bring them into you version control. 

Also, make sure `functions = "netlify/functions"` is in `netlify.toml`, so Netlify knows where to look.

Write your Rust function! You can just copy the one I wrote if you want a copy-paste example -- be sure to copy over everything in this project's `netlify/functions/dice`, not just the `src` folder. If you're starting from scratch, be aware that the template Netlify gives us has some out-of-date dependencies you'll need to update in 

Once you have your function written, make sure it compiles by running `cargo build` in its directory. You'll get some deprecation warnings if you're using Netlify's starter code, but for our purposes they're safe to ignore.

Run `netlify dev`, and then check out http://localhost:8888/.netlify/functions/dice?count=1&sides=d6 -- we get a response! Note the query parameters are not really necessary; the function I wrote expects them, but it knows what to do without them.

### The Frontend

Last up, add something on your frontend to actually let the user interact with the function. I've added a button with some [HTMX](https://htmx.org/) behind it, which seems like a reasonable way for a user to interact with the dice roller.