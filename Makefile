BINDGEN ?= bindgen
LIBNITROKEY ?= $(wildcard libnitrokey-v*)

src/ffi.rs: ${LIBNITROKEY}/NK_C_API.h
	quilt pop -af || true
	${BINDGEN} \
		--whitelist-function "NK_.*" \
		--whitelist-var "NK_.*" \
		--whitelist-var "MAXIMUM_STR_REPLY_LENGTH" \
		--with-derive-default \
		--output "$@" \
		"$<" \
		-- "-I${LIBNITROKEY}/libnitrokey"
	quilt push -a
