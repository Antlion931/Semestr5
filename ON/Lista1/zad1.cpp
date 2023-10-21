#include <iostream>
#include <float.h>

int main() {
    std::cout << "float epsilon " << FLT_EPSILON << std::endl;
    std::cout << "double epsilon " << DBL_EPSILON << std::endl;
    std::cout << "float max  " << FLT_MAX << std::endl;
    std::cout << "double max " << DBL_MAX << std::endl;
    return 0;
}