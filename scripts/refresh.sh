#!/bin/bash

# kill processes listening on ports 3000 and 3001
fuser -k 3000/tcp
fuser -k 3001/tcp
