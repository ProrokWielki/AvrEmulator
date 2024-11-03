cmake_minimum_required(VERSION 3.29)

project(timer_register_check)

add_executable(${PROJECT_NAME} main.c)
set_target_properties(${PROJECT_NAME} PROPERTIES OUTPUT_NAME "${PROJECT_NAME}.elf")

add_custom_target(hex ALL ${OBJCOPY} -O ihex -j .text -j .data "${PROJECT_NAME}.elf" "${PROJECT_NAME}.hex" DEPENDS ${PROJECT_NAME} )

add_test (NAME ${PROJECT_NAME}_test
  COMMAND pytest
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)
set_tests_properties(${PROJECT_NAME}_test PROPERTIES DEPENDS ${PROJECT_NAME})

enable_testing()