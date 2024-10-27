#include <3ds.h>

#define IS_N3DS                                                                \
  (OS_KernelConfig->app_memtype >= 6) // APPMEMTYPE. Hacky but doesn't use APT

bool pl_is_n3ds() { return IS_N3DS; }
