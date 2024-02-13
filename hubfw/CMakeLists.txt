cmake_minimum_required(VERSION 3.13)

add_definitions(-DUSBD_MANUFACTURER="RWCockpit")
add_definitions(-DUSBD_PRODUCT="Hardware Hub")

include(pico_sdk_import.cmake)

project(hubfw)

pico_sdk_init()

add_executable(hubfw src/main.c)
target_link_libraries(hubfw pico_stdlib)
pico_add_extra_outputs(hubfw)
pico_enable_stdio_usb(hubfw 1)
pico_enable_stdio_uart(hubfw 1)