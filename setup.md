# py-fhe-http SetUp

### Description
- This setup script integrates py_fhe_http and assembler into a packages and upload the package to [PyPI](https://pypi.org/) and [TestPyPI](https://test.pypi.org/).
- For the uploaded package on TestPyPI, please check: [fhe-http 0.1.9](https://test.pypi.org/project/fhe-http/)
- For the uploaded package on PyPI, please check: [fhe-http 0.1.9](https://pypi.org/project/fhe-http/0.1.9/)

### How to build package and upload to PyPI
```shellscript=
$ cd fhe-http
$ python setup.py sdist bdist_wheel
$ twine check dist/*

// upload to test pypi
$ twine upload --skip-existing --repository testpypi dist/*

// upload to pypi
$ twine upload dist/*
```

### Information
- For more details, please check: [How to upload your package to the Python Package Index (PyPI) test server](https://kynan.github.io/blog/2020/05/23/how-to-upload-your-package-to-the-python-package-index-pypi-test-server)

