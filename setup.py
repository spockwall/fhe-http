import setuptools_rust
from setuptools import setup, find_packages

setup(
    name="fhe-http",
    version="0.1.9",
    author="spockwall",
    author_email="e1e1e1n9n9n9@gmail.com",
    description="This is an test package for Python Fhe Http ",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/spockwall/fhe-http",  # Replace with your project's URL
    packages=find_packages(),
    install_requires=[
        "maturin>=1.5.0",
        "ply>=3.11",
        "setuptools-rust>=1.9.0",
        "setuptools>=68.2.2",
        "wheel>=0.43.0",
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
