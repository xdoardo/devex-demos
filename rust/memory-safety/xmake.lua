-- Copyright Microsoft and CHERIoT Contributors.
-- SPDX-License-Identifier: MIT

set_project("Memory safety examples in Rust")
sdkdir = "../../cheriot-rtos/sdk"
includes(sdkdir)
set_toolchains("cheriot-clang")

option("board", function()
	set_default("sail")
end)

target("runner", function()
	add_rules("cheriot.compartment")
	add_deps("freestanding", "debug", "stdio")
	add_files("driver.cc")
	add_files("./memory_safety/Cargo.toml", { sourcekind = "cheriot_rust_crate" })
	  add_rcflags("--crate-type=staticlib", {force = true})
end)

-- Firmware image for the example.
firmware("memory_safety")
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
