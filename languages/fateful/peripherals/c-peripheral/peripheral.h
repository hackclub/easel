#include <stdint.h>

__declspec(dllexport) void init(uint8_t);
__declspec(dllexport) uint8_t read(uint8_t);
__declspec(dllexport) void write(uint8_t, uint8_t);
__declspec(dllexport) void reset(void);
__declspec(dllexport) const char* name(void);
