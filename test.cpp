#include <stdio.h>
#include <iostream>
#include <thread>
#include <stdlib.h>
#include <vector>
#include <sstream>

std::vector<std::string> split (const std::string &s, char delim) {
    std::vector<std::string> result;
    std::stringstream ss (s);
    std::string item;

    while (getline (ss, item, delim)) {
        result.push_back (item);
    }

    return result;
}



int main(void){
	while (true) {
		printf("habtdb> ");
		std::string input;
		std::getline(std::cin, input);
		if (input.size() == 0){ continue; }
		std::vector<std::string>  values = split(input, ' ');
		if (values[0] == "fork") {
		
		} else if (values[0] == "cat") {
		
		} else if (values[0] == "echo") {
			printf("%s\n", values[1]);
		}
	}
}

