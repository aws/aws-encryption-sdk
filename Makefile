.PHONY: clippy test examples duvet

clippy:
	cd prim && make clippy
	cd mpl && make clippy
	cd esdk && make clippy
	cd aws-esdk-cxx && make clippy

test:
	cd prim && make test
	cd mpl && make test
	cd esdk && make test
	cd aws-esdk-cxx && make test

examples:
	cd mpl && make examples
	cd esdk && make examples

duvet:
	cd prim && make duvet
	cd mpl && make duvet
	cd esdk && make duvet
	cd aws-esdk-cxx && make duvet
