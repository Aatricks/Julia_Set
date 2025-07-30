@echo off
echo Building Julia Set Docker image...
docker-compose build

echo Running Julia Set generator...
docker-compose run --rm julia-set-rust --verbose

echo Generated image should be in the ./output directory
pause
