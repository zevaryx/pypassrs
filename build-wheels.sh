#!/bin/bash
set -ex
curl https://sh.rustup.rs -sSf | sh -s -- -y
export PATH="$HOME/.cargo/bin:$PATH"
cd /io
for PYBIN in /opt/python/{cp310-cp130}/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"
    "${PYBIN}/pip" install -U setuptools wheel setuptools-rust
    "${PYBIN}/python" setup.py bdist_wheel
done
for whl in dist/*.whl; do
    auditwheel repair "$whl" -w dist/
done
