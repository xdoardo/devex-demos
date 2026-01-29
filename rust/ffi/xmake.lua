-- Copyright Microsoft and CHERIoT Contributors.
-- SPDX-License-Identifier: MIT

set_project("What happens to Rust's invariants when crossing FFI boundaries?")
sdkdir = "../../cheriot-rtos/sdk"
includes(sdkdir)
set_toolchains("cheriot-clang")

option("board", function()
	set_default("sail")
end)

target("runner", function()
	add_rules("cheriot.compartment")

	-- memcpu
    add_deps("freestanding", "string", "crt", "cxxrt", "atomic_fixed", "compartment_helpers", "debug", "softfloat")
    add_deps("message_queue", "locks", "event_group", "cheriot.allocator", "stdio", "strtol", "unwind_error_handler")
	add_files("driver.cc")
	add_files("./ffi/Cargo.toml", { sourcekind = "cheriot_rust_crate" })
	  add_rcflags("--crate-type=staticlib", {force = true})
end)

-- Firmware image for the example.
firmware("ffi")
  add_deps("runner")
    on_load(function(target)
        target:values_set("board", "$(board)")
        target:values_set("threads", {
            {
                compartment = "runner",
                priority = 1,
                entry_point = "run",
                stack_size = 0x800,
                trusted_stack_frames = 1
            }
        }, {expand = false})
    end)
