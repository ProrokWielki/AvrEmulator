cmake_minimum_required(VERSION 3.29)

project(nop_in_while)

add_executable(${PROJECT_NAME} main.c)
set_target_properties(${PROJECT_NAME} PROPERTIES OUTPUT_NAME "${PROJECT_NAME}.elf")

add_custom_target(hex ALL ${OBJCOPY} -O ihex -j .text -j .data "${PROJECT_NAME}.elf" "${PROJECT_NAME}.hex" DEPENDS ${PROJECT_NAME} )

add_test (NAME nop_in_while_test
  COMMAND pytest
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)
set_tests_properties(nop_in_while_test PROPERTIES DEPENDS nop_in_while)

enable_testing()