# smallcheck - an exhaustive checker

smallcheck is an exhaustive checker.

In the vein of [quickcheck](https://github.com/BurntSushi/quickcheck)
`smallcheck` is a testing tool which asks that you specify properites of the
system under test and automatically generates inputs for testing. Where
`quickcheck` uses random sampling over the domain of inputs `smallcheck`
exhausts the domain, to some 'depth', going from smallest to largest values.

This implementation is based on:

    * Runciman, C., Naylor, M., & Lindblad, F. (2008). Smallcheck and lazy smallcheck: automatic exhaustive testing for small values. Acm Sigplan Notices.
    * Reich, J. S., Naylor, M., & Runciman, C. (2012). Advances in Lazy SmallCheck. In Implementation and Application of Functional Languages (Vol. 8241, pp. 53–70). Berlin, Heidelberg: Springer, Berlin, Heidelberg. http://doi.org/10.1007/978-3-642-41582-1_4

- - -

I listened to so much Brian Eno when writing this library. So. Much. 
