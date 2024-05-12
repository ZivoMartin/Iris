#include <stdio.h>
#include <stdlib.h>

extern void iris_new_request(char* req);

int main() {
	iris_new_request("Hello Rust, from C");
	return 1;
}
