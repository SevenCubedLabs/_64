small:
	cargo rustc --bin _64 --release --features=minsize -- --emit=obj=smol/_64.o
	cd smol && ./smold.py -fno-align-stack -lc -lSDL2 -lGL _64.o ../_64
