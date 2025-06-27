#include <stdint.h>

typedef void (*Callback)(int);

void call_callback(Callback cb) { cb(42); }
