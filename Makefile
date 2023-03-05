tcp:
	 nc -v 127.0.0.1 8080

http:
	curl -v 127.0.0.1:8080

test:
	curl -v 127.0.0.1:8080/Makefile
