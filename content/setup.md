---
layout: layouts/base.njk
tags: project
eleventyNavigation:
  key: Setup
  parent: Howto
---
# Setup

Here's your pre-work to follow along. First read Netlify's [instructions for running a Rust function](https://www.netlify.com/blog/2021/10/14/write-netlify-functions-in-rust/). Second, make sure [Rust is installed](https://www.rust-lang.org/tools/install) in your dev environment. And check to see what tools your IDE has for Rust development -- e.g. [Rust Analyzer](https://code.visualstudio.com/docs/languages/rust) if you use VS Code.

### The App

Once we're ready to get started, clone the Eleventy starter of your choice and start a new Github repo with it. There's nothing distinctive about this part. I used https://github.com/11ty/eleventy-base-blog. Trim out anything you don't plan to use, and add whatever you feel is missing.

### The Infrastructure

Next, we'll set up Netlify. Add a new site on Netlify, and link your repo to it.  Then run `netlify init` and follow the usual steps to connect your Eleventy app to Netlify. At this point, it's a good idea to test your CI/CD: push a commit to Github and make sure the Netlify build runs to success.

### The Serverless Function

Now we're ready to think about our function. Rust functions on Netlify are still experimental, so we need to enable them. Run: `netlify env:set NETLIFY_EXPERIMENTAL_BUILD_RUST_SOURCE true` in your terminal.

We don't need to set up a separate repo for our function: we can nest it inside our Eleventy app just like a normal JavaScript one. If you prefer, you can manually create `netlify/functions` and run `cargo init <name>` from there to spin up the new Rust package. I've called my function `dice`.

Netlify also provides a starter template, so we'll use that instead: `netlify functions:create --language=rust`. Following the wizard, you'll want to set it up as a Serverless function on the default path.

Now you have some new files in `netlify/functions`. Be sure to run `git add` at this point to bring them into you version control. You'll also want to add some [appropriate gitignore rules](https://github.com/github/gitignore/blob/main/Rust.gitignore) at this point.

Also, make sure `functions = "netlify/functions"` is in `netlify.toml`, so Netlify knows where to look.

Write your Rust function! You can just copy the one I wrote if you want a copy-paste example -- be sure to copy over everything in this project's `netlify/functions/dice`, not just the `src` folder. If you're starting from scratch, be aware that the template Netlify gives us has some out-of-date dependencies you'll need to update in `Cargo.toml`. (Most importantly, bump simple_logger from 1.16.0 to 4.1 or your program won't compile.)

Once you have your function written, make sure it compiles by running `cargo build` in its directory. You'll get some deprecation warnings if you're using Netlify's starter code, but for our purposes they're safe to ignore.

Run `netlify dev`, and then check out http://localhost:8888/.netlify/functions/dice?count=1&sides=d6 -- we get a response! Note the query parameters are not really necessary; the function I wrote expects them, but it knows what to do without them.

### The Frontend

Last up, add something on your frontend to actually let the user interact with the function. I've added a button with some [HTMX](https://htmx.org/) behind it, which seems like a reasonable way for a user to interact with the dice roller.