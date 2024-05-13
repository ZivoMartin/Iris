#include <stdio.h>
#include <stdlib.h>
#include <string.h>


extern void iris_new_request(char* req);
extern void iris_load_file(char* path);


int main() {
	iris_load_file("demo.sql");
	iris_new_request("SELECT * FROM Humain WHERE 1==1;");
	return 0;
}
