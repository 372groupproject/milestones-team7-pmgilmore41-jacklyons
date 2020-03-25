helloworld: helloworld.rs
	rustc helloworld.rs

runhelloworld: helloworld
	./helloworld

snippet1: snippet1.rs
	rustc snippet1.rs

runsnippet1: snippet1
	./snippet1


clean:
	find . -depth 1 -type f -not -name '*.rs' -not -name 'Makefile' -delete