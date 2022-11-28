import sys

from pathlib import Path
from setuptools import setup
from setuptools_rust import Binding, RustExtension

readme = Path("README.md").read_text()

setup(
    name="pypassrs",
    version="0.5.5",
    description="Python wrapper for passrs",
    author="Casey Burklow",
    author_email="zevaryx@gmail.com",
    url="https://git.zevaryx.com/zevaryx/pypassrs",
    rust_extensions=[RustExtension("pypassrs.pypassrs", binding=Binding.RustCPython)],
    packages=["pypassrs"],
    # Rust extensions are not zip safe
    zip_safe=False,
    long_description=readme,
    long_description_content_type="text/markdown",
    license="MIT",
    python_requires=">=3.10",
    classifiers=[
        "Development Status :: 3 - Alpha",
        "License :: OSI Approved :: MIT License",
        "Topic :: Software Development :: Libraries",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Programming Language :: Rust",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3 :: Only",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11"
    ],
)
