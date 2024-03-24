cmake_minimum_required(VERSION 3.22)
project(your_project_name)

# Set the C standard
set(CMAKE_C_STANDARD 99)

# Add the main.c file to the project
add_executable(Algorithms main.c)

# Add the search_algorithms directory
# add_subdirectory(search_algorithms)


# # Link the search_algorithms library with the executable
# target_link_libraries(Algorithms search_algorithms)

# custom command to run the program after compolation
add_custom_command(TARGET Algorithms
                   POST_BUILD
                   COMMAND Algorithms)