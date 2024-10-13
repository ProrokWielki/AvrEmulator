cmake_minimum_required(VERSION 3.29)

# Variables regarding the AVR chip
set(MCU   atmega8a)
set(F_CPU 8000000)
add_definitions(-DF_CPU=${F_CPU})

# program names
set(AVRC     avr-gcc)
set(AVRCPP   avr-g++)
set(OBJCOPY  avr-objcopy)

# Sets the compiler
# Needs to come before the project function
set(CMAKE_SYSTEM_NAME  Generic)
set(CMAKE_CXX_COMPILER ${AVRCPP})
set(CMAKE_C_COMPILER   ${AVRC})
set(CMAKE_ASM_COMPILER ${AVRC})

set(CSTANDARD "-std=gnu99")
set(CMCU      "-mmcu=${MCU}")
set(CDEFS     "-DF_CPU=${F_CPU} -DBAUD=${BAUD}")

set(CFLAGS   "${CMCU} ${CDEBUG} ${CDEFS} ${COPT} ${CWARN} ${CSTANDARD} ${CTUNING}")
set(CXXFLAGS "${CMCU} ${CDEBUG} ${CDEFS} ${COPT} ${CTUNING}")

set(CMAKE_C_FLAGS   "${CFLAGS}")
set(CMAKE_CXX_FLAGS "${CXXFLAGS}")
set(CMAKE_ASM_FLAGS   "${CFLAGS}")
