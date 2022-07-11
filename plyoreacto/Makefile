build:
	docker build -t jstubbs/plyoreacto .; cd pyobserver; docker build -t jstubbs/pyobserver .; cd ..

down:
	docker-compose down

up-demo: 
	ECHO "DEMO no longer supported.."

up-engine:
	docker-compose up
	
all: down build up