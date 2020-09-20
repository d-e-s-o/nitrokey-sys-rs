BINDGEN ?= bindgen
LIBNITROKEY ?= $(wildcard libnitrokey-v*)

src/ffi.rs: ${LIBNITROKEY}/NK_C_API.h
	${BINDGEN} \
		--whitelist-function "NK_.*" \
		--whitelist-var "NK_.*" \
		--whitelist-var "MAXIMUM_STR_REPLY_LENGTH" \
		--output "$@" \
		"$<" \
		-- "-I${LIBNITROKEY}/libnitrokey"
