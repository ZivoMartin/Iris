iris_c:
cargo build && gcc -Wall other_languages/iris.c -o iris_c -liris -L./target/debug && LD_LIBRARY_PATH=./target/debug ./iris_c                            
