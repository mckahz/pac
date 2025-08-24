# Canonicalization
In canonicalizing the source code AST for our program, we need an environment containing all constructors (for evaluating expressions) and types (for type checking).

However, in order to have the interface

THIS IS WHY EXPLICIT IMPORTS ARE IMPORTANT!!!

The interface for a particular module should only be generated after that module is canonicalised. In order to have
