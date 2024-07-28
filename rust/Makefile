profile-valgrind = CARGO_PROFILE_RELEASE_DEBUG=true cargo build --release; \
	valgrind --tool=callgrind --callgrind-out-file=callgrind.out \
		--collect-jumps=yes --simulate-cache=yes \
		target/release/process $(1)

profile-flame = CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph -- $(1)

profile.valgrind.%:
	$(call profile-valgrind, measurements_e$*.txt)

profile.flame.%:
	$(call profile-flame, measurements_e$*.txt)
