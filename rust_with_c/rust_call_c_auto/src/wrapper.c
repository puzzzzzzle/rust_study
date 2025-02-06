#include "wrapper.h"
#ifdef _WIN32
#define EXPORT_SYMBOL __declspec(dllexport)
#else
#define EXPORT_SYMBOL
#endif

EXPORT_SYMBOL int zero() {
    return 0;
}