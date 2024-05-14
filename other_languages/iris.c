#ifndef SAFE_IRIS
#define SAFE_IRIS

extern void iris_new_request(char* req);
extern void iris_load_file(char* path);

void new_request(char* req) {
	iris_new_request(req);
}

void load_file(char* path) {
	iris_load_file(path);
}

#endif
