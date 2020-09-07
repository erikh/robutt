gen:
	docker run --rm -u $$(id -u):$$(id -g) -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i /local/spec.yaml \
    -g rust \
    -o /local
