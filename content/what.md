---
layout: layouts/base.njk
tags: project
eleventyNavigation:
  key: What
  title: What Is This?
  order: 1
---

# What's This Project About?

JavaScript and TypeScript are often perfectly fine for many serverless functions, but they're not the only options available to us. Just looking around with a few quick Google searches:
* Netlify [officially supports Go](https://docs.netlify.com/functions/build/?fn-language=ts), and Rust as an ['experimental' option](https://www.netlify.com/blog/2021/10/14/write-netlify-functions-in-rust/). 
* AWS Lambda supports a [ton of languages](https://aws.amazon.com/lambda/features/): Java, Go, PowerShell, Node.js, C#, Python, and Ruby. They also have SDKs for [Rust](https://aws.amazon.com/sdk-for-rust/) and [Kotlin](https://docs.aws.amazon.com/sdk-for-kotlin/latest/developer-guide/kotlin_lambda_code_examples.html), and I assume they have others I'm just not seeing in their search. 
* Cloudflare's platform seems to support [anything they can compile to JavaScript](https://developers.cloudflare.com/workers/platform/languages/).

And so on.

