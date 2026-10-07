# Ergonomics

Designing the API from the call site backwards — the consumer's line of code is the spec.

- **Write the README example first.** Draft the three most common call sites before implementing anything; if the example needs a comment to be understood, redesign until it does not.
- **Layered depth:** a one-line happy path on top (`Lib::run(input)`), granular control beneath it (builders, options structs, traits/protocols to implement). Beginners never see the depth; experts never fight the shortcut.
- **Defaults are the design:** every optional knob has the value 90% of consumers want; the knob exists only if a real consumer needed it. An options struct with twenty fields nobody sets is API rot.
- **Accept the general, return the specific:** parameters take the loosest type that works (slices, iterables, path-likes, into-conversions); returns are concrete owned types the caller can use immediately.
- **Progressive disclosure in errors too:** a top-level error kind that matches broadly, detail underneath for those who dig — the consumer chooses the resolution.
- Constructors tell the truth: `new` is cheap and infallible; anything that touches IO or can fail is a named method (`connect`, `open`, `load`) returning a result.
- Documentation is runnable: every public item carries an example that compiles/executes in doc tests or the equivalent; examples are the API's regression suite.
- Feel the friction deliberately: build a small real consumer of your own library (the CLI shell counts) — pain felt there is API debt, fix it at the source, never in the consumer.
- Guard ergonomics in review: any new public item must show its call site in the change. No call site, no export.
