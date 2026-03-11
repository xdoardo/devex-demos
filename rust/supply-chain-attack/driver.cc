// Copyright Microsoft and CHERIoT Contributors.
// SPDX-License-Identifier: MIT

#include "../../include/example-error-handler.hh"
#include <compartment.h>
#include <cstdint>
#include <debug.hh>
#include <unwind.h>

using Debug = ConditionalDebug<true, "RustRunner">;

template <typename... Args> void debug_log(const char *fmt, Args... args) {
  Debug::log(fmt, std::forward<Args>(args)...);
}

#define TEST(cond, msg, ...) Test::Invariant((cond), msg, ##__VA_ARGS__)

/* Things that Rust expects from us */
extern "C" void *cheriot_alloc(size_t size) {
  // debug_log("Trying to allocate {} bytes!", size);
  Timeout timeout{5};
  void *ret = heap_allocate(&timeout, MALLOC_CAPABILITY, size);

  Debug::Invariant((CHERI ::Capability{ret}.is_valid()),
                   "Allocation is invalid, got pointer: {} -- {}", ret,
                   (int)ret);
  return ret;
}

extern "C" void cheriot_free(void *ptr) { free(ptr); }

extern "C" void cheriot_panic() {
  Debug::Invariant((false), "Reached panic in Rust!");
}

extern "C" void cheriot_print_str(char *s) { printf("%s", s); }

extern "C" void do_it();

int __attribute__((cheriot_compartment("runner"))) run() {

  CHERIOT_DURING
  do_it();
  CHERIOT_HANDLER
  Debug::log("Something wrong happened, giving up!");
  CHERIOT_END_HANDLER

  return 0;
}
