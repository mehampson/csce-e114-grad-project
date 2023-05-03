---
layout: layouts/base.njk
tags: project
eleventyNavigation:
  key: What
  title: What Is This?
  order: 1
---

## What's This Project About?

This site is my graduate project for CSCI E-114. We're going to take a look at writing serverless functions in languages other than JavaScript.

JavaScript and TypeScript are often perfectly fine for many serverless functions, but they're not the only options available to us. Just looking around with a few quick Google searches:
* Netlify [officially supports Go](https://docs.netlify.com/functions/build/?fn-language=ts), and Rust as an ['experimental' option](https://www.netlify.com/blog/2021/10/14/write-netlify-functions-in-rust/). 
* AWS Lambda supports a [ton of languages](https://aws.amazon.com/lambda/features/): Java, Go, PowerShell, Node.js, C#, Python, and Ruby. They also have SDKs for [Rust](https://aws.amazon.com/sdk-for-rust/) and [Kotlin](https://docs.aws.amazon.com/sdk-for-kotlin/latest/developer-guide/kotlin_lambda_code_examples.html), and I assume they have others I'm just not seeing in their search. 
* Cloudflare's platform seems to support [anything they can compile to JavaScript](https://developers.cloudflare.com/workers/platform/languages/).

And so on.

## Why Bother? JavaScript's fine.

JavaScript's easy and universal, and TypeScript's not far behind. But every language has its strengths and weaknesses. Depending on your application, you might find something else is a better fit for your needs. Some of the big features you might find elsewhere:
* Type Safety: Many of the languages mentioned above are strongly typed, reducing the possibility of unexplained behavior from execution paths you didn't anticipate -- or at least making it easier to troubleshoot those bugs when they occur.

* Performance: Once you're out of your platform's free tier, you're paying for a function's compute time. If your function responds twice as quickly in a different language than in it does in JavaScript, it'll cost you half as much to run. Or you might find that JavaScript can't reliably complete an operation in your platform's function time-out window, but a faster language can.
* Native Ecosystems: Another language might simply be the obvious choice for your application. For example, if you're writing a function for a data science application, it's hard to justify using JavaScript over Python. Or you might need to use one specific library and it doesn't have JavaScript FFI. 
* Developer Preference: Who says Javascript is your default first-choice anyway? Or that you know it well enough to write the function you need? If you're more fluent with a different language and it's well-supported on your serverless platform, that's a perfectly legitimate reason.

So how do we actually do it? It's actually not much different than using JavaScript. Let's walk through [setting one up on Netlify](/howto).