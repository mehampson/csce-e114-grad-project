---
layout: layouts/base.njk
tags: project
eleventyNavigation:
  key: Howto
  title: How Do?
  order: 4
---
# How To Function

Before you get started with the implementation, you have three unknowns to figure out: your application architecture, your infrastructure, and the not-JavaScript language your function will be built in.

For our application, we're just going to use a simple Eleventy blog. This part doesn't actually matter too much -- it just needs to be able to call a serverless function on the client side. It's probably safe for us to assume that if you have a problem that serverless functions might be a good solution to, you already have this part figured out.

Our infrastructure in this example will just be Netlify. Strictly speaking, remember you need to handle hosting and serving both your compiled markup files and your serverless function -- it makes things easier if it's all in place but that's not a requirement. You could stick your markup on Cloudflare and call an AWS Lambda function from them if you needed.

Lastly, our function. We're going to build a little tool to let us roll some dice in the browser. And since we're all nerds here, of course we're not going to settle for your basic standard hexahedrons -- we'll need to choose from all the expanded dice sizes seen in board games and TTRPGs, because what's the point of building it at all if we don't do that. 

We'll build the function in Rust. Although Rust isn't *fully* supported yet by many serverless providers, the core functionality is there -- particularly for AWS-based systems, as they're a sponsor and have heavily invested in it. It's a very useful language: it's compiled rather than interpreted, so [performance is quite good](https://blog.scanner.dev/serverless-speed-rust-vs-go-java-python-in-aws-lambda-functions/), and it's memory-safe without resorting to garbage collection. Netlify, interestingly enough, will let us include the source files in our CI/CD and then compile the binary on their end. So we'll see how well it goes.