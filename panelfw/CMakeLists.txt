cmake_minimum_required(VERSION 3.13)

if (NOT DEFINED ENV{RWCOCKPIT_PANELFW_I2C_ADDR})
    message(FATAL_ERROR "RWCOCKPIT_PANELFW_I2C_ADDR is not set.")
endif ()

add_definitions(-DUSBD_MANUFACTURER="RWCockpit")
add_definitions(-DUSBD_PRODUCT="Panel $ENV{RWCOCKPIT_PANELFW_I2C_ADDR}")

include(pico_sdk_import.cmake)

project(panelfw)

pico_sdk_init()

add_executable(panelfw src/main.c)
target_link_libraries(panelfw pico_stdlib)
pico_add_extra_outputs(panelfw)
pico_enable_stdio_usb(panelfw 1)
pico_enable_stdio_uart(panelfw 1)