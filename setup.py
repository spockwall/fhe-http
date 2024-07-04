import setuptools_rust
from setuptools import setup, find_packages

setup(
    name="fhe-http",
    version="0.1.3",
    author="spockwall",
    author_email="abcdefg35216874@gmail.com",
    description="This is an test package for Python Fhe Http ",
    long_description=open("./fhe_http_python/README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/spockwall/fhe-http",  # Replace with your project's URL
    packages=find_packages(),
    install_requires=[
        "maturin>=1.5.0",
        "ply>=3.11",
    ],
    rust_extensions=[
        setuptools_rust.RustExtension(
            "fhe_http_python.fhe_http_python",
            "fhe_http_python/Cargo.toml",
            binding=setuptools_rust.Binding.PyO3,
        )
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.10",  # Adjust this based on your project's requirements
)
