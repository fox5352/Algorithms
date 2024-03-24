cmake_minimum_required(VERSION 3.22)
project(your_project_name)

# Set the C standard
set(CMAKE_C_STANDARD 99)

# Add the lib directory to the include path

include_directories(./lib)

# Add the source files
set(SOURCES
    main.c
    lib/search_algorithms.c
    lib/sort_algorithms.c
    # Add other source files here if needed
)

# Add the header files
set(HEADERS
    lib/search_algorithms.h
    lib/sort_algorithms.h
    # Add other header files here if needed
)

# Add the executable target
add_executable(Algorithms ${SOURCES} ${HEADERS})

# custom command to run the program after compilation
add_custom_command(TARGET Algorithms POST_BUILD COMMAND Algorithms)