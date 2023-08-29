#!/bin/sh

listen=127.0.0.1:2951
redis=redis://127.0.0.1

ENV_ADDR="${listen}" \
	ENV_REDIS="${redis}" \
	./helo
