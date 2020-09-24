BINDGEN ?= bindgen
LIBNITROKEY ?= $(wildcard libnitrokey-v*)

src/ffi.rs: ${LIBNITROKEY}/NK_C_API.h
	quilt pop -af || true
	# always keep options in sync with build.rs
	${BINDGEN} \
		--whitelist-function "NK_.*" \
		--whitelist-var "NK_.*" \
		--whitelist-var "MAXIMUM_STR_REPLY_LENGTH" \
		--with-derive-default \
		--no-layout-tests \
		--output "$@" \
		"$<" \
		-- "-I${LIBNITROKEY}/libnitrokey"
	quilt push -a --refresh

.PHONY: verify-bindings
verify-bindings:
	@git diff --exit-code src/ffi.rs || \
		(echo "This test can only be executed on a clean working tree"; exit 1)
	@$(MAKE) --always-make src/ffi.rs > /dev/null
	@git diff --exit-code src/ffi.rs && \
		(echo "The generated bindings have been verified successfully!") || \
		(echo "Error: The generated bindings differ from the pre-generated bindings!"; exit 1)
