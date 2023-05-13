# Plugin-Licensor-Open-Source-API-Methods
Just an API method for transparency. I spent like a whole week rewriting this API method to try to make it more maintainable, to make it have the same output as "get_license", and to encrypt and sign the response.
I'm hoping to make a template for Rust API methods based on this code, but it eventually needs to be updated to use Elliptic Curve Cryptography, which might make the first template somewhat invalid if the first template uses RSA.

## FAQ
Why aren't the other API methods open source?
* Well... they are messy. Much messier than this since I was learning new ways to organize Rust Lambda functions using traits, structs, modules, and error handling as I wrote more Lambda Functions. I still have room for improvement with error handling though, as I don't use `.map_err()` or `?` in my code very often, and the folder structure could use some help.
