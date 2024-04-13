# Colors
RED=\033[0;31m
GREEN=\033[0;32m
MAGENTA=\033[0;35m
NC=\033[0m # No Color

RC = cargo
RP = run -- -h
BP = build --release
CP = clean
ZIP = r -- zip -s test -o testziped.zip
run:
	@printf "${MAGENTA} Running the program...${NC}\n"
	@$(RC) $(RP)
	
zip:
	@printf "${MAGENTA} Running the program...${NC}\n"
	@$(RC) $(ZIP)

build:
	@printf "${MAGENTA} Building the program...${NC}\n"
	@$(RC) $(BP)
	@sudo cp target/release/orion /usr/bin

clean:
	@printf "${MAGENTA} Cleaning the program...${NC}\n"
	@$(RC) $(CP)

create:
	@printf "${MAGENTA} Create the necessary files...${NC}\n"
	@mkdir -p test 
	@mkdir -p test/play
	@echo "# hello world" > test/hey.md
	@echo "# cool" > test/dude.md
	@echo "# cool asf" > test/play/ok.md

del:
	@printf "${MAGENTA} Delete the necessary files...${NC}\n"
	@rm -rf test testziped.zip

