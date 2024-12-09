##
## EPITECH PROJECT, 2024
## My_Torch
## File description:
## Makefile
##

SRC	=
NAME1 = my_torch_generator
NAME2 = my_torch_analyzer

.PHONY: all clean fclean re tests_run

all: compile

compile:
	cd torch_generator && cargo build --release --offline && cd .. ; \
	cd torch_analyzer && cargo build --release --offline && cd .. ; \
	cp release/$(NAME1) .
	cp release/$(NAME2) .
	rm -rf debug
	rm -rf release
	rm -f .rustc_info.json

clean:

fclean: clean
	rm  -f $(NAME1) $(NAME2)

re: fclean compile

tests_run:
	cd torch_generator && cargo test && cd .. ; \
	cd torch_analyzer && cargo test && cd ..
