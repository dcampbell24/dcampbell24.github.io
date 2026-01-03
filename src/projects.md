## Projects

### Hnefatafl [1][1] [2][2]

[1]: https://hnefatfl.org
[2]: https://github.com/dcampbell24/hnefatafl

### Financial Accounts [3][3]

[3]: https://github.com/dcampbell24/financial-accounts

### Cubes [4][4] (Bash, C, Fortran, Go, Julia, Python, Rust, Tcl)

[4]: https://github.com/dcampbell24/cubes

One day David was at his uncle's house spending time with relatives and his
uncle brought out some puzzles for everyone to play with. The puzzles consisted
of a collection of [polycubes] which fit together to form 3x3x3 cubes.

[polycubes]: http://en.wikipedia.org/wiki/Polycube

While struggling to solve the puzzles by hand he quipped, "I bet a computer
could do this in no time at all", and so he decided he should get a computer
to solve them for him.

Since writing the models of the pieces by hand that the program needs as input
is tedious and error prone, he wrote a GUI program to create the models with.
He also created a program to display what the solution looks like.

He discovered that, although, in general the space packing problem is hard to
solve, it can be done very quickly for such a small region if some tricks are
used to reduce the search space.

Important optimizations include caching previous calculations from the search,
not looking at rotations of the first piece, trying to place the pieces from
hardest to easiest, and using flood fill to find regions that can't possibly be
filled.

### Julia Benchmarks [5][5] (Julia)

[5]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/measurements/julia.html

Julia is a language for doing scientific computing so it is important that
Julia is fast. David wrote several of the [benchmarks game] benchmarks to
show how Julia compares with other languages and to help prevent performance
regressions.

[benchmarks game]: https://github.com/JuliaLang/julia/pulls?q=is%3Apr+author%3Adcampbell24

<p xmlns:cc="http://creativecommons.org/ns#" xmlns:dct="http://purl.org/dc/terms/"><a property="dct:title" rel="cc:attributionURL" href="https://dlc.name">DLC's Website</a> by <a rel="cc:attributionURL dct:creator" property="cc:attributionName" href="https://dlc.name">David Lawrence Campbell</a> is licensed under <a href="https://creativecommons.org/licenses/by-nc-nd/4.0/?ref=chooser-v1" target="_blank" rel="license noopener noreferrer" style="display:inline-block;">Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International<img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1" alt=""><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/by.svg?ref=chooser-v1" alt=""><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/nc.svg?ref=chooser-v1" alt=""><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/nd.svg?ref=chooser-v1" alt=""></a></p>