helloworld: helloworld.rs
	rustc helloworld.rs

runhelloworld: helloworld
	./helloworld

snippet1: snippet1.rs
	rustc snippet1.rs

runsnippet1: snippet1
	./snippet1

runif: p2_if
	./p2_if

runfor: p2_for
	./p2_for

runwhile: p2_while
	./p2_while

runloop: p2_loop
	./p2_loop

runswitch: switch
	./switch

struct: struct.rs
	rustc struct.rs

runstruct: struct
	./struct
	
clean:
	find . -depth 1 -type f -not -name '*.rs' -not -name 'Makefile' -delete
