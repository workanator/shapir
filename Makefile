all:
	CFLAGS="-I/opt/local/include" cargo build;

package:
	CFLAGS="-I/opt/local/include" cargo package;

