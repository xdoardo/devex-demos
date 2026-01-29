// Copyright Microsoft and CHERIoT Contributors.
// SPDX-License-Identifier: MIT

#include "../../include/example-error-handler.hh"
#include <compartment.h>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <debug.hh>
#include <unwind.h>

using Debug = ConditionalDebug<true, "RustRunner">;

template <typename... Args> void debug_log(const char *fmt, Args... args) {
  Debug::log(fmt, std::forward<Args>(args)...);
}

#define TEST(cond, msg, ...) Test::Invariant((cond), msg, ##__VA_ARGS__)

/* Things that Rust expects from us */
// Re-export because `cleanup_list_head` is marked as `__always_inline static inline`.
extern "C" struct CleanupList **get_cleanup_list_head() {
  return cleanup_list_head();
}

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

// Example 1
extern "C" void oob_read();

extern "C" bool json_verify(char *s) {
  int len = strlen(s);
  return len < 100;
}

// Example 2
extern "C" void uaf();
static char *DATA;
extern "C" void save(char *data) { DATA = data; }
extern "C" char load() { return *DATA; }

int __attribute__((cheriot_compartment("runner"))) run() {
  oob_read();
  uaf();
  return 0;
}
