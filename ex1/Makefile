COM = gcc -Wall -w -pedantic -c -g

data: main.c input.c input.h data.c data.h
	gcc -o data -g -Wall -w -pedantic main.c

test:
	valgrind --fair-sched=yes --leak-check=full --show-leak-kinds=all ./data

clean:
	rm *.o

fclean:
	rm *.o
	rm data
