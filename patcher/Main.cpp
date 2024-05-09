/*
	kill me
*/

#include <iostream>
#include <fstream>
#include <vector>

void WriteBytes(std::fstream &File, std::uintptr_t Offset, std::string Bytes) {
	File.seekg(Offset, std::ios::beg);
	File.write(Bytes.c_str(), Bytes.size());
}

int main(int argc, char* argv[])  {
	if (argc == 2) {
		std::fstream File(argv[1], std::ios::in | std::ios::out | std::ios::binary);

		if (File.good()) {
			std::cout << "patching . . ." << std::endl;
			
			// 80 bf 78 01 00 00 00 74 05 e8
			// changes shit from a 'je' to a 'jne' i am so fucking funny
			// 'x74' being the 'je' and 'x75' being the 'jne'
			WriteBytes(File, 0x761030, "\x75");

			File.close();

			std::cout << "patched!" << std::endl;
		}
		
		std::cin.get();
		return 0;
	}
}