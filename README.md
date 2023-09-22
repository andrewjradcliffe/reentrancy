# Examples of reentrant programs

Closures and interior mutability provide interesting ways to
illustrate reentrant programs which are not necessarily memory safe
(if written in a language without a strict ownership model).

Why might someone write such programs? In most cases, it would be
unintentional; if a library permits a function registration interface,
then it is possible to introduce the necessary recursion.  On one
hand, we have obvious reentrancy, where user code creates recursive
calls that should be detected and handled the user -- any errors would
be the fault of the user. The `external` and `fib_iter` functions
provide examples of reentrant programs, expressed through a function
registration interface. `external` is a clear case of user-provided
code which, upon inspection, will generate an error (and infinite
recursion if the error did not occur). `fib_iter` is a circuitous
expression of something with which we are all familiar, but which
might resemble a real program which acts through a function
registration interface (i.e. call a callback sequence until some
condition is satisfied).

On the other hand, we have the possibility of user-introduced
recursive calls which cause problems in the library internals; this
would be the fault of the library writer. The latter case implies
either a problem in library design or an insufficiently tight function
registration interface. An example of this is provided in the
`internal` function, where library details lead to an internal error
(though the program would error due the user-written code if the
internal error did not occur).
