build:
	cargo build
	mkdir -p python
	mkdir -p r

	cp target/debug/librmath_c.dylib python/librmath.so
	cp target/rmath.h python/rmath.h
	cp rmath.i python/
	swig -python -outdir python python/rmath.i
	gcc -fPIC -Wno-register -Wall -c -o python/rmath_wrap.o python/rmath_wrap.c -Lpython/ $(shell python2-config --includes)
	gcc -Wno-deprecated-register -shared -o python/_rmath.so python/rmath_wrap.o -Lpython/ -lrmath -lpython
	
	cp target/debug/librmath_c.dylib r/librmath.so
	cp target/rmath.h r/rmath.h
	cp rmath.i r/
	swig -r -outdir r r/rmath.i	
	PKG_LIBS="r/librmath.so" R CMD SHLIB r/rmath_wrap.c -o r/rmath.so

clean:
	rm -rf target
	rm -rf python
	rm -rf r