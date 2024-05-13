#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

typedef struct Sender Sender;

extern void iris_new_request(char* req);
extern void iris_init();


int main() {
	iris_init();
	iris_new_request("SELECT * FROM Humain WHERE 1==1;");
	sleep(1);
	return 0;
}
