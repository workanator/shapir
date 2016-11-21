all:
	CFLAGS="-I/usr/local/opt/openssl/include" cargo build;

package:
	CFLAGS="-I/usr/local/opt/openssl/include" cargo package;

publish:
	CFLAGS="-I/usr/local/opt/openssl/include" cargo publish;
