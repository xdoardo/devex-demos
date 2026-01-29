// Copyright Microsoft and CHERIoT Contributors.
// SPDX-License-Identifier: MIT

#include "../../include/example-error-handler.hh"
#include <compartment.h>
#include <debug.hh>
#include <unwind.h>

using Debug = ConditionalDebug<true, "RustRunner">;

template <typename... Args> void debug_log(const char *fmt, Args... args) {
  Debug::log(fmt, std::forward<Args>(args)...);
}

#define TEST(cond, msg, ...) Test::Invariant((cond), msg, ##__VA_ARGS__)

/* Things that Rust expects from us */
extern "C" void *cheriot_alloc(size_t size) {
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

/* Imports from Rust */
extern "C" char spatial_safety_error_stack();
extern "C" char spatial_safety_error_global();
extern "C" char spatial_safety_error_heap();
extern "C" char use_after_free();

int __attribute__((cheriot_compartment("runner"))) run() {

  CHERIOT_DURING
  spatial_safety_error_stack();
  CHERIOT_HANDLER
  Debug::log(
      "recovered from spatial safety error on a stack-allocated variable");
  CHERIOT_END_HANDLER

  CHERIOT_DURING
  spatial_safety_error_global();
  CHERIOT_HANDLER
  Debug::log("recovered from spatial safety error on a global variable");
  CHERIOT_END_HANDLER

  CHERIOT_DURING
  spatial_safety_error_heap();
  CHERIOT_HANDLER
  Debug::log("recovered from spatial safety error on a heap variable");
  CHERIOT_END_HANDLER

  CHERIOT_DURING
  use_after_free();
  CHERIOT_HANDLER
  Debug::log("recovered from use after free");
  CHERIOT_END_HANDLER

  return 0;
}
